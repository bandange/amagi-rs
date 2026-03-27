//! User-facing startup output for text and JSON modes.

mod printer;

/// Output printer used by CLI and server startup flows.
pub use printer::Printer;

use crate::error::AppError;

/// Print a plain-text startup error before a [`Printer`] is available.
pub fn print_startup_error(app_name: &str, error: &AppError) {
    eprintln!("[{app_name}] error: {error}");
}
