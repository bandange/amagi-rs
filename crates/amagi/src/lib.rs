#![doc = include_str!("../../../README.md")]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![warn(missing_docs)]

#[cfg(any(feature = "adapters", feature = "platforms"))]
pub use amagi_adapters::spec::{
    all_platform_api_specs, all_platform_specs, find_method, find_operation, get_api_route,
    get_chinese_method_name, get_chinese_operation_name, get_english_method_name, get_fetcher_name,
    get_operation_route, method_specs, operation_specs, platform_api_spec, platform_spec,
};
#[cfg(any(feature = "adapters", feature = "platforms"))]
pub use amagi_adapters::{AdapterContext, PlatformClient};
#[cfg(any(feature = "adapters", feature = "platforms"))]
pub use amagi_adapters::{bilibili, douyin, kuaishou, twitter, xiaohongshu};
#[cfg(feature = "cli")]
pub use amagi_cli::{print_startup_error, run, run_env};
#[cfg(feature = "client")]
pub use amagi_client::{
    AmagiClient, AmagiEvent, AmagiEventType, ApiErrorEventData, ApiSuccessEventData, EventBus,
    EventLogLevel, HttpRequestEventData, HttpResponseEventData, LogEventData,
    NetworkErrorEventData, NetworkRetryEventData, create_amagi_client,
};
#[cfg(feature = "core")]
pub use amagi_core::{
    APP_NAME, ApiMethodSpec, ApiOperationSpec, AppError, ClientOptions, CookieConfig, DEFAULT_HOST,
    DEFAULT_PORT, HttpMethod, ParsePlatformError, Platform, PlatformApiSpec, PlatformSpec,
    RequestConfig, RequestProfile, build_info,
};

/// Compatibility exports for the former internal `catalog` module.
#[cfg(feature = "catalog")]
pub mod catalog {
    pub use amagi_adapters::spec::{
        all_platform_api_specs, all_platform_specs, find_method, find_operation, get_api_route,
        get_chinese_method_name, get_chinese_operation_name, get_english_method_name,
        get_fetcher_name, get_operation_route, method_specs, operation_specs, platform_api_spec,
        platform_spec,
    };
    pub use amagi_core::{
        ApiMethodSpec, ApiOperationSpec, HttpMethod, ParsePlatformError, Platform, PlatformApiSpec,
        PlatformSpec,
    };
}

/// Shared error types used across the crate.
#[cfg(feature = "core")]
pub mod error {
    pub use amagi_core::error::*;
}

/// `.env` loading and environment resolution helpers shared across runtimes.
#[cfg(feature = "core")]
pub mod env {
    pub use amagi_core::env::*;
}

/// SDK-style client configuration and platform accessors.
#[cfg(feature = "client")]
pub mod client {
    pub use amagi_client::*;
}

/// Typed event bus shared across client, CLI, and server flows.
#[cfg(feature = "client")]
pub mod events {
    pub use amagi_client::events::*;
}

/// Rust-native adapter implementations and shared upstream transport helpers.
#[cfg(any(feature = "adapters", feature = "platforms"))]
pub mod adapters {
    pub use amagi_adapters::*;
}

/// Compatibility module for the former platform-adapter module name.
#[cfg(any(feature = "adapters", feature = "platforms"))]
pub mod platforms {
    pub use amagi_adapters::*;
}

/// HTTP server entrypoint and helpers.
#[cfg(feature = "server")]
pub mod server {
    pub use amagi_server::serve;
    pub use amagi_server::server::*;
}

/// Node-network support for server-mode multi-node deployments.
#[cfg(feature = "server")]
pub mod node {
    pub use amagi_server::node::*;
}

/// Command-line argument parsing.
#[cfg(feature = "cli")]
pub mod cli {
    pub use amagi_cli::cli::*;
}

/// Shared configuration types used by the CLI and HTTP server.
#[cfg(any(feature = "cli", feature = "server"))]
pub mod config {
    pub use amagi_cli::config::*;
}

/// Human-readable and machine-readable output helpers.
#[cfg(any(feature = "cli", feature = "server"))]
pub mod output {
    pub use amagi_cli::output::*;
}

/// Application runtime and top-level command dispatch.
#[cfg(feature = "cli")]
pub mod app {
    pub use amagi_cli::app::*;
}

/// Structured telemetry setup backed by `tracing`.
#[cfg(feature = "cli")]
pub mod telemetry {
    pub use amagi_cli::telemetry::*;
}
