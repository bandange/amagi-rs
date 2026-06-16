use dioxus::prelude::*;
use material_color_utilities_rs::{
    ThemeCssVariablesOptions, theme_from_source_color, theme_to_css_variables,
};
use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, html};
use std::collections::BTreeMap;

const THEME_SOURCE_COLOR: u32 = 0xff4f6762;
const THEME_PALETTE_TONES: &[i32] = &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];
const THEME_STORAGE_KEY: &str = "amagi-docs-theme";
const LANGUAGES: &[Language] = &[Language::Chinese, Language::English];
const PROJECT_REPOSITORY_URL: &str = "https://github.com/bandange/amagi-rs";
const PROJECT_LICENSE_URL: &str = "https://github.com/bandange/amagi-rs/blob/main/LICENSE";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Language {
    English,
    Chinese,
}

impl Language {
    fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    fn docs(self) -> &'static [DocPage] {
        match self {
            Language::English => ENGLISH_DOCS,
            Language::Chinese => CHINESE_DOCS,
        }
    }

    fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" | "en-US" => Some(Language::English),
            "zh" | "zh-CN" | "cn" => Some(Language::Chinese),
            _ => None,
        }
    }

    fn search_label(self) -> &'static str {
        match self {
            Language::English => "Search",
            Language::Chinese => "搜索",
        }
    }

    fn search_placeholder(self) -> &'static str {
        match self {
            Language::English => "Search content",
            Language::Chinese => "搜索全文",
        }
    }

    fn empty_search_label(self) -> &'static str {
        match self {
            Language::English => "No matching content.",
            Language::Chinese => "没有匹配内容。",
        }
    }

    fn docs_label(self) -> &'static str {
        match self {
            Language::English => "Docs",
            Language::Chinese => "文档",
        }
    }

    fn language_control_label(self) -> &'static str {
        match self {
            Language::English => "Language",
            Language::Chinese => "语言",
        }
    }

    fn source_label(self) -> &'static str {
        match self {
            Language::English => "Source",
            Language::Chinese => "源文件",
        }
    }

    fn search_results_label(self) -> &'static str {
        match self {
            Language::English => "Search results",
            Language::Chinese => "搜索结果",
        }
    }

    fn sidebar_kicker(self) -> &'static str {
        match self {
            Language::English => "Workspace",
            Language::Chinese => "工作区",
        }
    }

    fn sidebar_title(self) -> &'static str {
        match self {
            Language::English => "Amagi-rs Documentation",
            Language::Chinese => "Amagi-rs 文档",
        }
    }

    fn sidebar_description(self) -> &'static str {
        match self {
            Language::English => "Rust API, CLI, service API, and platform references.",
            Language::Chinese => "Rust API、命令行、服务接口和平台参考。",
        }
    }

    fn footer_note(self) -> &'static str {
        match self {
            Language::English => "Non-official project. Use responsibly.",
            Language::Chinese => "非官方项目，请自行确保合规使用。",
        }
    }

    fn version_label(self) -> &'static str {
        match self {
            Language::English => "Version",
            Language::Chinese => "版本",
        }
    }

    fn disclaimer_label(self) -> &'static str {
        match self {
            Language::English => "Disclaimer",
            Language::Chinese => "免责声明",
        }
    }

    fn license_label(self) -> &'static str {
        match self {
            Language::English => "License",
            Language::Chinese => "许可证",
        }
    }

    fn repository_label(self) -> &'static str {
        match self {
            Language::English => "Repository",
            Language::Chinese => "项目地址",
        }
    }

    fn read_before_use_label(self) -> &'static str {
        match self {
            Language::English => "Read before use",
            Language::Chinese => "使用前请阅读",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DocPage {
    slug: &'static str,
    title: &'static str,
    description: &'static str,
    section: &'static str,
    path: &'static str,
    markdown: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
struct SearchResult {
    page: &'static DocPage,
    snippet: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ThemePreference {
    System,
    Light,
    Dark,
}

impl ThemePreference {
    fn label(self) -> &'static str {
        match self {
            ThemePreference::System => "System",
            ThemePreference::Light => "Light",
            ThemePreference::Dark => "Dark",
        }
    }

    fn storage_value(self) -> &'static str {
        match self {
            ThemePreference::System => "system",
            ThemePreference::Light => "light",
            ThemePreference::Dark => "dark",
        }
    }

    fn root_class(self) -> &'static str {
        match self {
            ThemePreference::System => "theme-root theme-system",
            ThemePreference::Light => "theme-root theme-light",
            ThemePreference::Dark => "theme-root theme-dark",
        }
    }

    fn from_storage(value: &str) -> Option<Self> {
        match value {
            "system" | "auto" => Some(ThemePreference::System),
            "light" => Some(ThemePreference::Light),
            "dark" => Some(ThemePreference::Dark),
            _ => None,
        }
    }

    fn next(self) -> Self {
        match self {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        }
    }

    fn icon_class(self) -> &'static str {
        match self {
            ThemePreference::System => "theme-icon theme-icon-system",
            ThemePreference::Light => "theme-icon theme-icon-light",
            ThemePreference::Dark => "theme-icon theme-icon-dark",
        }
    }
}

