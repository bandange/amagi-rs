use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::OsString,
    io::IsTerminal,
    str::FromStr,
    sync::OnceLock,
};

use clap::{Arg, ArgAction, ColorChoice, Command};
use serde::Deserialize;

use crate::env::DotenvMap;

const DEFAULT_LOCALE_CODE: &str = "en-US";
const DEFAULT_CLI_COLOR: ColorChoice = ColorChoice::Auto;
const EMBEDDED_CATALOGS: &[&str] = &[
    include_str!("../../locales/cli/zh-CN.json"),
    include_str!("../../locales/cli/en-US.json"),
];

const BILIBILI_COMMANDS: &[&str] = &[
    "video-info",
    "video-stream",
    "video-danmaku",
    "comments",
    "comment-replies",
    "user-card",
    "user-dynamic-list",
    "user-space-info",
    "uploader-total-views",
    "dynamic-detail",
    "dynamic-card",
    "bangumi-info",
    "bangumi-stream",
    "live-room-info",
    "live-room-init",
    "login-status",
    "login-qrcode",
    "qrcode-status",
    "emoji-list",
    "av-to-bv",
    "bv-to-av",
    "article-content",
    "article-cards",
    "article-info",
    "article-list-info",
    "captcha-from-voucher",
    "validate-captcha",
];

const DOUYIN_COMMANDS: &[&str] = &[
    "parse-work",
    "video-work",
    "image-album-work",
    "slides-work",
    "text-work",
    "work-comments",
    "comment-replies",
    "user-profile",
    "user-video-list",
    "user-favorite-list",
    "user-recommend-list",
    "search",
    "suggest-words",
    "music-info",
    "live-room-info",
    "login-qrcode",
    "emoji-list",
    "dynamic-emoji-list",
    "danmaku-list",
];

const KUAISHOU_COMMANDS: &[&str] = &[
    "video-work",
    "work-comments",
    "emoji-list",
    "user-profile",
    "user-work-list",
    "live-room-info",
];

const XIAOHONGSHU_COMMANDS: &[&str] = &[
    "home-feed",
    "note-detail",
    "note-comments",
    "user-profile",
    "user-note-list",
    "emoji-list",
    "search",
];

const TWITTER_COMMANDS: &[&str] = &[
    "search-tweets",
    "user-profile",
    "user-timeline",
    "user-replies",
    "user-media",
    "user-followers",
    "user-following",
    "user-likes",
    "user-bookmarks",
    "user-followed",
    "user-recommended",
    "search-users",
    "tweet-detail",
    "tweet-replies",
    "tweet-likers",
    "tweet-retweeters",
    "space-detail",
];

/// Supported CLI help language resolved from built-in locale catalogs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CliLanguage {
    code: &'static str,
}

impl CliLanguage {
    fn parse_raw(value: &str) -> Option<Self> {
        let normalized = normalize_language_tag(value);
        if normalized.is_empty() {
            return None;
        }

        find_catalog(&normalized).map(|catalog| Self {
            code: catalog.meta.code.as_str(),
        })
    }
}

impl FromStr for CliLanguage {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse_raw(value).ok_or_else(|| {
            format!(
                "supported values: {}",
                supported_language_values().join(", ")
            )
        })
    }
}

pub(crate) fn resolve_cli_language(args: &[OsString], dotenv: Option<&DotenvMap>) -> CliLanguage {
    detect_arg_language(args)
        .or_else(|| {
            std::env::var("AMAGI_LANG")
                .ok()
                .as_deref()
                .and_then(CliLanguage::parse_raw)
        })
        .or_else(|| {
            dotenv
                .and_then(|values| values.get("AMAGI_LANG"))
                .and_then(|value| CliLanguage::parse_raw(value))
        })
        .or_else(detect_locale_language)
        .or_else(detect_windows_ui_language)
        .unwrap_or_else(default_language)
}

pub(crate) fn localize_command(command: Command, lang: CliLanguage) -> Command {
    localize_root(command, catalog_for(lang))
}

