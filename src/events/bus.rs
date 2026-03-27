use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::broadcast;

use crate::catalog::Platform;

use super::payloads::{
    AmagiEvent, ApiErrorEventData, ApiSuccessEventData, EventLogLevel, HttpRequestEventData,
    HttpResponseEventData, LogEventData, NetworkErrorEventData, NetworkRetryEventData,
};

/// Broadcast-based event bus.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<AmagiEvent>,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus").finish_non_exhaustive()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new(256)
    }
}

impl EventBus {
    /// Create an event bus with a custom broadcast buffer size.
    pub fn new(buffer: usize) -> Self {
        let (sender, _) = broadcast::channel(buffer);
        Self { sender }
    }

    /// Subscribe to all future events.
    pub fn subscribe(&self) -> broadcast::Receiver<AmagiEvent> {
        self.sender.subscribe()
    }

    /// Emit an arbitrary event.
    pub fn emit(&self, event: AmagiEvent) {
        let _ = self.sender.send(event);
    }

    /// Emit a log event at the requested level.
    pub fn emit_log<S, I>(&self, level: EventLogLevel, message: S, args: I)
    where
        S: Into<String>,
        I: IntoIterator,
        I::Item: Into<String>,
    {
        let data = LogEventData {
            level,
            message: message.into(),
            args: args.into_iter().map(Into::into).collect(),
            timestamp_ms: now_timestamp_ms(),
        };

        let event = match level {
            EventLogLevel::Info => AmagiEvent::LogInfo(data),
            EventLogLevel::Warn => AmagiEvent::LogWarn(data),
            EventLogLevel::Error => AmagiEvent::LogError(data),
            EventLogLevel::Debug => AmagiEvent::LogDebug(data),
            EventLogLevel::Mark => AmagiEvent::LogMark(data),
        };

        self.emit(event);
    }

    /// Emit an outgoing HTTP request event.
    pub fn emit_http_request(&self, method: impl Into<String>, url: impl Into<String>) {
        self.emit(AmagiEvent::HttpRequest(HttpRequestEventData {
            method: method.into(),
            url: url.into(),
            headers: Vec::new(),
            timestamp_ms: now_timestamp_ms(),
        }));
    }

    /// Emit a completed HTTP response event.
    pub fn emit_http_response(
        &self,
        method: impl Into<String>,
        url: impl Into<String>,
        status_code: u16,
        response_time_ms: u64,
    ) {
        self.emit(AmagiEvent::HttpResponse(HttpResponseEventData {
            method: method.into(),
            url: url.into(),
            status_code,
            response_time_ms,
            client_ip: None,
            request_size: None,
            response_size: None,
            timestamp_ms: now_timestamp_ms(),
        }));
    }

    /// Emit a retry attempt event.
    pub fn emit_network_retry(
        &self,
        error_code: impl Into<String>,
        attempt: u32,
        max_retries: u32,
        delay_ms: u64,
        url: Option<String>,
    ) {
        self.emit(AmagiEvent::NetworkRetry(NetworkRetryEventData {
            error_code: error_code.into(),
            attempt,
            max_retries,
            delay_ms,
            url,
            timestamp_ms: now_timestamp_ms(),
        }));
    }

    /// Emit a network error event.
    pub fn emit_network_error(
        &self,
        error_code: impl Into<String>,
        message: impl Into<String>,
        retries: u32,
        url: Option<String>,
    ) {
        self.emit(AmagiEvent::NetworkError(NetworkErrorEventData {
            error_code: error_code.into(),
            message: message.into(),
            retries,
            url,
            timestamp_ms: now_timestamp_ms(),
        }));
    }

    /// Emit an API success event.
    pub fn emit_api_success(
        &self,
        platform: Platform,
        method_key: impl Into<String>,
        status_code: u16,
        duration_ms: u64,
    ) {
        self.emit(AmagiEvent::ApiSuccess(ApiSuccessEventData {
            platform,
            method_key: method_key.into(),
            status_code,
            duration_ms,
            timestamp_ms: now_timestamp_ms(),
        }));
    }

    /// Emit an API error event.
    pub fn emit_api_error(
        &self,
        platform: Platform,
        method_key: impl Into<String>,
        error_code: Option<String>,
        error_message: impl Into<String>,
        url: Option<String>,
        duration_ms: Option<u64>,
    ) {
        self.emit(AmagiEvent::ApiError(ApiErrorEventData {
            platform,
            method_key: method_key.into(),
            error_code,
            error_message: error_message.into(),
            url,
            duration_ms,
            timestamp_ms: now_timestamp_ms(),
        }));
    }
}

fn now_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
