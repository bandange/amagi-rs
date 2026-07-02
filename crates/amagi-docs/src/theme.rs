use material_color_utilities_rs::{
    ThemeCssVariablesOptions, theme_from_source_color, theme_to_css_variables,
};
use std::collections::BTreeMap;

const THEME_SOURCE_COLOR: u32 = 0xff4f6762;
const THEME_PALETTE_TONES: &[i32] = &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

pub(crate) fn docs_theme_css() -> String {
    let theme = theme_from_source_color(THEME_SOURCE_COLOR, &[]);
    let light = theme_to_css_variables(
        &theme,
        &ThemeCssVariablesOptions {
            dark: false,
            brightness_suffix: false,
            palette_tones: THEME_PALETTE_TONES.to_vec(),
        },
    );
    let dark = theme_to_css_variables(
        &theme,
        &ThemeCssVariablesOptions {
            dark: true,
            brightness_suffix: false,
            palette_tones: THEME_PALETTE_TONES.to_vec(),
        },
    );

    let mut css = String::from("/* Generated from material-color-utilities-rs. */\n");
    push_theme_rule(&mut css, ":root", &light);
    css.push_str("\n@media (prefers-color-scheme: dark) {\n");
    push_theme_rule(&mut css, "  :root", &dark);
    css.push_str("}\n");
    push_theme_rule(&mut css, ".theme-light", &light);
    push_theme_rule(&mut css, ".theme-dark", &dark);
    css
}

fn push_theme_rule(css: &mut String, selector: &str, variables: &BTreeMap<String, String>) {
    css.push_str(selector);
    css.push_str(" {\n");
    css.push_str("  --amagi-theme-source: #4f6762;\n");
    for (name, value) in variables {
        css.push_str("  ");
        css.push_str(name);
        css.push_str(": ");
        css.push_str(value);
        css.push_str(";\n");
    }
    css.push_str("}\n");
}