pub(crate) fn render_clap_error(error: &clap::Error, lang: CliLanguage) -> String {
    let rendered = if should_render_ansi(error.use_stderr()) {
        error.render().ansi().to_string()
    } else {
        error.to_string()
    };
    localize_clap_error_message(&rendered, lang)
}

pub(crate) fn localize_clap_error_message(message: &str, lang: CliLanguage) -> String {
    match lang.code {
        "zh-CN" => {
            let mut localized = message.to_owned();
            for (from, to) in [
                (
                    "For more information, try '--help'.",
                    "如需更多信息，请使用 '--help'。",
                ),
                (
                    "the following required arguments were not provided:",
                    "缺少以下必需参数：",
                ),
                ("a value is required for", "以下参数需要一个值："),
                ("but none was supplied", "但未提供值"),
                ("unexpected argument '", "发现意外参数 '"),
                ("' found", "'"),
                (
                    "' requires a subcommand but one was not provided",
                    "' 需要一个子命令，但未提供",
                ),
                ("unrecognized subcommand", "无法识别的子命令"),
                ("unexpected argument", "发现意外参数"),
                ("invalid value", "无效的取值"),
                (" for '", "，参数 '"),
                (" as a value, use ", " 作为值传入，请使用 "),
                ("possible values:", "可选值："),
                ("tip: to pass", "提示：若要将"),
                ("Usage:", "用法:"),
                ("error:", "错误:"),
            ] {
                localized = localized.replace(from, to);
            }
            localize_help_hint_zh(localized)
        }
        _ => message.to_owned(),
    }
}

fn localize_help_hint_zh(message: String) -> String {
    let exact = "For more information, try '--help'.";
    if message.contains(exact) {
        return message.replace(exact, "如需更多信息，请使用 '--help'。");
    }

    let prefix = "For more information, try '";
    let Some(start) = message.find(prefix) else {
        return message;
    };

    let mut localized = String::with_capacity(message.len() + 16);
    localized.push_str(&message[..start]);
    localized.push_str("如需更多信息，请使用 '");

    let rest = &message[start + prefix.len()..];
    if let Some(end) = rest.find("'.") {
        localized.push_str(&rest[..end]);
        localized.push_str("'。");
        localized.push_str(&rest[end + 2..]);
    } else {
        localized.push_str(rest);
    }

    localized
}

fn detect_arg_language(args: &[OsString]) -> Option<CliLanguage> {
    let mut index = 1usize;
    while index < args.len() {
        let arg = args[index].to_string_lossy();

        if arg == "--lang" {
            return args
                .get(index + 1)
                .and_then(|value| CliLanguage::parse_raw(&value.to_string_lossy()));
        }

        if let Some(value) = arg.strip_prefix("--lang=") {
            return CliLanguage::parse_raw(value);
        }

        index += 1;
    }

    None
}

pub(crate) fn default_cli_color() -> ColorChoice {
    DEFAULT_CLI_COLOR
}

fn should_render_ansi(use_stderr: bool) -> bool {
    match DEFAULT_CLI_COLOR {
        ColorChoice::Always => true,
        ColorChoice::Auto => {
            if use_stderr {
                std::io::stderr().is_terminal()
            } else {
                std::io::stdout().is_terminal()
            }
        }
        ColorChoice::Never => false,
    }
}

fn detect_locale_language() -> Option<CliLanguage> {
    ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"]
        .into_iter()
        .find_map(|name| std::env::var(name).ok())
        .as_deref()
        .and_then(CliLanguage::parse_raw)
}

#[cfg(windows)]
fn detect_windows_ui_language() -> Option<CliLanguage> {
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetUserDefaultUILanguage() -> u16;
    }

    let lang_id = unsafe { GetUserDefaultUILanguage() };
    let primary = lang_id & 0x03ff;

    match primary {
        0x0004 => CliLanguage::parse_raw("zh"),
        0x0009 => CliLanguage::parse_raw("en"),
        _ => None,
    }
}

#[cfg(not(windows))]
fn detect_windows_ui_language() -> Option<CliLanguage> {
    None
}

