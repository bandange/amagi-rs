//! Command-line runtime for amagi.

pub mod app;
pub mod cli;
pub mod config;
pub mod output;
pub mod telemetry;

pub use amagi_core::{APP_NAME, DEFAULT_HOST, DEFAULT_PORT};
pub use app::run;
pub use cli::{parse_env, parse_from, try_parse_env};

/// Parse process arguments and run the application.
///
/// # Errors
///
/// Returns an error when startup or runtime initialization fails.
pub async fn run_env() -> Result<(), amagi_core::AppError> {
    let config = cli::try_parse_env()?;
    app::run(config).await
}

/// Print an [`amagi_core::AppError`] in a human-friendly fallback format.
pub fn print_startup_error(error: &amagi_core::AppError) {
    output::print_startup_error(amagi_core::APP_NAME, error);
}
