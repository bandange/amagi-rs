use serde::Serialize;

use crate::catalog::Platform;

/// Log levels emitted through the shared event bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum EventLogLevel {
    /// Informational output.
    Info,
    /// Warning output.
    Warn,
    /// Error output.
    Error,
    /// Debug output.
    Debug,
    /// Highlighted lifecycle markers.
    Mark,
}

/// Stable event categories exposed by the bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AmagiEventType {
    /// `log:info`
    LogInfo,
    /// `log:warn`
    LogWarn,
    /// `log:error`
    LogError,
    /// `log:debug`
    LogDebug,
    /// `log:mark`
    LogMark,
    /// `http:request`
    HttpRequest,
    /// `http:response`
    HttpResponse,
    /// `http:error`
    HttpError,
    /// `network:retry`
    NetworkRetry,
    /// `network:error`
    NetworkError,
    /// `api:success`
    ApiSuccess,
    /// `api:error`
    ApiError,
}

/// Shared payload for log events.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LogEventData {
    /// Log level emitted by the runtime.
    pub level: EventLogLevel,
    /// Primary log message.
    pub message: String,
    /// Additional structured arguments already rendered into strings.
    pub args: Vec<String>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for outgoing HTTP requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HttpRequestEventData {
    /// Request method.
    pub method: String,
    /// Absolute or relative request URL.
    pub url: String,
    /// Request headers after normalization.
    pub headers: Vec<(String, String)>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for completed HTTP responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HttpResponseEventData {
    /// Request method.
    pub method: String,
    /// Absolute or relative request URL.
    pub url: String,
    /// HTTP response status code.
    pub status_code: u16,
    /// Observed response time in milliseconds.
    pub response_time_ms: u64,
    /// Optional client IP.
    pub client_ip: Option<String>,
    /// Optional request body size.
    pub request_size: Option<String>,
    /// Optional response body size.
    pub response_size: Option<String>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for retry attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetworkRetryEventData {
    /// Network error code.
    pub error_code: String,
    /// Current retry attempt number.
    pub attempt: u32,
    /// Configured retry ceiling.
    pub max_retries: u32,
    /// Delay before the next attempt.
    pub delay_ms: u64,
    /// Request URL when known.
    pub url: Option<String>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for network failures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NetworkErrorEventData {
    /// Network error code.
    pub error_code: String,
    /// Error message.
    pub message: String,
    /// Number of retries already attempted.
    pub retries: u32,
    /// Request URL when known.
    pub url: Option<String>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for successful API calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiSuccessEventData {
    /// Platform the API belongs to.
    pub platform: Platform,
    /// Stable method key used for the call.
    pub method_key: String,
    /// HTTP status code observed by the caller.
    pub status_code: u16,
    /// End-to-end duration in milliseconds.
    pub duration_ms: u64,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Shared payload for failed API calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ApiErrorEventData {
    /// Platform the API belongs to.
    pub platform: Platform,
    /// Stable method key used for the call.
    pub method_key: String,
    /// Optional platform or HTTP error code.
    pub error_code: Option<String>,
    /// Human-readable error message.
    pub error_message: String,
    /// Request URL when known.
    pub url: Option<String>,
    /// End-to-end duration in milliseconds when measured.
    pub duration_ms: Option<u64>,
    /// Unix timestamp in milliseconds.
    pub timestamp_ms: u64,
}

/// Any event emitted by the runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "kebab-case")]
pub enum AmagiEvent {
    /// Informational log event.
    LogInfo(LogEventData),
    /// Warning log event.
    LogWarn(LogEventData),
    /// Error log event.
    LogError(LogEventData),
    /// Debug log event.
    LogDebug(LogEventData),
    /// Highlight lifecycle event.
    LogMark(LogEventData),
    /// Outgoing HTTP request event.
    HttpRequest(HttpRequestEventData),
    /// HTTP response event.
    HttpResponse(HttpResponseEventData),
    /// HTTP error event.
    HttpError(NetworkErrorEventData),
    /// Retry attempt event.
    NetworkRetry(NetworkRetryEventData),
    /// Network error event.
    NetworkError(NetworkErrorEventData),
    /// API success event.
    ApiSuccess(ApiSuccessEventData),
    /// API error event.
    ApiError(ApiErrorEventData),
}

impl AmagiEvent {
    /// Return the stable event category for this payload.
    pub const fn event_type(&self) -> AmagiEventType {
        match self {
            Self::LogInfo(_) => AmagiEventType::LogInfo,
            Self::LogWarn(_) => AmagiEventType::LogWarn,
            Self::LogError(_) => AmagiEventType::LogError,
            Self::LogDebug(_) => AmagiEventType::LogDebug,
            Self::LogMark(_) => AmagiEventType::LogMark,
            Self::HttpRequest(_) => AmagiEventType::HttpRequest,
            Self::HttpResponse(_) => AmagiEventType::HttpResponse,
            Self::HttpError(_) => AmagiEventType::HttpError,
            Self::NetworkRetry(_) => AmagiEventType::NetworkRetry,
            Self::NetworkError(_) => AmagiEventType::NetworkError,
            Self::ApiSuccess(_) => AmagiEventType::ApiSuccess,
            Self::ApiError(_) => AmagiEventType::ApiError,
        }
    }
}
