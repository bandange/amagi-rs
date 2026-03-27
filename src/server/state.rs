//! Shared web application state.

use crate::client::AmagiClient;
use crate::config::ServeConfig;

/// Shared state injected into web handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Crate name used in responses.
    pub app_name: &'static str,
    /// Crate version used in responses.
    pub version: &'static str,
    /// Bound web serving configuration.
    pub serve: ServeConfig,
    /// Shared SDK-style client carrying catalog and request defaults.
    pub client: AmagiClient,
}

impl AppState {
    /// Create a new application state container.
    pub fn new(
        app_name: &'static str,
        version: &'static str,
        serve: ServeConfig,
        client: AmagiClient,
    ) -> Self {
        Self {
            app_name,
            version,
            serve,
            client,
        }
    }
}
