//! Shared configuration types used by the CLI, runtime, and HTTP server.

mod app;
mod logging;
mod output;
mod tasks;

#[cfg(feature = "server")]
pub use amagi_server::{ServeConfig, ServeRuntimeOverrides};
pub use app::*;
pub use logging::*;
pub use output::*;
pub use tasks::*;
