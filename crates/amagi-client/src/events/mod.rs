//! Typed event bus shared across SDK, CLI, and web flows.
//!
//! The TypeScript source tree exposes a global event emitter. This Rust port
//! keeps the same cross-cutting event categories, but models them as a typed
//! broadcast bus so multiple consumers can subscribe concurrently.

mod bus;
mod payloads;

pub use bus::EventBus;
pub use payloads::{
    AmagiEvent, AmagiEventType, ApiErrorEventData, ApiSuccessEventData, EventLogLevel,
    HttpRequestEventData, HttpResponseEventData, LogEventData, NetworkErrorEventData,
    NetworkRetryEventData,
};