const ENGLISH_DOCS: &[DocPage] = &[
    DocPage {
        slug: "home",
        title: "Overview",
        description: "Project structure, capabilities, and documentation map.",
        section: "Start",
        path: "README.md",
        markdown: include_str!("../../../README.md"),
    },
    DocPage {
        slug: "installation",
        title: "Installation",
        description: "Install the CLI and configure runtime requirements.",
        section: "Start",
        path: "docs/en/installation.md",
        markdown: include_str!("../../../docs/en/installation.md"),
    },
    DocPage {
        slug: "cli",
        title: "CLI Reference",
        description: "Command groups, flags, and usage examples.",
        section: "Reference",
        path: "docs/en/reference/cli.md",
        markdown: include_str!("../../../docs/en/reference/cli.md"),
    },
    DocPage {
        slug: "rust-api",
        title: "Rust API Reference",
        description: "Rust API modules and integration notes.",
        section: "Reference",
        path: "docs/en/reference/rust-api.md",
        markdown: include_str!("../../../docs/en/reference/rust-api.md"),
    },
    DocPage {
        slug: "web-api",
        title: "Service API Reference",
        description: "HTTP routes exposed by the service runtime.",
        section: "Reference",
        path: "docs/en/reference/web-api.md",
        markdown: include_str!("../../../docs/en/reference/web-api.md"),
    },
    DocPage {
        slug: "api-catalog",
        title: "API Spec Reference",
        description: "Catalog coverage and platform API contracts.",
        section: "Reference",
        path: "docs/en/reference/api-catalog.md",
        markdown: include_str!("../../../docs/en/reference/api-catalog.md"),
    },
    DocPage {
        slug: "testing",
        title: "Testing Layout",
        description: "Workspace test crates, fixtures, and verification flow.",
        section: "Operations",
        path: "docs/en/testing.md",
        markdown: include_str!("../../../docs/en/testing.md"),
    },
    DocPage {
        slug: "disclaimer",
        title: "Disclaimer",
        description: "Warranty, affiliation, compliance, and third-party rights notice.",
        section: "Legal",
        path: "DISCLAIMER.md",
        markdown: include_str!("../../../DISCLAIMER.md"),
    },
];