fn localize_root(command: Command, catalog: &'static LocaleCatalog) -> Command {
    let mut command = command;

    command = command.mut_subcommand("run", |sub| localize_run(sub, catalog));

    #[cfg(feature = "server")]
    {
        command = command.mut_subcommand("serve", |sub| localize_serve(sub, catalog));
    }

    standard_command(
        command,
        catalog,
        "root",
        catalog.headings.global_options.clone(),
    )
    .version(leak_str(&version_output(catalog)))
    .long_version(leak_str(&version_output(catalog)))
    .after_help(catalog.help.root_after_help.clone())
}

fn localize_run(command: Command, catalog: &'static LocaleCatalog) -> Command {
    let command = command
        .mut_subcommand("bilibili", |sub| {
            localize_leaf_group(sub, catalog, "run.bilibili", BILIBILI_COMMANDS)
        })
        .mut_subcommand("douyin", |sub| {
            localize_leaf_group(sub, catalog, "run.douyin", DOUYIN_COMMANDS)
        })
        .mut_subcommand("kuaishou", |sub| {
            localize_leaf_group(sub, catalog, "run.kuaishou", KUAISHOU_COMMANDS)
        })
        .mut_subcommand("twitter", |sub| {
            localize_leaf_group(sub, catalog, "run.twitter", TWITTER_COMMANDS)
        })
        .mut_subcommand("xiaohongshu", |sub| {
            localize_leaf_group(sub, catalog, "run.xiaohongshu", XIAOHONGSHU_COMMANDS)
        });

    standard_command(command, catalog, "run", catalog.headings.options.clone())
}

#[cfg(feature = "server")]
fn localize_serve(command: Command, catalog: &'static LocaleCatalog) -> Command {
    standard_command(command, catalog, "serve", catalog.headings.options.clone())
}

fn localize_leaf_group(
    mut command: Command,
    catalog: &'static LocaleCatalog,
    path: &str,
    children: &[&str],
) -> Command {
    for child in children {
        let key = format!("{path}.{child}");
        command =
            command.mut_subcommand(child, move |sub| leaf_command(sub, catalog, key.as_str()));
    }

    standard_command(command, catalog, path, catalog.headings.options.clone())
}

fn leaf_command(command: Command, catalog: &'static LocaleCatalog, key: &str) -> Command {
    standard_command(command, catalog, key, catalog.headings.options.clone())
}

fn standard_command(
    command: Command,
    catalog: &'static LocaleCatalog,
    key: &str,
    options_heading_text: String,
) -> Command {
    let mut command = command
        .about(catalog.command_about(key).to_owned())
        .long_about(None)
        .help_template(help_template(catalog))
        .subcommand_help_heading(leak_str(&catalog.headings.commands))
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .arg(custom_help_arg(catalog, &options_heading_text));

    if key == "root" {
        command = command
            .disable_version_flag(true)
            .arg(custom_version_arg(catalog, &options_heading_text));
    }

    localize_known_args(command, catalog, key, options_heading_text)
}

fn localize_known_args(
    mut command: Command,
    catalog: &'static LocaleCatalog,
    command_key: &str,
    options_heading_text: String,
) -> Command {
    for id in catalog.arg_ids() {
        let Some(is_positional) = arg_is_positional(&command, id) else {
            continue;
        };

        let heading = if is_positional {
            catalog.headings.arguments.clone()
        } else {
            options_heading_text.clone()
        };
        let help = catalog.arg_help(command_key, id).to_owned();
        let hide_possible_values = catalog.has_command_arg_override(command_key, id)
            && arg_has_possible_values(&command, id);

        command = command.mut_arg(id, move |arg| {
            let arg = arg
                .help(help.clone())
                .help_heading(Some(leak_str(&heading)));
            let arg = if arg.get_env().is_some() {
                arg.hide_env(true)
            } else {
                arg
            };

            if hide_possible_values {
                arg.hide_possible_values(true)
            } else {
                arg
            }
        });
    }

    command
}

fn arg_is_positional(command: &Command, id: &str) -> Option<bool> {
    command
        .get_arguments()
        .find(|arg| arg.get_id().as_str() == id)
        .map(|arg| arg.get_index().is_some())
}

