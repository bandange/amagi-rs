use crate::models::{DocPage, Language};

pub(crate) const ENGLISH_DOCS: &[DocPage] = &[
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

pub(crate) const CHINESE_DOCS: &[DocPage] = &[
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

pub(crate) fn docs_for(language: Language) -> &'static [DocPage] {
    match language {
        Language::English => ENGLISH_DOCS,
        Language::Chinese => CHINESE_DOCS,
    }
}

pub(crate) fn find_page(language: Language, slug: &str) -> Option<&'static DocPage> {
    docs_for(language).iter().find(|page| page.slug == slug)
}

pub(crate) fn route_path_for(language: Language, slug: &str) -> String {
    if slug == "home" {
        format!("/{}", language.code())
    } else if find_page(language, slug).is_some() {
        format!("/{}/{}", language.code(), slug)
    } else {
        format!("/{}", language.code())
    }
}
