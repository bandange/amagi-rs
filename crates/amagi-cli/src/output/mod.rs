//! User-facing startup output for text and JSON modes.

mod printer;

/// Output printer used by CLI and server startup flows.
pub use printer::Printer;

use amagi_core::AppError;

/// Print a plain-text startup error before a [`Printer`] is available.
pub fn print_startup_error(app_name: &str, error: &AppError) {
    eprintln!("[{app_name}] {}: {error}", startup_error_label());
}

/// Print a localized plain-text startup message before a [`Printer`] is available.
pub fn print_startup_message(app_name: &str, message_en: &str, message_zh: &str) {
    let message = localized_startup_text(message_en, message_zh);
    eprintln!("[{app_name}] {}: {message}", startup_error_label());
}

fn startup_error_label() -> &'static str {
    localized_startup_text("error", "错误")
}

fn localized_startup_text<'a>(message_en: &'a str, message_zh: &'a str) -> &'a str {
    match crate::cli::resolve_runtime_language(None, None).code() {
        "zh-CN" => message_zh,
        _ => message_en,
    }
}