const CHINESE_DOCS: &[DocPage] = &[
    DocPage {
        slug: "home",
        title: "概览",
        description: "项目结构、能力范围和文档入口。",
        section: "开始",
        path: "README.zh-CN.md",
        markdown: include_str!("../../../README.zh-CN.md"),
    },
    DocPage {
        slug: "installation",
        title: "安装指南",
        description: "安装命令行工具并配置运行环境。",
        section: "开始",
        path: "docs/中文/安装指南.md",
        markdown: include_str!("../../../docs/中文/安装指南.md"),
    },
    DocPage {
        slug: "cli",
        title: "命令行参考",
        description: "命令分组、参数和调用示例。",
        section: "参考",
        path: "docs/中文/参考/命令行参考.md",
        markdown: include_str!("../../../docs/中文/参考/命令行参考.md"),
    },
    DocPage {
        slug: "rust-api",
        title: "Rust API 参考",
        description: "Rust API 模块和集成说明。",
        section: "参考",
        path: "docs/中文/参考/Rust API 参考.md",
        markdown: include_str!("../../../docs/中文/参考/Rust API 参考.md"),
    },
    DocPage {
        slug: "web-api",
        title: "服务接口参考",
        description: "服务运行时暴露的 HTTP 路由。",
        section: "参考",
        path: "docs/中文/参考/服务接口参考.md",
        markdown: include_str!("../../../docs/中文/参考/服务接口参考.md"),
    },
    DocPage {
        slug: "api-catalog",
        title: "接口规格参考",
        description: "接口目录覆盖范围和平台契约。",
        section: "参考",
        path: "docs/中文/参考/接口规格参考.md",
        markdown: include_str!("../../../docs/中文/参考/接口规格参考.md"),
    },
    DocPage {
        slug: "testing",
        title: "测试分层",
        description: "工作区测试 crate、夹具和验证流程。",
        section: "运维",
        path: "docs/中文/测试分层.md",
        markdown: include_str!("../../../docs/中文/测试分层.md"),
    },
    DocPage {
        slug: "disclaimer",
        title: "免责声明",
        description: "无担保、非隶属关系、合规责任和第三方权利说明。",
        section: "法律",
        path: "DISCLAIMER.zh-CN.md",
        markdown: include_str!("../../../DISCLAIMER.zh-CN.md"),
    },
];

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/:lang")]
    LanguageHome { lang: String },
    #[route("/:lang/:slug")]
    Doc { lang: String, slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let theme_css = docs_theme_css();
    let theme_preference = use_signal(read_stored_theme_preference);
    use_context_provider(|| theme_preference);
    let root_class = theme_preference().root_class();

    rsx! {
        style { "{theme_css}" }
        div { class: "{root_class}",
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        DocsLayout {
            language: Language::Chinese,
            slug: "home".to_string(),
        }
    }
}

#[component]
fn LanguageHome(lang: String) -> Element {
    match Language::from_code(&lang) {
        Some(language) => rsx! {
            DocsLayout {
                language,
                slug: "home".to_string(),
            }
        },
        None => rsx! {
            NotFoundView {
                requested: format!("/{lang}"),
            }
        },
    }
}

#[component]
fn Doc(lang: String, slug: String) -> Element {
    match Language::from_code(&lang) {
        Some(language) if find_page(language, &slug).is_some() => rsx! {
            DocsLayout {
                language,
                slug,
            }
        },
        _ => rsx! {
            NotFoundView {
                requested: format!("/{lang}/{slug}"),
            }
        },
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        NotFoundView {
            requested: format!("/{}", route.join("/")),
        }
    }
}

#[component]
fn DocsLayout(language: Language, slug: String) -> Element {
    let mut filter = use_signal(String::new);
    let pages = language.docs();
    let current_page = find_page(language, &slug).unwrap_or(&pages[0]);
    let rendered_markdown = render_markdown(current_page.markdown, current_page.path);
    let current_slug = current_page.slug;
    let normalized_filter = filter().trim().to_lowercase();
    let navigation_results = search_pages(pages, "");
    let search_results = if normalized_filter.is_empty() {
        Vec::new()
    } else {
        search_pages(pages, &normalized_filter)
    };
    let show_search_results = !normalized_filter.is_empty();
    let search_result_count = search_result_count_label(language, search_results.len());

    rsx! {
        div { class: "docs-shell",
            header { class: "topbar",
                Link {
                    class: "brand-link",
                    to: Route::LanguageHome {
                        lang: language.code().to_string(),
                    },
                    span { class: "brand-mark", "A" }
                    span { class: "brand-text",
                        strong { "Amagi-rs" }
                        small { "{language.docs_label()}" }
                    }
                }
                Link {
                    class: "route-pill read-before-link",
                    to: route_for(language, "disclaimer"),
                    "{language.read_before_use_label()}"
                }

                div { class: "topbar-actions",
                    label { class: "search-box topbar-search",
                        span { "{language.search_label()}" }
                        input {
                            value: "{filter}",
                            placeholder: "{language.search_placeholder()}",
                            oninput: move |event| filter.set(event.value()),
                        }
                    }
                    ThemeSwitcher {}
                    LanguageDropdown { language, current_slug }
                    a {
                        class: "source-button",
                        href: format!(
                            "https://github.com/bandange/amagi-rs/blob/main/{}",
                            current_page.path,
                        ),
                        target: "_blank",
                        rel: "noreferrer",
                        "{language.source_label()}"
                    }
                }
            }

            if show_search_results {
                section { class: "search-results-panel", "aria-label": "{language.search_results_label()}",
                    div { class: "search-results-inner",
                        div { class: "search-results-head",
                            p { class: "search-results-kicker", "{language.search_results_label()}" }
                            p { class: "search-results-count", "{search_result_count}" }
                        }
                        div { class: "search-results-list",
                            for result in &search_results {
                                Link {
                                    key: "{result.page.slug}",
                                    class: "search-result-link",
                                    to: route_for(language, result.page.slug),
                                    onclick: move |_| filter.set(String::new()),
                                    span { class: "search-result-title", "{result.page.title}" }
                                    span { class: "search-result-path", "{route_path_for(language, result.page.slug)}" }
                                    span { class: "search-result-description", "{result.page.description}" }
                                    if let Some(snippet) = &result.snippet {
                                        span { class: "search-result-snippet", "{snippet}" }
                                    }
                                }
                            }
                            if search_results.is_empty() {
                                p { class: "empty-state search-empty", "{language.empty_search_label()}" }
                            }
                        }
                    }
                }
            }

            aside { class: "sidebar",
                div { class: "sidebar-head",
                    p { class: "sidebar-kicker", "{language.sidebar_kicker()}" }
                    h1 { "{language.sidebar_title()}" }
                    p { "{language.sidebar_description()}" }
                }

                nav { class: "doc-nav", "aria-label": "Documents",
                    for (section, section_results) in grouped_search_results(&navigation_results) {
                        div { class: "nav-section", key: "{section}",
                            p { class: "nav-section-title", "{section}" }
                            for result in section_results {
                                Link {
                                    key: "{result.page.slug}",
                                    class: nav_link_class(result.page.slug == current_page.slug),
                                    to: route_for(language, result.page.slug),
                                    span { class: "nav-title", "{result.page.title}" }
                                    span { class: "nav-description", "{result.page.description}" }
                                }
                            }
                        }
                    }
                }
            }

            main { class: "content",
                div { class: "content-inner",
                    header { class: "doc-header",
                        div { class: "doc-meta-row",
                            span { class: "doc-section", "{current_page.section}" }
                            span { class: "doc-language", "{language.label()}" }
                        }
                        h2 { "{current_page.title}" }
                        p { "{current_page.description}" }
                        code { class: "doc-path", "{current_page.path}" }
                    }

                    article {
                        class: "markdown-body",
                        dangerous_inner_html: "{rendered_markdown}",
                    }

                    SiteFooter { language }
                }
            }
        }
    }
}

#[component]
fn SiteFooter(language: Language) -> Element {
    rsx! {
        footer { class: "site-footer",
            div { class: "site-footer-main",
                strong { "Amagi-rs" }
                span { "{language.version_label()} {env!(\"CARGO_PKG_VERSION\")}" }
                span { "{language.footer_note()}" }
            }
            nav { class: "site-footer-links", "aria-label": "Footer",
                Link {
                    to: route_for(language, "disclaimer"),
                    "{language.disclaimer_label()}"
                }
                a {
                    href: PROJECT_LICENSE_URL,
                    target: "_blank",
                    rel: "noreferrer",
                    "{language.license_label()}"
                }
                a {
                    href: PROJECT_REPOSITORY_URL,
                    target: "_blank",
                    rel: "noreferrer",
                    "{language.repository_label()}"
                }
            }
        }
    }
}

#[component]
fn LanguageDropdown(language: Language, current_slug: &'static str) -> Element {
    rsx! {
        details { class: "language-menu",
            summary {
                class: "language-menu-button",
                "aria-label": "{language.language_control_label()}",
                span { class: "language-menu-current", "{language.label()}" }
                span { class: "language-menu-chevron", "aria-hidden": "true" }
            }
            div { class: "language-menu-list", role: "listbox",
                for option_language in LANGUAGES {
                    Link {
                        key: "{option_language.code()}",
                        class: if *option_language == language {
                            "language-menu-option selected"
                        } else {
                            "language-menu-option"
                        },
                        to: route_for(*option_language, current_slug),
                        role: "option",
                        "aria-selected": "{*option_language == language}",
                        span { "{option_language.label()}" }
                    }
                }
            }
        }
    }
}

#[component]
fn ThemeSwitcher() -> Element {
    let mut theme_preference = use_context::<Signal<ThemePreference>>();
    let current = theme_preference();
    let next = current.next();

    rsx! {
        button {
            class: "theme-toggle",
            r#type: "button",
            title: format!("Theme: {}. Switch to {}.", current.label(), next.label()),
            "aria-label": format!("Theme: {}. Switch to {}.", current.label(), next.label()),
            onclick: move |_| {
                theme_preference.set(next);
                store_theme_preference(next);
            },
            span { class: "{current.icon_class()}" }
        }
    }
}

#[component]
fn NotFoundView(requested: String) -> Element {
    rsx! {
        div { class: "not-found-shell",
            div { class: "not-found-panel",
                span { class: "brand-mark", "A" }
                p { class: "sidebar-kicker", "404" }
                h1 { "Document not found" }
                p { "No Amagi-rs document is registered for {requested}." }
                div { class: "not-found-actions",
                    Link {
                        class: "source-button primary",
                        to: Route::LanguageHome {
                            lang: Language::Chinese.code().to_string(),
                        },
                        "中文文档"
                    }
                    Link {
                        class: "source-button",
                        to: Route::LanguageHome {
                            lang: Language::English.code().to_string(),
                        },
                        "English Docs"
                    }
                }
            }
        }
    }
}

fn find_page(language: Language, slug: &str) -> Option<&'static DocPage> {
    language.docs().iter().find(|page| page.slug == slug)
}

