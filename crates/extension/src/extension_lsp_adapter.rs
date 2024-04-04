use crate::wasm_host::{
    wit::{self, LanguageServerConfig},
    WasmExtension, WasmHost,
};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Future, FutureExt};
use gpui::AsyncAppContext;
use language::{
    CodeLabel, HighlightId, Language, LanguageServerName, LspAdapter, LspAdapterDelegate,
};
use lsp::LanguageServerBinary;
use std::ops::Range;
use std::{
    any::Any,
    path::{Path, PathBuf},
    pin::Pin,
    sync::Arc,
};
use util::{maybe, ResultExt};
use wasmtime_wasi::WasiView as _;

pub struct ExtensionLspAdapter {
    pub(crate) extension: WasmExtension,
    pub(crate) language_server_id: LanguageServerName,
    pub(crate) config: LanguageServerConfig,
    pub(crate) host: Arc<WasmHost>,
}

#[async_trait(?Send)]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.config.name.clone().into())
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        _: Arc<Language>,
        _: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncAppContext,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move {
            let command = self
                .extension
                .call({
                    let this = self.clone();
                    |extension, store| {
                        async move {
                            let resource = store.data_mut().table().push(delegate)?;
                            let command = extension
                                .call_language_server_command(
                                    store,
                                    &this.language_server_id,
                                    &this.config,
                                    resource,
                                )
                                .await?
                                .map_err(|e| anyhow!("{}", e))?;
                            anyhow::Ok(command)
                        }
                        .boxed()
                    }
                })
                .await?;

            let path = self
                .host
                .path_from_extension(&self.extension.manifest.id, command.command.as_ref());

            // TODO: This should now be done via the `zed::make_file_executable` function in
            // Zed extension API, but we're leaving these existing usages in place temporarily
            // to avoid any compatibility issues between Zed and the extension versions.
            //
            // We can remove once the following extension versions no longer see any use:
            // - toml@0.0.2
            // - zig@0.0.1
            if ["toml", "zig"].contains(&self.extension.manifest.id.as_ref()) {
                #[cfg(not(windows))]
                {
                    use std::fs::{self, Permissions};
                    use std::os::unix::fs::PermissionsExt;

                    fs::set_permissions(&path, Permissions::from_mode(0o755))
                        .context("failed to set file permissions")?;
                }
            }

            Ok(LanguageServerBinary {
                path,
                arguments: command.args.into_iter().map(|arg| arg.into()).collect(),
                env: Some(command.env.into_iter().collect()),
            })
        }
        .boxed_local()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        unreachable!("get_language_server_command is overridden")
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        None
    }

    fn language_ids(&self) -> HashMap<String, String> {
        // TODO: The language IDs can be provided via the language server options
        // in `extension.toml now but we're leaving these existing usages in place temporarily
        // to avoid any compatibility issues between Zed and the extension versions.
        //
        // We can remove once the following extension versions no longer see any use:
        // - php@0.0.1
        if self.extension.manifest.id.as_ref() == "php" {
            return HashMap::from_iter([("PHP".into(), "php".into())]);
        }

        self.extension
            .manifest
            .language_servers
            .get(&LanguageServerName(self.config.name.clone().into()))
            .map(|server| server.language_ids.clone())
            .unwrap_or_default()
    }

    async fn initialization_options(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        let delegate = delegate.clone();
        let json_options = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        let resource = store.data_mut().table().push(delegate)?;
                        let options = extension
                            .call_language_server_initialization_options(
                                store,
                                &this.language_server_id,
                                &this.config,
                                resource,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))?;
                        anyhow::Ok(options)
                    }
                    .boxed()
                }
            })
            .await?;
        Ok(if let Some(json_options) = json_options {
            serde_json::from_str(&json_options).with_context(|| {
                format!("failed to parse initialization_options from extension: {json_options}")
            })?
        } else {
            None
        })
    }

    async fn labels_for_completions(
        self: Arc<Self>,
        completions: &[lsp::CompletionItem],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let completions = completions
            .into_iter()
            .map(|completion| wit::Completion::from(completion.clone()))
            .collect::<Vec<_>>();

        let labels = self
            .extension
            .call({
                let this = self.clone();
                |extension, store| {
                    async move {
                        extension
                            .call_labels_for_completions(
                                store,
                                &this.language_server_id,
                                completions,
                            )
                            .await?
                            .map_err(|e| anyhow!("{}", e))
                    }
                    .boxed()
                }
            })
            .await?;

        Ok(labels
            .into_iter()
            .map(|label| {
                label.map(|label| {
                    build_code_label(
                        &label,
                        &language.highlight_text(&label.code.as_str().into(), 0..label.code.len()),
                        &language,
                    )
                })
            })
            .collect())
    }
}

