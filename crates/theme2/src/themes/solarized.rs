// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn solarized() -> UserThemeFamily {
    UserThemeFamily {
        name: "Solarized".into(),
        author: "Zed Industries".into(),
        themes: vec![
            UserTheme {
                name: "Solarized Light".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x9faaa8ff).into()),
                        border_variant: Some(rgba(0x9faaa8ff).into()),
                        elevated_surface_background: Some(rgba(0xcfd0c4ff).into()),
                        background: Some(rgba(0xcfd0c4ff).into()),
                        panel_background: Some(rgba(0xf3eddaff).into()),
                        element_hover: Some(rgba(0x9faaa880).into()),
                        element_selected: Some(rgba(0x7f919480).into()),
                        text: Some(rgba(0x34555eff).into()),
                        text_muted: Some(rgba(0x788b8fff).into()),
                        text_placeholder: Some(rgba(0x788b8fff).into()),
                        text_disabled: Some(rgba(0x002b36ff).into()),
                        text_accent: Some(rgba(0x298bd1ff).into()),
                        status_bar_background: Some(rgba(0xcfd0c4ff).into()),
                        title_bar_background: Some(rgba(0xcfd0c4ff).into()),
                        toolbar_background: Some(rgba(0xfdf6e3ff).into()),
                        tab_bar_background: Some(rgba(0xf3eddaff).into()),
                        tab_inactive_background: Some(rgba(0xf3eddaff).into()),
                        tab_active_background: Some(rgba(0xfdf6e3ff).into()),
                        scrollbar_thumb_background: Some(rgba(0x002b364d).into()),
                        scrollbar_thumb_hover_background: Some(rgba(0x002b364d).into()),
                        scrollbar_thumb_border: Some(rgba(0xf5eedbff).into()),
                        scrollbar_track_border: Some(rgba(0xf5eedbff).into()),
                        editor_foreground: Some(rgba(0x002b36ff).into()),
                        editor_background: Some(rgba(0xfdf6e3ff).into()),
                        editor_gutter_background: Some(rgba(0xfdf6e3ff).into()),
                        editor_line_number: Some(rgba(0x002b3659).into()),
                        editor_active_line_number: Some(rgba(0x002b36ff).into()),
                        editor_wrap_guide: Some(rgba(0x002b360d).into()),
                        editor_active_wrap_guide: Some(rgba(0x002b361a).into()),
                        terminal_background: Some(rgba(0xfdf6e3ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x7b8e91ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xfaa091ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0xc6cb8bff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xe1c28aff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0xa5c3e9ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xf0a2bfff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x9fd0cbff).into()),
                        terminal_ansi_bright_white: Some(rgba(0x002b36ff).into()),
                        terminal_ansi_black: Some(rgba(0xfdf6e3ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc3330ff).into()),
                        terminal_ansi_green: Some(rgba(0x859904ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58904ff).into()),
                        terminal_ansi_blue: Some(rgba(0x298bd1ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33882ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x2ca198ff).into()),
                        terminal_ansi_white: Some(rgba(0x002b36ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        created: Some(rgba(0xa0ad46ff).into()),
                        deleted: Some(rgba(0xdc3330ff).into()),
                        error: Some(rgba(0xdc3330ff).into()),
                        modified: Some(rgba(0xb58904ff).into()),
                        success: Some(rgba(0x002b36ff).into()),
                        warning: Some(rgba(0xb58904ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x30525bff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x30525bff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "embedded".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x002b36ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "enum".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xb58904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "hint".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x5889a3ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "label".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "link_text".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "link_uri".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "predictive".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x679aafff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "preproc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x002b36ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "primary".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x002b36ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "property".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x05333eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x05333eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x05333eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x05333eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x05333eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x30525bff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4c18ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "title".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x002b36ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x2ca198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x002b36ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x298bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Solarized Dark".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x2b4f58ff).into()),
                        border_variant: Some(rgba(0x2b4f58ff).into()),
                        elevated_surface_background: Some(rgba(0x083743ff).into()),
                        background: Some(rgba(0x083743ff).into()),
                        panel_background: Some(rgba(0x04313cff).into()),
                        element_hover: Some(rgba(0x2b4f5880).into()),
                        element_selected: Some(rgba(0x566d7480).into()),
                        text: Some(rgba(0x93a1a1ff).into()),
                        text_muted: Some(rgba(0x5f757dff).into()),
                        text_placeholder: Some(rgba(0x5f757dff).into()),
                        text_disabled: Some(rgba(0xfdf6e3ff).into()),
                        text_accent: Some(rgba(0x288bd1ff).into()),
                        status_bar_background: Some(rgba(0x083743ff).into()),
                        title_bar_background: Some(rgba(0x083743ff).into()),
                        toolbar_background: Some(rgba(0x002b36ff).into()),
                        tab_bar_background: Some(rgba(0x04313cff).into()),
                        tab_inactive_background: Some(rgba(0x04313cff).into()),
                        tab_active_background: Some(rgba(0x002b36ff).into()),
                        scrollbar_thumb_background: Some(rgba(0xfdf6e34d).into()),
                        scrollbar_thumb_hover_background: Some(rgba(0xfdf6e34d).into()),
                        scrollbar_thumb_border: Some(rgba(0x032f3bff).into()),
                        scrollbar_track_border: Some(rgba(0x032f3bff).into()),
                        editor_foreground: Some(rgba(0xfdf6e3ff).into()),
                        editor_background: Some(rgba(0x002b36ff).into()),
                        editor_gutter_background: Some(rgba(0x002b36ff).into()),
                        editor_line_number: Some(rgba(0xfdf6e359).into()),
                        editor_active_line_number: Some(rgba(0xfdf6e3ff).into()),
                        editor_wrap_guide: Some(rgba(0xfdf6e30d).into()),
                        editor_active_wrap_guide: Some(rgba(0xfdf6e31a).into()),
                        terminal_background: Some(rgba(0x002b36ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x5c7279ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0x7d181cff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x434a11ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0x5d4310ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x214465ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x6f1f40ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x204e4aff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xfdf6e3ff).into()),
                        terminal_ansi_black: Some(rgba(0x002b36ff).into()),
                        terminal_ansi_red: Some(rgba(0xdc3330ff).into()),
                        terminal_ansi_green: Some(rgba(0x859904ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xb58903ff).into()),
                        terminal_ansi_blue: Some(rgba(0x288bd1ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xd33782ff).into()),
                        terminal_ansi_cyan: Some(rgba(0x2ca198ff).into()),
                        terminal_ansi_white: Some(rgba(0xfdf6e3ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        created: Some(rgba(0x859904ff).into()),
                        deleted: Some(rgba(0xb52727ff).into()),
                        error: Some(rgba(0xdc3330ff).into()),
                        modified: Some(rgba(0xb58903ff).into()),
                        success: Some(rgba(0xfdf6e3ff).into()),
                        warning: Some(rgba(0xb58903ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x99a5a4ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x99a5a4ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "embedded".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xfdf6e3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "emphasis.strong".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "enum".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xb58903ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "hint".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x4f8297ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "label".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "link_text".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "link_uri".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x859904ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "operator".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "predictive".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x40728bff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "preproc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xfdf6e3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "primary".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xfdf6e3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "property".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xefe9d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xefe9d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xefe9d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xefe9d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xefe9d6ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x99a5a4ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xcb4b17ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "title".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xfdf6e3ff).into()),
                                    font_weight: Some(UserFontWeight(700.0)),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x2ca198ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xfdf6e3ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x288bd1ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
        ],
    }
}