fn route_for(language: Language, slug: &str) -> Route {
    if slug == "home" {
        Route::LanguageHome {
            lang: language.code().to_string(),
        }
    } else if find_page(language, slug).is_some() {
        Route::Doc {
            lang: language.code().to_string(),
            slug: slug.to_string(),
        }
    } else {
        Route::LanguageHome {
            lang: language.code().to_string(),
        }
    }
}

fn search_pages(pages: &'static [DocPage], query: &str) -> Vec<SearchResult> {
    pages
        .iter()
        .filter_map(|page| search_page(page, query))
        .collect()
}

fn search_result_count_label(language: Language, count: usize) -> String {
    match language {
        Language::English if count == 1 => "1 result".to_string(),
        Language::English => format!("{count} results"),
        Language::Chinese => format!("{count} 个结果"),
    }
}

fn search_page(page: &'static DocPage, query: &str) -> Option<SearchResult> {
    if query.is_empty() {
        return Some(SearchResult {
            page,
            snippet: None,
        });
    }

    let title = page.title.to_lowercase();
    let description = page.description.to_lowercase();
    let path = page.path.to_lowercase();
    let markdown = compact_search_text(page.markdown);
    let normalized_markdown = markdown.to_lowercase();
    let metadata_matches = title.contains(query)
        || description.contains(query)
        || page.slug.contains(query)
        || path.contains(query);
    let snippet = search_snippet(&markdown, &normalized_markdown, query);

    if metadata_matches || snippet.is_some() {
        Some(SearchResult { page, snippet })
    } else {
        None
    }
}

