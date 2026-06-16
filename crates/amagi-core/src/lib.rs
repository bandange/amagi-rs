//! Shared core types, errors, and environment helpers for amagi crates.

pub mod defaults;
pub mod env;
pub mod error;
pub mod platform;
pub mod request;
pub mod spec;

pub use env::{
    DotenvMap, dotenv_values, dotenv_values_from_layers, dotenv_values_from_path, env_or_dotenv,
    user_dotenv_path, user_dotenv_values,
};
pub use error::AppError;
pub use platform::{ParsePlatformError, Platform};
pub use request::{ClientOptions, CookieConfig, RequestConfig, RequestProfile};
pub use spec::{ApiMethodSpec, ApiOperationSpec, HttpMethod, PlatformApiSpec, PlatformSpec};

/// Crate name used by CLI output and server metadata.
pub const APP_NAME: &str = "amagi";
/// Default host used by the built-in HTTP server.
pub const DEFAULT_HOST: &str = "127.0.0.1";
/// Default port used by the built-in HTTP server.
pub const DEFAULT_PORT: u16 = 4567;

/// Minimal readiness output interface used by the HTTP server.
pub trait ServerReadyPrinter {
    /// Print readiness information for a listening HTTP server.
    ///
    /// # Errors
    ///
    /// Returns an error when output cannot be written.
    fn print_server_ready_message(
        &self,
        app_name: &str,
        version: &str,
        bind_addr: &str,
    ) -> Result<(), AppError>;
}
