#![doc = include_str!("../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![warn(missing_docs)]

/// Application runtime and top-level command dispatch for a resolved [`config::AppConfig`].
#[cfg(feature = "cli")]
pub mod app;
/// Shared catalog metadata and route descriptors for the client and server surfaces.
#[cfg(feature = "catalog")]
pub mod catalog;
/// Command-line argument parsing and conversion into [`config::AppConfig`].
#[cfg(feature = "cli")]
pub mod cli;
/// Client configuration and platform accessors.
#[cfg(feature = "client")]
pub mod client;
/// Shared configuration types used by the CLI, runtime, and HTTP server.
#[cfg(any(feature = "cli", feature = "server"))]
pub mod config;
/// `.env` loading and environment resolution helpers shared across runtimes.
#[cfg(any(feature = "client", feature = "cli", feature = "server"))]
pub mod env;
/// Shared error types used across the crate.
#[cfg(feature = "client")]
pub mod error;
/// Typed event bus shared across the client, CLI, and server flows.
#[cfg(feature = "client")]
pub mod events;
/// Human-readable and machine-readable output helpers.
#[cfg(any(feature = "cli", feature = "server"))]
pub mod output;
/// Rust-native platform fetchers and shared upstream transport adapters.
#[cfg(feature = "client")]
pub mod platforms;
/// HTTP server entrypoint, router, and handlers.
#[cfg(feature = "server")]
pub mod server;
/// Structured telemetry setup backed by `tracing`.
#[cfg(feature = "cli")]
pub mod telemetry;

/// Core catalog types and lookup helpers re-exported at the crate root.
#[cfg(feature = "catalog")]
pub use catalog::{
    ApiMethodSpec, HttpMethod, ParsePlatformError, Platform, PlatformSpec, all_platform_specs,
    find_method, get_api_route, get_chinese_method_name, get_english_method_name, method_specs,
    platform_spec,
};
/// Core client types and constructors re-exported at the crate root.
#[cfg(feature = "client")]
pub use client::{
    AmagiClient, ClientOptions, CookieConfig, PlatformClient, RequestConfig, RequestProfile,
    create_amagi_client,
};
/// Shared application error type re-exported at the crate root.
#[cfg(feature = "client")]
pub use error::AppError;
/// Typed event bus exports re-exported at the crate root.
#[cfg(feature = "client")]
pub use events::{
    AmagiEvent, AmagiEventType, ApiErrorEventData, ApiSuccessEventData, EventBus, EventLogLevel,
    HttpRequestEventData, HttpResponseEventData, LogEventData, NetworkErrorEventData,
    NetworkRetryEventData,
};
/// Platform client modules re-exported at the crate root for direct access.
#[cfg(feature = "client")]
pub use platforms::{bilibili, douyin, kuaishou, twitter, xiaohongshu};

/// Crate name used by CLI output and server metadata.
pub const APP_NAME: &str = "amagi";
/// Default host used by the built-in HTTP server.
pub const DEFAULT_HOST: &str = "127.0.0.1";
/// Default port used by the built-in HTTP server.
pub const DEFAULT_PORT: u16 = 4567;


/// Parse process arguments and run the application.
///
/// This is a convenience wrapper around [`cli::parse_env`] followed by [`app::run`].
///
/// # Errors
///
/// Returns an error when startup or runtime initialization fails.
#[cfg(feature = "cli")]
pub async fn run_env() -> Result<(), error::AppError> {
    let config = cli::try_parse_env()?;
    app::run(config).await
}

/// Print an [`error::AppError`] in a human-friendly fallback format.
///
/// This helper is intended for early startup failures before structured output
/// has been configured.
#[cfg(feature = "cli")]
pub fn print_startup_error(error: &error::AppError) {
    output::print_startup_error(APP_NAME, error);
}