fn grouped_search_results(results: &[SearchResult]) -> Vec<(&'static str, Vec<&SearchResult>)> {
    let mut groups: Vec<(&'static str, Vec<&SearchResult>)> = Vec::new();

    for result in results {
        match groups
            .iter_mut()
            .find(|(section, _)| *section == result.page.section)
        {
            Some((_, section_results)) => section_results.push(result),
            None => groups.push((result.page.section, vec![result])),
        }
    }

    groups
}

fn compact_search_text(markdown: &str) -> String {
    markdown.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn search_snippet(text: &str, normalized_text: &str, query: &str) -> Option<String> {
    let match_start = normalized_text.find(query)?;
    let match_end = match_start + query.len();
    let start = snippet_boundary_before(text, match_start, 42);
    let end = snippet_boundary_after(text, match_end, 94);
    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < text.len() { "..." } else { "" };

    Some(format!("{prefix}{}{suffix}", text[start..end].trim()))
}

fn snippet_boundary_before(text: &str, index: usize, max_chars: usize) -> usize {
    let boundary = nearest_char_boundary_before(text, index);

    text[..boundary]
        .char_indices()
        .rev()
        .nth(max_chars.saturating_sub(1))
        .map(|(position, _)| position)
        .unwrap_or(0)
}

fn snippet_boundary_after(text: &str, index: usize, max_chars: usize) -> usize {
    let boundary = nearest_char_boundary_after(text, index);

    text[boundary..]
        .char_indices()
        .nth(max_chars)
        .map(|(position, _)| boundary + position)
        .unwrap_or(text.len())
}

fn nearest_char_boundary_before(text: &str, mut index: usize) -> usize {
    index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn nearest_char_boundary_after(text: &str, mut index: usize) -> usize {
    index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

fn nav_link_class(selected: bool) -> &'static str {
    if selected {
        "nav-link selected"
    } else {
        "nav-link"
    }
}

fn render_markdown(markdown: &str, current_path: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser =
        Parser::new_ext(markdown, options).map(|event| rewrite_markdown_link(event, current_path));
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn rewrite_markdown_link<'a>(event: Event<'a>, current_path: &str) -> Event<'a> {
    match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let dest_url = rewrite_doc_link(current_path, dest_url.as_ref())
                .map(CowStr::from)
                .unwrap_or(dest_url);

            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            })
        }
        _ => event,
    }
}

