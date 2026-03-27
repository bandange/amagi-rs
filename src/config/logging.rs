use clap::ValueEnum;
use serde::Serialize;

/// Logging settings used by [`crate::telemetry::init`].
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Copy)]
pub struct LoggingConfig {
    /// Minimum level accepted by the logger when no environment override is set.
    pub level: LogLevel,
    /// Renderer used by the logger.
    pub format: LogFormat,
}

/// Format used by the structured logger.
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    /// Human-readable plain text logs.
    Text,
    /// JSON logs suitable for log pipelines.
    Json,
}

#[cfg(feature = "cli")]
impl Default for LogFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// Logging level used by the runtime.
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Only error events.
    Error,
    /// Warning and error events.
    Warn,
    /// Informational, warning, and error events.
    Info,
    /// Debug and above.
    Debug,
    /// Verbose trace logging.
    Trace,
}

#[cfg(feature = "cli")]
impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[cfg(feature = "cli")]
impl LogLevel {
    /// Return the `tracing_subscriber` filter directive for this level.
    pub fn as_filter(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}
