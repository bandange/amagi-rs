#![cfg(feature = "server")]

use amagi_client::CookieConfig;
use amagi_server::server::{CookieReloadMode, CookieReloadPlan};

#[test]
fn cookie_reload_plan_preserves_cli_overrides_and_tracks_env_backed_values() {
    let startup = CookieConfig {
        douyin: Some("cli-douyin".into()),
        bilibili: Some("env-bilibili".into()),
        kuaishou: None,
        twitter: Some("env-twitter".into()),
        xiaohongshu: Some("cli-xhs".into()),
    };
    let startup_env = CookieConfig {
        douyin: Some("env-douyin".into()),
        bilibili: Some("env-bilibili".into()),
        kuaishou: None,
        twitter: Some("env-twitter".into()),
        xiaohongshu: None,
    };
    let refreshed_env = CookieConfig {
        douyin: Some("env-douyin-new".into()),
        bilibili: Some("env-bilibili-new".into()),
        kuaishou: Some("env-kuaishou-new".into()),
        twitter: None,
        xiaohongshu: Some("env-xhs-new".into()),
    };

    let plan = CookieReloadPlan::from_startup(&startup, &startup_env);

    assert_eq!(plan.douyin, CookieReloadMode::PinnedSnapshot);
    assert_eq!(plan.bilibili, CookieReloadMode::LayeredEnv);
    assert_eq!(plan.kuaishou, CookieReloadMode::LayeredEnv);
    assert_eq!(plan.twitter, CookieReloadMode::LayeredEnv);
    assert_eq!(plan.xiaohongshu, CookieReloadMode::PinnedSnapshot);

    let resolved = plan.resolve(&startup, &refreshed_env);
    assert_eq!(resolved.douyin.as_deref(), Some("cli-douyin"));
    assert_eq!(resolved.bilibili.as_deref(), Some("env-bilibili-new"));
    assert_eq!(resolved.kuaishou.as_deref(), Some("env-kuaishou-new"));
    assert_eq!(resolved.twitter, None);
    assert_eq!(resolved.xiaohongshu.as_deref(), Some("cli-xhs"));
}
