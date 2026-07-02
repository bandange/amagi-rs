//! Published API metadata for every platform adapter.

use amagi_core::{Platform, PlatformApiSpec};

mod bilibili;
mod douyin;
mod kuaishou;
mod lookup;
mod twitter;
mod xiaohongshu;

pub use lookup::{
    find_method, find_operation, get_api_route, get_chinese_method_name,
    get_chinese_operation_name, get_english_method_name, get_fetcher_name, get_operation_route,
    method_specs, operation_specs,
};

pub(super) const TAG_WORK: &[&str] = &["work"];
pub(super) const TAG_COMMENT: &[&str] = &["comment"];
pub(super) const TAG_USER: &[&str] = &["user"];
pub(super) const TAG_SEARCH: &[&str] = &["search"];
pub(super) const TAG_AUTH: &[&str] = &["auth"];
pub(super) const TAG_EMOJI: &[&str] = &["emoji"];
pub(super) const TAG_DYNAMIC: &[&str] = &["dynamic"];
pub(super) const TAG_BANGUMI: &[&str] = &["bangumi"];
pub(super) const TAG_LIVE: &[&str] = &["live"];
pub(super) const TAG_SPACE: &[&str] = &["space"];
pub(super) const TAG_ARTICLE: &[&str] = &["article"];
pub(super) const TAG_TOOL: &[&str] = &["tool"];
pub(super) const TAG_CAPTCHA: &[&str] = &["captcha"];
pub(super) const TAG_DANMAKU: &[&str] = &["danmaku"];
pub(super) const TAG_MUSIC: &[&str] = &["music"];
pub(super) const TAG_FEED: &[&str] = &["feed"];

/// Return API metadata for a single platform adapter.
pub const fn platform_api_spec(platform: Platform) -> PlatformApiSpec {
    match platform {
        Platform::Bilibili => PlatformApiSpec {
            platform,
            api_base_path: Platform::Bilibili.api_base_path(),
            methods: &bilibili::BILIBILI_METHODS,
        },
        Platform::Douyin => PlatformApiSpec {
            platform,
            api_base_path: Platform::Douyin.api_base_path(),
            methods: &douyin::DOUYIN_METHODS,
        },
        Platform::Kuaishou => PlatformApiSpec {
            platform,
            api_base_path: Platform::Kuaishou.api_base_path(),
            methods: &kuaishou::KUAISHOU_METHODS,
        },
        Platform::Xiaohongshu => PlatformApiSpec {
            platform,
            api_base_path: Platform::Xiaohongshu.api_base_path(),
            methods: &xiaohongshu::XIAOHONGSHU_METHODS,
        },
        Platform::Twitter => PlatformApiSpec {
            platform,
            api_base_path: Platform::Twitter.api_base_path(),
            methods: &twitter::TWITTER_METHODS,
        },
    }
}

/// Return all platform API metadata in a stable display order.
pub fn all_platform_api_specs() -> [PlatformApiSpec; 5] {
    Platform::ALL.map(platform_api_spec)
}

/// Compatibility wrapper for the former catalog-oriented name.
pub const fn platform_spec(platform: Platform) -> PlatformApiSpec {
    platform_api_spec(platform)
}

/// Compatibility wrapper for the former catalog-oriented name.
pub fn all_platform_specs() -> [PlatformApiSpec; 5] {
    all_platform_api_specs()
}
