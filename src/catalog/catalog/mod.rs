use super::{Platform, PlatformSpec};

mod bilibili;
mod douyin;
mod kuaishou;
mod twitter;
mod xiaohongshu;

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

/// Return the catalog for a single platform.
pub const fn platform_spec(platform: Platform) -> PlatformSpec {
    match platform {
        Platform::Bilibili => PlatformSpec {
            platform,
            api_base_path: Platform::Bilibili.api_base_path(),
            methods: &bilibili::BILIBILI_METHODS,
        },
        Platform::Douyin => PlatformSpec {
            platform,
            api_base_path: Platform::Douyin.api_base_path(),
            methods: &douyin::DOUYIN_METHODS,
        },
        Platform::Kuaishou => PlatformSpec {
            platform,
            api_base_path: Platform::Kuaishou.api_base_path(),
            methods: &kuaishou::KUAISHOU_METHODS,
        },
        Platform::Xiaohongshu => PlatformSpec {
            platform,
            api_base_path: Platform::Xiaohongshu.api_base_path(),
            methods: &xiaohongshu::XIAOHONGSHU_METHODS,
        },
        Platform::Twitter => PlatformSpec {
            platform,
            api_base_path: Platform::Twitter.api_base_path(),
            methods: &twitter::TWITTER_METHODS,
        },
    }
}

/// Return the full platform catalog in a stable display order.
pub fn all_platform_specs() -> [PlatformSpec; 5] {
    Platform::ALL.map(platform_spec)
}
