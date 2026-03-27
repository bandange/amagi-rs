//! Error types shared across crate entrypoints and runtime helpers.

use std::fmt;
use std::io;

use reqwest::StatusCode;

/// Shared error type used by application startup and runtime helpers.
#[derive(Debug)]
pub enum AppError {
    /// An I/O error originating from the standard library or network runtime.
    Io(io::Error),
    /// A JSON serialization or deserialization error.
    #[cfg(feature = "client")]
    Json(serde_json::Error),
    /// An HTTP transport error raised while talking to an upstream platform.
    Http(reqwest::Error),
    /// An invalid runtime request configuration such as malformed headers.
    InvalidRequestConfig(String),
    /// An upstream platform returned a failure response or malformed payload.
    UpstreamResponse {
        /// HTTP status code when the upstream failure included one.
        status: Option<StatusCode>,
        /// Human-readable error detail.
        message: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            #[cfg(feature = "client")]
            Self::Json(error) => write!(f, "{error}"),
            Self::Http(error) => write!(f, "{error}"),
            Self::InvalidRequestConfig(message) => f.write_str(message),
            Self::UpstreamResponse {
                status: Some(status),
                message,
            } => write!(f, "upstream request failed with status {status}: {message}"),
            Self::UpstreamResponse {
                status: None,
                message,
            } => f.write_str(message),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            #[cfg(feature = "client")]
            Self::Json(error) => Some(error),
            Self::Http(error) => Some(error),
            Self::InvalidRequestConfig(_) | Self::UpstreamResponse { .. } => None,
        }
    }
}

impl From<io::Error> for AppError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[cfg(feature = "client")]
impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}
