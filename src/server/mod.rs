//! HTTP server bootstrap, routing, and request handlers.

use std::net::SocketAddr;

mod handlers;
mod router;
pub(crate) mod runtime;
pub(crate) mod state;

use tokio::net::TcpListener;
use tracing::info;

use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::ServeConfig;
use crate::error::AppError;
use crate::node;
use crate::output::Printer;
use runtime::ServerRuntimeConfig;

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

    let state = state::AppState::new(
        APP_NAME,
        env!("CARGO_PKG_VERSION"),
        config.clone(),
        client,
        runtime,
    )?;
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

async fn shutdown_signal(state: state::AppState) {
    if tokio::signal::ctrl_c().await.is_ok() {
        state.broadcast_shutdown_notice(Some("server shutdown"), Some(3_000));
        let _ = state.announce_upstream_drain(Some("server shutdown"));
        info!("shutdown signal received");
    }
}
