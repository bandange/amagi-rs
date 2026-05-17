//! HTTP server bootstrap, routing, and request handlers.

use std::net::SocketAddr;
use std::time::Duration;

mod handlers;
mod router;
pub(crate) mod runtime;
pub(crate) mod state;

use tokio::net::TcpListener;
use tracing::{debug, info, warn};

use crate::APP_NAME;
use crate::client::{AmagiClient, ClientOptions, CookieConfig};
use crate::config::ServeConfig;
use crate::error::AppError;
use crate::node;
use crate::output::Printer;
use runtime::ServerRuntimeConfig;

const COOKIE_RELOAD_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CookieReloadMode {
    PinnedSnapshot,
    LayeredEnv,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CookieReloadPlan {
    douyin: CookieReloadMode,
    bilibili: CookieReloadMode,
    kuaishou: CookieReloadMode,
    twitter: CookieReloadMode,
    xiaohongshu: CookieReloadMode,
}

impl CookieReloadPlan {
    fn from_startup(startup: &CookieConfig, layered_env: &CookieConfig) -> Self {
        Self {
            douyin: reload_mode_for_cookie(&startup.douyin, &layered_env.douyin),
            bilibili: reload_mode_for_cookie(&startup.bilibili, &layered_env.bilibili),
            kuaishou: reload_mode_for_cookie(&startup.kuaishou, &layered_env.kuaishou),
            twitter: reload_mode_for_cookie(&startup.twitter, &layered_env.twitter),
            xiaohongshu: reload_mode_for_cookie(&startup.xiaohongshu, &layered_env.xiaohongshu),
        }
    }

    fn pinned_snapshot() -> Self {
        Self {
            douyin: CookieReloadMode::PinnedSnapshot,
            bilibili: CookieReloadMode::PinnedSnapshot,
            kuaishou: CookieReloadMode::PinnedSnapshot,
            twitter: CookieReloadMode::PinnedSnapshot,
            xiaohongshu: CookieReloadMode::PinnedSnapshot,
        }
    }

    fn resolve(&self, startup: &CookieConfig, layered_env: &CookieConfig) -> CookieConfig {
        CookieConfig {
            douyin: resolve_cookie_value(self.douyin, &startup.douyin, &layered_env.douyin),
            bilibili: resolve_cookie_value(self.bilibili, &startup.bilibili, &layered_env.bilibili),
            kuaishou: resolve_cookie_value(self.kuaishou, &startup.kuaishou, &layered_env.kuaishou),
            twitter: resolve_cookie_value(self.twitter, &startup.twitter, &layered_env.twitter),
            xiaohongshu: resolve_cookie_value(
                self.xiaohongshu,
                &startup.xiaohongshu,
                &layered_env.xiaohongshu,
            ),
        }
    }
}

/// Bind the configured address, print startup output, and serve the HTTP app.
///
/// This function blocks until a shutdown signal is received or the server
/// returns an error.
///
/// # Errors
///
/// Returns an error if the TCP listener cannot be bound, startup output cannot
/// be written, or the HTTP server fails while serving requests.
pub async fn serve(
    config: ServeConfig,
    client: AmagiClient,
    printer: &Printer,
) -> Result<(), AppError> {
    let bind_addr = config.bind_addr();
    let listener = TcpListener::bind(&bind_addr).await?;
    let local_addr = listener.local_addr()?.to_string();
    let runtime =
        ServerRuntimeConfig::from_env_with_overrides(|name| config.runtime_override(name))?;
    let startup_cookies = client.options().cookies.clone();
    let cookie_reload_plan = match ClientOptions::from_env() {
        Ok(options) => CookieReloadPlan::from_startup(&startup_cookies, &options.cookies),
        Err(error) => {
            warn!(
                app = APP_NAME,
                mode = "server",
                error = %error,
                "failed to inspect layered env cookies at startup; cookie auto-reload will reuse the startup snapshot"
            );
            CookieReloadPlan::pinned_snapshot()
        }
    };

    let state = state::AppState::new(
        APP_NAME,
        env!("CARGO_PKG_VERSION"),
        config.clone(),
        client,
        runtime,
    )?;
    spawn_cookie_reload_task(state.clone(), startup_cookies, cookie_reload_plan);
    node::client::spawn_upstream_connector(state.clone());
    let app = router::build(state.clone());

    printer.print_banner(APP_NAME, env!("CARGO_PKG_VERSION"))?;
    printer.print_server_ready(APP_NAME, env!("CARGO_PKG_VERSION"), &local_addr)?;

    info!(
        app = APP_NAME,
        mode = "server",
        addr = %local_addr,
        "http server listening"
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(state))
    .await?;

    Ok(())
}

fn spawn_cookie_reload_task(
    state: state::AppState,
    startup_cookies: CookieConfig,
    plan: CookieReloadPlan,
) {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(COOKIE_RELOAD_INTERVAL).await;

            let layered_env_cookies = match ClientOptions::from_env() {
                Ok(options) => options.cookies,
                Err(error) => {
                    warn!(
                        app = APP_NAME,
                        mode = "server",
                        error = %error,
                        "cookie auto-reload skipped because layered env resolution failed"
                    );
                    continue;
                }
            };

            let next_cookies = plan.resolve(&startup_cookies, &layered_env_cookies);
            let changed = state.replace_cookies(next_cookies);

            if changed {
                info!(
                    app = APP_NAME,
                    mode = "server",
                    interval_seconds = COOKIE_RELOAD_INTERVAL.as_secs(),
                    "reloaded configured cookies for service mode"
                );
            } else {
                debug!(
                    app = APP_NAME,
                    mode = "server",
                    interval_seconds = COOKIE_RELOAD_INTERVAL.as_secs(),
                    "cookie auto-reload completed without changes"
                );
            }
        }
    });
}

fn reload_mode_for_cookie(
    startup: &Option<String>,
    layered_env: &Option<String>,
) -> CookieReloadMode {
    if startup == layered_env {
        CookieReloadMode::LayeredEnv
    } else {
        CookieReloadMode::PinnedSnapshot
    }
}

fn resolve_cookie_value(
    mode: CookieReloadMode,
    startup: &Option<String>,
    layered_env: &Option<String>,
) -> Option<String> {
    match mode {
        CookieReloadMode::PinnedSnapshot => startup.clone(),
        CookieReloadMode::LayeredEnv => layered_env.clone(),
    }
}

async fn shutdown_signal(state: state::AppState) {
    if tokio::signal::ctrl_c().await.is_ok() {
        state.broadcast_shutdown_notice(Some("server shutdown"), Some(3_000));
        let _ = state.announce_upstream_drain(Some("server shutdown"));
        info!("shutdown signal received");
    }
}

#[cfg(test)]
mod tests {
    use super::{CookieReloadMode, CookieReloadPlan};
    use crate::client::CookieConfig;

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
}