fn build_code_label(
    label: &wit::CodeLabel,
    parsed_runs: &[(Range<usize>, HighlightId)],
    language: &Arc<Language>,
) -> CodeLabel {
    let mut text = String::new();
    let mut runs = vec![];

    for span in &label.spans {
        match span {
            wit::CodeLabelSpan::CodeRange(range) => {
                let range = Range::from(*range);

                let mut input_ix = range.start;
                let mut output_ix = text.len();
                for (run_range, id) in parsed_runs {
                    if run_range.start >= range.end {
                        break;
                    }
                    if run_range.end <= input_ix {
                        continue;
                    }

                    if run_range.start > input_ix {
                        output_ix += run_range.start - input_ix;
                        input_ix = run_range.start;
                    }

                    {
                        let len = range.end.min(run_range.end) - input_ix;
                        runs.push((output_ix..output_ix + len, *id));
                        output_ix += len;
                        input_ix += len;
                    }
                }

                text.push_str(&label.code[range]);
            }
            wit::CodeLabelSpan::Literal(span) => {
                let highlight_id = language
                    .grammar()
                    .zip(span.highlight_name.as_ref())
                    .and_then(|(grammar, highlight_name)| {
                        grammar.highlight_id_for_name(&highlight_name)
                    })
                    .unwrap_or_default();
                let ix = text.len();
                runs.push((ix..ix + span.text.len(), highlight_id));
                text.push_str(&span.text);
            }
        }
    }

    CodeLabel {
        text,
        runs,
        filter_range: label.filter_range.into(),
    }
}

impl From<wit::Range> for Range<usize> {
    fn from(range: wit::Range) -> Self {
        let start = range.start as usize;
        let end = range.end as usize;
        start..end
    }
}

impl From<lsp::CompletionItem> for wit::Completion {
    fn from(value: lsp::CompletionItem) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
            kind: value.kind.map(Into::into),
            insert_text_format: value.insert_text_format.map(Into::into),
        }
    }
}

impl From<lsp::CompletionItemKind> for wit::CompletionKind {
    fn from(value: lsp::CompletionItemKind) -> Self {
        match value {
            lsp::CompletionItemKind::TEXT => Self::Text,
            lsp::CompletionItemKind::METHOD => Self::Method,
            lsp::CompletionItemKind::FUNCTION => Self::Function,
            lsp::CompletionItemKind::CONSTRUCTOR => Self::Constructor,
            lsp::CompletionItemKind::FIELD => Self::Field,
            lsp::CompletionItemKind::VARIABLE => Self::Variable,
            lsp::CompletionItemKind::CLASS => Self::Class,
            lsp::CompletionItemKind::INTERFACE => Self::Interface,
            lsp::CompletionItemKind::MODULE => Self::Module,
            lsp::CompletionItemKind::PROPERTY => Self::Property,
            lsp::CompletionItemKind::UNIT => Self::Unit,
            lsp::CompletionItemKind::VALUE => Self::Value,
            lsp::CompletionItemKind::ENUM => Self::Enum,
            lsp::CompletionItemKind::KEYWORD => Self::Keyword,
            lsp::CompletionItemKind::SNIPPET => Self::Snippet,
            lsp::CompletionItemKind::COLOR => Self::Color,
            lsp::CompletionItemKind::FILE => Self::File,
            lsp::CompletionItemKind::REFERENCE => Self::Reference,
            lsp::CompletionItemKind::FOLDER => Self::Folder,
            lsp::CompletionItemKind::ENUM_MEMBER => Self::EnumMember,
            lsp::CompletionItemKind::CONSTANT => Self::Constant,
            lsp::CompletionItemKind::STRUCT => Self::Struct,
            lsp::CompletionItemKind::EVENT => Self::Event,
            lsp::CompletionItemKind::OPERATOR => Self::Operator,
            lsp::CompletionItemKind::TYPE_PARAMETER => Self::TypeParameter,
            _ => {
                let value = maybe!({
                    let kind = serde_json::to_value(&value)?;
                    serde_json::from_value(kind)
                });

                Self::Other(value.log_err().unwrap_or(-1))
            }
        }
    }
}

impl From<lsp::InsertTextFormat> for wit::InsertTextFormat {
    fn from(value: lsp::InsertTextFormat) -> Self {
        match value {
            lsp::InsertTextFormat::PLAIN_TEXT => Self::PlainText,
            lsp::InsertTextFormat::SNIPPET => Self::Snippet,
            _ => {
                let value = maybe!({
                    let kind = serde_json::to_value(&value)?;
                    serde_json::from_value(kind)
                });

                Self::Other(value.log_err().unwrap_or(-1))
            }
        }
    }
}

#[test]
fn test_build_code_label() {
    use util::test::marked_text_ranges;

    let (code, ranges) = marked_text_ranges(
        "«const» «a»: «fn»(«Bcd»(«Efgh»)) -> «Ijklm» = pqrs.tuv",
        false,
    );
    let runs = ranges
        .iter()
        .map(|range| (range.clone(), HighlightId(0)))
        .collect::<Vec<_>>();

    let label = build_code_label(
        &wit::CodeLabel {
            spans: vec![
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find("pqrs").unwrap() as u32,
                    end: code.len() as u32,
                }),
                wit::CodeLabelSpan::CodeRange(wit::Range {
                    start: code.find(": fn").unwrap() as u32,
                    end: code.find(" = ").unwrap() as u32,
                }),
            ],
            filter_range: wit::Range {
                start: 0,
                end: "pqrs.tuv".len() as u32,
            },
            code,
        },
        &runs,
        &language::PLAIN_TEXT,
    );

    let (text, ranges) = marked_text_ranges("pqrs.tuv: «fn»(«Bcd»(«Efgh»)) -> «Ijklm»", false);
    let runs = ranges
        .iter()
        .map(|range| (range.clone(), HighlightId(0)))
        .collect::<Vec<_>>();

    assert_eq!(
        label,
        CodeLabel {
            text,
            runs,
            filter_range: label.filter_range.clone()
        }
    )
}