fn rewrite_doc_link(current_path: &str, dest_url: &str) -> Option<String> {
    let dest_url = dest_url.trim();

    if dest_url.is_empty()
        || dest_url.starts_with('#')
        || dest_url.starts_with("//")
        || has_url_scheme(dest_url)
    {
        return None;
    }

    let (path, suffix) = split_link_suffix(dest_url);
    if !path.ends_with(".md") {
        return None;
    }

    let target_path = if path.starts_with('/') {
        normalize_doc_path(path.trim_start_matches('/'))
    } else {
        let base_dir = current_path.rsplit_once('/').map_or("", |(dir, _)| dir);
        normalize_doc_path(&format!("{base_dir}/{path}"))
    };

    doc_route_for_path(&target_path).map(|route| format!("{route}{suffix}"))
}

fn split_link_suffix(dest_url: &str) -> (&str, &str) {
    match dest_url.find(['#', '?']) {
        Some(index) => (&dest_url[..index], &dest_url[index..]),
        None => (dest_url, ""),
    }
}

fn has_url_scheme(dest_url: &str) -> bool {
    let Some(colon_index) = dest_url.find(':') else {
        return false;
    };
    let slash_index = dest_url.find('/').unwrap_or(usize::MAX);

    colon_index < slash_index
        && dest_url[..colon_index].chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
}