fn arg_has_possible_values(command: &Command, id: &str) -> bool {
    command
        .get_arguments()
        .find(|arg| arg.get_id().as_str() == id)
        .map(|arg| !arg.get_possible_values().is_empty())
        .unwrap_or(false)
}

fn help_template(catalog: &LocaleCatalog) -> String {
    format!(
        "{{before-help}}{{name}} {}\n{{about-with-newline}}{}: {{usage}}\n\n{{all-args}}{{after-help}}",
        env!("CARGO_PKG_VERSION"),
        catalog.headings.usage,
    )
}

fn version_output(catalog: &LocaleCatalog) -> String {
    let labels = version_labels(catalog);

    format!(
        "{}\n{}: {}\n{}: {}\n{}: {}",
        env!("CARGO_PKG_VERSION"),
        labels.build_time,
        build_time(),
        labels.toolchain,
        build_toolchain(),
        labels.target,
        build_target(),
    )
}

fn version_labels(catalog: &LocaleCatalog) -> VersionLabels {
    match catalog.meta.code.as_str() {
        "zh-CN" => VersionLabels {
            build_time: "构建时间",
            toolchain: "构建工具链",
            target: "目标平台",
        },
        _ => VersionLabels {
            build_time: "Build Time",
            toolchain: "Build Toolchain",
            target: "Target",
        },
    }
}

fn build_time() -> &'static str {
    option_env!("AMAGI_BUILD_TIME").unwrap_or("unknown")
}

fn build_toolchain() -> &'static str {
    option_env!("AMAGI_BUILD_TOOLCHAIN").unwrap_or("unknown")
}

fn build_target() -> &'static str {
    option_env!("AMAGI_BUILD_TARGET").unwrap_or("unknown")
}

fn custom_help_arg(catalog: &LocaleCatalog, heading: &str) -> Arg {
    let help = leak_str(catalog.arg_help("root", "help"));
    Arg::new("help")
        .short('h')
        .long("help")
        .action(ArgAction::Help)
        .help(help)
        .help_heading(Some(leak_str(heading)))
}

fn custom_version_arg(catalog: &LocaleCatalog, heading: &str) -> Arg {
    let help = leak_str(catalog.arg_help("root", "version"));
    Arg::new("version")
        .short('V')
        .long("version")
        .action(ArgAction::Version)
        .help(help)
        .help_heading(Some(leak_str(heading)))
}

fn default_language() -> CliLanguage {
    CliLanguage {
        code: catalog_by_code(DEFAULT_LOCALE_CODE)
            .expect("default CLI locale must exist")
            .meta
            .code
            .as_str(),
    }
}

fn supported_language_values() -> Vec<String> {
    catalogs()
        .iter()
        .map(|catalog| catalog.preferred_selector().to_owned())
        .collect()
}

fn find_catalog(normalized: &str) -> Option<&'static LocaleCatalog> {
    catalog_by_normalized_tag(normalized).or_else(|| {
        normalized
            .split('-')
            .next()
            .filter(|primary| !primary.is_empty() && *primary != normalized)
            .and_then(catalog_by_normalized_tag)
    })
}

fn catalog_for(lang: CliLanguage) -> &'static LocaleCatalog {
    catalog_by_code(lang.code)
        .or_else(|| catalog_by_code(DEFAULT_LOCALE_CODE))
        .expect("at least one CLI locale catalog must exist")
}

fn catalog_by_code(code: &str) -> Option<&'static LocaleCatalog> {
    catalogs().iter().find(|catalog| catalog.meta.code == code)
}

fn catalog_by_normalized_tag(normalized: &str) -> Option<&'static LocaleCatalog> {
    catalogs()
        .iter()
        .find(|catalog| catalog.matches(normalized))
}

fn catalogs() -> &'static [LocaleCatalog] {
    static CATALOGS: OnceLock<Vec<LocaleCatalog>> = OnceLock::new();
    CATALOGS.get_or_init(load_catalogs)
}

fn load_catalogs() -> Vec<LocaleCatalog> {
    let catalogs = EMBEDDED_CATALOGS
        .iter()
        .map(|raw| {
            serde_json::from_str::<LocaleCatalog>(raw).expect("embedded CLI locale JSON must parse")
        })
        .collect::<Vec<_>>();

    validate_catalogs(&catalogs);
    catalogs
}

