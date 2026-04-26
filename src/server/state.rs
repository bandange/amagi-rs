//! Shared web application state.

use crate::client::AmagiClient;
use crate::config::ServeConfig;
use crate::error::AppError;

use super::runtime::{PlatformServeMode, ServerRuntimeConfig};
use crate::catalog::Platform;

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
    /// Node-aware runtime routing configuration.
    pub runtime: ServerRuntimeConfig,
    /// Shared HTTP client used for node-to-node proxying.
    pub proxy_client: reqwest::Client,
}

impl AppState {
    /// Create a new application state container.
    ///
    /// # Errors
    ///
    /// Returns an error if the shared proxy client cannot be initialized.
    pub fn new(
        app_name: &'static str,
        version: &'static str,
        serve: ServeConfig,
        client: AmagiClient,
        runtime: ServerRuntimeConfig,
    ) -> Result<Self, AppError> {
        let proxy_client = runtime.build_proxy_client()?;

        Ok(Self {
            app_name,
            version,
            serve,
            client,
            runtime,
            proxy_client,
        })
    }

    /// Return the serving mode for a platform.
    pub fn platform_mode(&self, platform: Platform) -> PlatformServeMode {
        self.runtime.platform_mode(platform)
    }

    /// Return whether the platform is published by the current node.
    pub fn is_platform_published(&self, platform: Platform) -> bool {
        self.runtime.is_platform_published(platform)
    }

    /// Return the configured upstream base URL for a platform.
    pub fn platform_upstream(&self, platform: Platform) -> Option<&str> {
        self.runtime.platform_upstream(platform)
    }

    /// Return the maximum allowed proxy hop count.
    pub const fn proxy_max_hops(&self) -> u32 {
        self.runtime.proxy_max_hops()
    }
}