fn normalize_doc_path(path: &str) -> String {
    let mut parts = Vec::new();

    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }

    parts.join("/")
}

fn doc_route_for_path(path: &str) -> Option<String> {
    [Language::Chinese, Language::English]
        .into_iter()
        .flat_map(|language| language.docs().iter().map(move |page| (language, page)))
        .find_map(|(language, page)| {
            if page.path == path {
                Some(route_path_for(language, page.slug))
            } else {
                None
            }
        })
}

fn route_path_for(language: Language, slug: &str) -> String {
    if slug == "home" {
        format!("/{}", language.code())
    } else if find_page(language, slug).is_some() {
        format!("/{}/{}", language.code(), slug)
    } else {
        format!("/{}", language.code())
    }
}

fn docs_theme_css() -> String {
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

fn read_stored_theme_preference() -> ThemePreference {
    storage_get(THEME_STORAGE_KEY)
        .as_deref()
        .and_then(ThemePreference::from_storage)
        .unwrap_or(ThemePreference::System)
}

fn store_theme_preference(preference: ThemePreference) {
    storage_set(THEME_STORAGE_KEY, preference.storage_value());
}

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(key)
        .ok()
        .flatten()
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_get(_key: &str) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(key, value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_set(_key: &str, _value: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_same_language_markdown_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("README.zh-CN.md", "docs/中文/参考/命令行参考.md").as_deref(),
            Some("/zh/cli")
        );
        assert_eq!(
            rewrite_doc_link("README.md", "docs/en/installation.md").as_deref(),
            Some("/en/installation")
        );
    }

    #[test]
    fn rewrites_root_readme_language_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("README.md", "README.zh-CN.md").as_deref(),
            Some("/zh")
        );
        assert_eq!(
            rewrite_doc_link("README.zh-CN.md", "README.md").as_deref(),
            Some("/en")
        );
    }

    #[test]
    fn rewrites_cross_language_markdown_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("docs/中文/参考/命令行参考.md", "../../en/reference/cli.md")
                .as_deref(),
            Some("/en/cli")
        );
        assert_eq!(
            rewrite_doc_link("docs/en/testing.md", "../中文/测试分层.md#fixtures").as_deref(),
            Some("/zh/testing#fixtures")
        );
    }

    #[test]
    fn rewrites_root_disclaimer_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("DISCLAIMER.md", "DISCLAIMER.zh-CN.md").as_deref(),
            Some("/zh/disclaimer")
        );
        assert_eq!(
            rewrite_doc_link("DISCLAIMER.zh-CN.md", "DISCLAIMER.md").as_deref(),
            Some("/en/disclaimer")
        );
    }

    #[test]
    fn leaves_external_and_anchor_links_unchanged() {
        assert_eq!(
            rewrite_doc_link("README.md", "https://example.com/readme.md"),
            None
        );
        assert_eq!(rewrite_doc_link("README.md", "#overview"), None);
    }

    #[test]
    fn empty_search_returns_every_page_without_snippets() {
        let results = search_pages(CHINESE_DOCS, "");

        assert_eq!(results.len(), CHINESE_DOCS.len());
        assert!(results.iter().all(|result| result.snippet.is_none()));
    }

    #[test]
    fn search_matches_markdown_body_content() {
        let results = search_pages(ENGLISH_DOCS, "x-amagi-cookie");

        assert!(
            results
                .iter()
                .any(|result| result.page.slug == "web-api" && result.snippet.is_some())
        );
    }

    #[test]
    fn search_matches_chinese_markdown_body_content() {
        let results = search_pages(CHINESE_DOCS, "游客模式");

        assert!(
            results
                .iter()
                .any(|result| result.page.slug == "web-api" && result.snippet.is_some())
        );
    }
}
