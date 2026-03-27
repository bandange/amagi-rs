#[cfg(feature = "cli")]
use clap::ValueEnum;
use serde::Serialize;

/// Output settings used by [`crate::output::Printer`].
#[cfg(any(feature = "cli", feature = "server"))]
#[derive(Debug, Clone)]
pub struct OutputConfig {
    /// Output format used by CLI-facing messages.
    pub format: OutputFormat,
    /// Optional path used to persist CLI-facing output.
    pub file: Option<String>,
    /// Whether JSON output should use pretty formatting.
    pub pretty: bool,
    /// Whether file output should append instead of truncating the target file.
    pub append: bool,
    /// Whether parent directories for the output file should be created automatically.
    pub create_parent_dirs: bool,
}

/// Output format used for CLI-facing stdout messages.
#[cfg(any(feature = "cli", feature = "server"))]
#[cfg_attr(feature = "cli", derive(ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    /// Human-readable plain text.
    Text,
    /// Machine-readable JSON lines.
    Json,
}

#[cfg(any(feature = "cli", feature = "server"))]
impl Default for OutputFormat {
    fn default() -> Self {
        Self::Text
    }
}
