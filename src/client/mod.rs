//! SDK-style client configuration and platform accessors.
//!
//! This module ports the shared client shape from `dev/server/index.ts` into a
//! Rust-native form so the crate can expose one configuration model for SDK,
//! CLI, and web entrypoints.

mod defaults;
mod types;

use std::fmt;

use serde::Serialize;

use crate::catalog::{ApiMethodSpec, Platform, PlatformSpec, method_specs, platform_spec};
use crate::events::EventBus;
use defaults::{platform_default_headers, platform_default_method};

pub use types::{ClientOptions, CookieConfig, RequestConfig, RequestProfile};

/// Shared client entrypoint for SDK consumers.
#[derive(Clone)]
pub struct AmagiClient {
    options: ClientOptions,
    events: EventBus,
}

impl fmt::Debug for AmagiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AmagiClient")
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl Default for AmagiClient {
    fn default() -> Self {
        Self::new(ClientOptions::default())
    }
}

impl AmagiClient {
    /// Create a new client from the provided options.
    pub fn new(options: ClientOptions) -> Self {
        Self {
            options,
            events: EventBus::default(),
        }
    }

    /// Create a new client from process environment variables and `.env`.
    ///
    /// # Errors
    ///
    /// Returns an error when `.env` cannot be read or contains invalid values.
    pub fn from_env() -> Result<Self, crate::error::AppError> {
        Ok(Self::new(ClientOptions::from_env()?))
    }

    /// Return the shared event bus.
    pub fn events(&self) -> &EventBus {
        &self.events
    }

    /// Return the client options used to build this instance.
    pub fn options(&self) -> &ClientOptions {
        &self.options
    }

    /// Return a platform-specific view bound to the current options.
    pub fn platform(&self, platform: Platform) -> PlatformClient {
        PlatformClient {
            platform,
            cookie: self
                .options
                .cookies
                .for_platform(platform)
                .map(str::to_owned),
            request: self.options.request.clone(),
        }
    }

    /// Return a platform-specific view for Bilibili.
    pub fn bilibili(&self) -> PlatformClient {
        self.platform(Platform::Bilibili)
    }

    /// Return the Rust-native Bilibili fetcher bound to the current options.
    pub fn bilibili_fetcher(&self) -> crate::platforms::bilibili::BilibiliFetcher {
        crate::platforms::bilibili::BilibiliFetcher::new(self.bilibili())
    }

    /// Return a platform-specific view for Douyin.
    pub fn douyin(&self) -> PlatformClient {
        self.platform(Platform::Douyin)
    }

    /// Return a platform-specific view for Kuaishou.
    pub fn kuaishou(&self) -> PlatformClient {
        self.platform(Platform::Kuaishou)
    }

    /// Return a platform-specific view for Xiaohongshu.
    pub fn xiaohongshu(&self) -> PlatformClient {
        self.platform(Platform::Xiaohongshu)
    }

    /// Return a platform-specific view for Twitter/X.
    pub fn twitter(&self) -> PlatformClient {
        self.platform(Platform::Twitter)
    }

    /// Return the Rust-native Xiaohongshu fetcher bound to the current options.
    pub fn xiaohongshu_fetcher(&self) -> crate::platforms::xiaohongshu::XiaohongshuFetcher {
        crate::platforms::xiaohongshu::XiaohongshuFetcher::new(self.xiaohongshu())
    }

    /// Return the Rust-native Douyin fetcher bound to the current options.
    pub fn douyin_fetcher(&self) -> crate::platforms::douyin::DouyinFetcher {
        crate::platforms::douyin::DouyinFetcher::new(self.douyin())
    }

    /// Return the Rust-native Kuaishou fetcher bound to the current options.
    pub fn kuaishou_fetcher(&self) -> crate::platforms::kuaishou::KuaishouFetcher {
        crate::platforms::kuaishou::KuaishouFetcher::new(self.kuaishou())
    }

    /// Return the Rust-native Twitter/X fetcher bound to the current options.
    pub fn twitter_fetcher(&self) -> crate::platforms::twitter::TwitterFetcher {
        crate::platforms::twitter::TwitterFetcher::new(self.twitter())
    }

    /// Return the full API catalog exposed by the client.
    pub fn catalog(&self) -> [PlatformSpec; 5] {
        Platform::ALL.map(|platform| self.platform(platform).spec())
    }
}

/// Convenience constructor mirroring the original TypeScript `createAmagiClient`.
pub fn create_amagi_client(options: ClientOptions) -> AmagiClient {
    AmagiClient::new(options)
}

/// Platform-specific client view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlatformClient {
    /// Platform this client view targets.
    pub platform: Platform,
    /// Bound cookie for the platform when configured.
    pub cookie: Option<String>,
    /// Shared request overrides carried into the platform adapter.
    pub request: RequestConfig,
}

impl PlatformClient {
    /// Return whether the platform view has a bound cookie.
    pub fn has_cookie(&self) -> bool {
        self.cookie
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    }

    /// Return the published API base path for this platform.
    pub fn api_base_path(&self) -> &'static str {
        self.platform.api_base_path()
    }

    /// Return the static API catalog for this platform.
    pub fn spec(&self) -> PlatformSpec {
        platform_spec(self.platform)
    }

    /// Return every published method for this platform.
    pub fn methods(&self) -> &'static [ApiMethodSpec] {
        method_specs(self.platform)
    }

    /// Build the effective request profile by merging defaults and overrides.
    pub fn request_profile(&self) -> RequestProfile {
        let mut headers = platform_default_headers(self.platform);
        let cookie_header = match self.platform {
            Platform::Xiaohongshu => "cookie",
            _ => "Cookie",
        };

        headers.insert(
            cookie_header.to_owned(),
            self.cookie.clone().unwrap_or_default(),
        );
        headers.extend(self.request.headers.clone());

        RequestProfile {
            platform: self.platform,
            method: platform_default_method(self.platform),
            timeout_ms: self.request.timeout_ms,
            max_retries: self.request.max_retries,
            headers,
        }
    }
}

