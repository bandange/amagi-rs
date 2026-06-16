//! Global tracing subscriber initialization and runtime telemetry defaults.

use std::fmt;
use std::io::IsTerminal;
use std::sync::OnceLock;

use tracing_subscriber::EnvFilter;

use crate::config::{LogFormat, LogLevel, LoggingConfig};

const RUST_LOG_ENV: &str = "RUST_LOG";

static TELEMETRY_INIT: OnceLock<Result<(), TelemetryInitError>> = OnceLock::new();

/// Error returned when the global telemetry subscriber cannot be installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TelemetryInitError {
    message: String,
}

impl TelemetryInitError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for TelemetryInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for TelemetryInitError {}

/// Initialize the global tracing subscriber once.
///
/// The first call selects the formatter and default level from `config`. If the
/// standard `RUST_LOG` environment variable is set, it takes precedence over
/// the configured level. Subsequent calls after a successful initialization are
/// ignored.
///
/// # Panics
///
/// Panics if the global telemetry subscriber cannot be installed.
pub fn init(config: LoggingConfig) {
    if let Err(error) = try_init(config) {
        panic!("failed to initialize telemetry: {error}");
    }
}

/// Try to initialize the global tracing subscriber once.
///
/// The result of the first initialization attempt is cached and returned on
/// subsequent calls.
///
/// # Errors
///
/// Returns an error if another global tracing subscriber has already been
/// installed or if the subscriber configuration is invalid.
pub fn try_init(config: LoggingConfig) -> Result<(), TelemetryInitError> {
    TELEMETRY_INIT
        .get_or_init(|| install_subscriber(config))
        .clone()
}

fn install_subscriber(config: LoggingConfig) -> Result<(), TelemetryInitError> {
    let filter = build_env_filter(config.level);

    match config.format {
        LogFormat::Text => init_text_subscriber(filter),
        LogFormat::Json => init_json_subscriber(filter),
    }
}

fn build_env_filter(level: LogLevel) -> EnvFilter {
    let directive = resolve_filter_directive(level, std::env::var(RUST_LOG_ENV).ok().as_deref());
    EnvFilter::try_new(&directive).unwrap_or_else(|_| EnvFilter::new(level.as_filter()))
}

fn resolve_filter_directive(level: LogLevel, rust_log: Option<&str>) -> String {
    rust_log
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| level.as_filter().to_owned())
}

fn init_text_subscriber(filter: EnvFilter) -> Result<(), TelemetryInitError> {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(stderr_supports_ansi())
        .with_target(true)
        .with_thread_names(true)
        .compact()
        .try_init()
        .map_err(|error| TelemetryInitError::new(error.to_string()))
}

fn init_json_subscriber(filter: EnvFilter) -> Result<(), TelemetryInitError> {
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .try_init()
        .map_err(|error| TelemetryInitError::new(error.to_string()))
}

fn stderr_supports_ansi() -> bool {
    std::io::stderr().is_terminal()
}
