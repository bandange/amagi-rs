#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Language {
    English,
    Chinese,
}

impl Language {
    pub(crate) fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Chinese => "zh",
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "中文",
        }
    }

    pub(crate) fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" | "en-US" => Some(Language::English),
            "zh" | "zh-CN" | "cn" => Some(Language::Chinese),
            _ => None,
        }
    }

    pub(crate) fn search_label(self) -> &'static str {
        match self {
            Language::English => "Search",
            Language::Chinese => "搜索",
        }
    }

    pub(crate) fn search_placeholder(self) -> &'static str {
        match self {
            Language::English => "Search content",
            Language::Chinese => "搜索全文",
        }
    }

    pub(crate) fn empty_search_label(self) -> &'static str {
        match self {
            Language::English => "No matching content.",
            Language::Chinese => "没有匹配内容。",
        }
    }

    pub(crate) fn docs_label(self) -> &'static str {
        match self {
            Language::English => "Docs",
            Language::Chinese => "文档",
        }
    }

    pub(crate) fn language_control_label(self) -> &'static str {
        match self {
            Language::English => "Language",
            Language::Chinese => "语言",
        }
    }

    pub(crate) fn source_label(self) -> &'static str {
        match self {
            Language::English => "Source",
            Language::Chinese => "源文件",
        }
    }

    pub(crate) fn search_results_label(self) -> &'static str {
        match self {
            Language::English => "Search results",
            Language::Chinese => "搜索结果",
        }
    }

    pub(crate) fn sidebar_kicker(self) -> &'static str {
        match self {
            Language::English => "Workspace",
            Language::Chinese => "工作区",
        }
    }

    pub(crate) fn sidebar_title(self) -> &'static str {
        match self {
            Language::English => "Amagi-rs Documentation",
            Language::Chinese => "Amagi-rs 文档",
        }
    }

    pub(crate) fn sidebar_description(self) -> &'static str {
        match self {
            Language::English => "Rust API, CLI, service API, and platform references.",
            Language::Chinese => "Rust API、命令行、服务接口和平台参考。",
        }
    }

    pub(crate) fn footer_note(self) -> &'static str {
        match self {
            Language::English => "Non-official project. Use responsibly.",
            Language::Chinese => "非官方项目，请自行确保合规使用。",
        }
    }

    pub(crate) fn version_label(self) -> &'static str {
        match self {
            Language::English => "Version",
            Language::Chinese => "版本",
        }
    }

    pub(crate) fn disclaimer_label(self) -> &'static str {
        match self {
            Language::English => "Disclaimer",
            Language::Chinese => "免责声明",
        }
    }

    pub(crate) fn license_label(self) -> &'static str {
        match self {
            Language::English => "License",
            Language::Chinese => "许可证",
        }
    }

    pub(crate) fn repository_label(self) -> &'static str {
        match self {
            Language::English => "Source",
            Language::Chinese => "源码",
        }
    }

    pub(crate) fn read_before_use_label(self) -> &'static str {
        match self {
            Language::English => "Read before use",
            Language::Chinese => "使用前请阅读",
        }
    }

    pub(crate) fn page_nav_label(self) -> &'static str {
        match self {
            Language::English => "On this page",
            Language::Chinese => "本页导航",
        }
    }
}

pub(crate) const LANGUAGES: &[Language] = &[Language::Chinese, Language::English];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DocPage {
    pub(crate) slug: &'static str,
    pub(crate) title: &'static str,
    pub(crate) description: &'static str,
    pub(crate) section: &'static str,
    pub(crate) path: &'static str,
    pub(crate) markdown: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SearchResult {
    pub(crate) page: &'static DocPage,
    pub(crate) snippet: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ThemePreference {
    System,
    Light,
    Dark,
}

impl ThemePreference {
    pub(crate) fn label(self) -> &'static str {
        match self {
            ThemePreference::System => "System",
            ThemePreference::Light => "Light",
            ThemePreference::Dark => "Dark",
        }
    }

    pub(crate) fn storage_value(self) -> &'static str {
        match self {
            ThemePreference::System => "system",
            ThemePreference::Light => "light",
            ThemePreference::Dark => "dark",
        }
    }

    pub(crate) fn root_class(self) -> &'static str {
        match self {
            ThemePreference::System => "theme-root theme-system",
            ThemePreference::Light => "theme-root theme-light",
            ThemePreference::Dark => "theme-root theme-dark",
        }
    }

    pub(crate) fn from_storage(value: &str) -> Option<Self> {
        match value {
            "system" | "auto" => Some(ThemePreference::System),
            "light" => Some(ThemePreference::Light),
            "dark" => Some(ThemePreference::Dark),
            _ => None,
        }
    }

    pub(crate) fn next(self) -> Self {
        match self {
            ThemePreference::System => ThemePreference::Light,
            ThemePreference::Light => ThemePreference::Dark,
            ThemePreference::Dark => ThemePreference::System,
        }
    }

    pub(crate) fn icon_class(self) -> &'static str {
        match self {
            ThemePreference::System => "theme-icon theme-icon-system",
            ThemePreference::Light => "theme-icon theme-icon-light",
            ThemePreference::Dark => "theme-icon theme-icon-dark",
        }
    }
}