fn validate_catalogs(catalogs: &[LocaleCatalog]) {
    assert!(
        !catalogs.is_empty(),
        "at least one CLI locale catalog is required"
    );

    let mut seen_codes = BTreeSet::new();
    for catalog in catalogs {
        assert!(
            seen_codes.insert(catalog.meta.code.clone()),
            "duplicate CLI locale code: {}",
            catalog.meta.code
        );
        assert!(
            !catalog.meta.aliases.is_empty(),
            "CLI locale {} must declare at least one alias",
            catalog.meta.code
        );
    }

    assert!(
        catalogs
            .iter()
            .any(|catalog| catalog.meta.code == DEFAULT_LOCALE_CODE),
        "default CLI locale {DEFAULT_LOCALE_CODE} must exist"
    );

    let expected_commands = catalogs[0]
        .commands
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_args = catalogs[0].args.keys().cloned().collect::<BTreeSet<_>>();
    let expected_command_args = catalogs[0]
        .command_args
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();

    for catalog in catalogs.iter().skip(1) {
        let command_keys = catalog.commands.keys().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            command_keys, expected_commands,
            "CLI locale {} has mismatched command translation keys",
            catalog.meta.code
        );

        let arg_keys = catalog.args.keys().cloned().collect::<BTreeSet<_>>();
        assert_eq!(
            arg_keys, expected_args,
            "CLI locale {} has mismatched argument translation keys",
            catalog.meta.code
        );

        let command_arg_keys = catalog
            .command_args
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        assert_eq!(
            command_arg_keys, expected_command_args,
            "CLI locale {} has mismatched command argument translation keys",
            catalog.meta.code
        );
    }
}

fn normalize_language_tag(value: &str) -> String {
    value
        .trim()
        .split('.')
        .next()
        .unwrap_or_default()
        .replace('_', "-")
        .to_ascii_lowercase()
}

fn leak_str(value: &str) -> &'static str {
    Box::leak(value.to_owned().into_boxed_str())
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocaleCatalog {
    meta: LocaleMeta,
    headings: LocaleHeadings,
    help: LocaleHelp,
    commands: BTreeMap<String, CommandText>,
    args: BTreeMap<String, String>,
    #[serde(default)]
    command_args: BTreeMap<String, String>,
}

impl LocaleCatalog {
    fn matches(&self, normalized: &str) -> bool {
        normalize_language_tag(&self.meta.code) == normalized
            || self
                .meta
                .aliases
                .iter()
                .any(|alias| normalize_language_tag(alias) == normalized)
    }

    fn preferred_selector(&self) -> &str {
        self.meta
            .aliases
            .first()
            .map(String::as_str)
            .unwrap_or_else(|| self.meta.code.as_str())
    }

    fn command_about(&self, key: &str) -> &str {
        self.commands
            .get(key)
            .unwrap_or_else(|| {
                panic!(
                    "missing CLI command translation key `{key}` for locale {}",
                    self.meta.code
                )
            })
            .about
            .as_str()
    }

    fn arg_ids(&self) -> impl Iterator<Item = &str> {
        self.args.keys().map(String::as_str)
    }

    fn arg_help(&self, command_key: &str, arg_id: &str) -> &str {
        self.command_args
            .get(&format!("{command_key}.{arg_id}"))
            .or_else(|| self.args.get(arg_id))
            .unwrap_or_else(|| {
                panic!(
                    "missing CLI argument translation key `{arg_id}` for locale {}",
                    self.meta.code
                )
            })
            .as_str()
    }

    fn has_command_arg_override(&self, command_key: &str, arg_id: &str) -> bool {
        self.command_args
            .contains_key(&format!("{command_key}.{arg_id}"))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocaleMeta {
    code: String,
    aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocaleHeadings {
    usage: String,
    global_options: String,
    options: String,
    arguments: String,
    commands: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LocaleHelp {
    root_after_help: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CommandText {
    about: String,
}

struct VersionLabels {
    build_time: &'static str,
    toolchain: &'static str,
    target: &'static str,
}
