//! Axum HTTP server and node transport for amagi.

mod config;
pub mod node;
pub mod server;

pub use config::{ServeConfig, ServeRuntimeOverrides};
pub use server::serve;
pub use server::serve_with_ready;
