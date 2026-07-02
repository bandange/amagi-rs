//! SDK-style client configuration and platform accessors.
//!
//! This crate exposes the shared SDK entrypoint while platform-specific
//! fetchers live in `amagi-adapters`.
#![warn(missing_docs)]

pub mod events;

use std::fmt;

use amagi_adapters::spec;
pub use amagi_adapters::{AdapterContext, PlatformClient};
use amagi_core::{AppError, Platform};
pub use amagi_core::{ClientOptions, CookieConfig, RequestConfig, RequestProfile};

pub use events::{
    AmagiEvent, AmagiEventType, ApiErrorEventData, ApiSuccessEventData, EventBus, EventLogLevel,
    HttpRequestEventData, HttpResponseEventData, LogEventData, NetworkErrorEventData,
    NetworkRetryEventData,
};

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
    pub fn from_env() -> Result<Self, AppError> {
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
    pub fn platform(&self, platform: Platform) -> AdapterContext {
        AdapterContext {
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
    pub fn bilibili(&self) -> AdapterContext {
        self.platform(Platform::Bilibili)
    }

    /// Return the Rust-native Bilibili fetcher bound to the current options.
    pub fn bilibili_fetcher(&self) -> amagi_adapters::bilibili::BilibiliFetcher {
        amagi_adapters::bilibili::BilibiliFetcher::new(self.bilibili())
    }

    /// Return a platform-specific view for Douyin.
    pub fn douyin(&self) -> AdapterContext {
        self.platform(Platform::Douyin)
    }

    /// Return the Rust-native Douyin fetcher bound to the current options.
    pub fn douyin_fetcher(&self) -> amagi_adapters::douyin::DouyinFetcher {
        amagi_adapters::douyin::DouyinFetcher::new(self.douyin())
    }

    /// Return a platform-specific view for Kuaishou.
    pub fn kuaishou(&self) -> AdapterContext {
        self.platform(Platform::Kuaishou)
    }

    /// Return the Rust-native Kuaishou fetcher bound to the current options.
    pub fn kuaishou_fetcher(&self) -> amagi_adapters::kuaishou::KuaishouFetcher {
        amagi_adapters::kuaishou::KuaishouFetcher::new(self.kuaishou())
    }

    /// Return a platform-specific view for Xiaohongshu.
    pub fn xiaohongshu(&self) -> AdapterContext {
        self.platform(Platform::Xiaohongshu)
    }

    /// Return the Rust-native Xiaohongshu fetcher bound to the current options.
    pub fn xiaohongshu_fetcher(&self) -> amagi_adapters::xiaohongshu::XiaohongshuFetcher {
        amagi_adapters::xiaohongshu::XiaohongshuFetcher::new(self.xiaohongshu())
    }

    /// Return a platform-specific view for Twitter/X.
    pub fn twitter(&self) -> AdapterContext {
        self.platform(Platform::Twitter)
    }

    /// Return the Rust-native Twitter/X fetcher bound to the current options.
    pub fn twitter_fetcher(&self) -> amagi_adapters::twitter::TwitterFetcher {
        amagi_adapters::twitter::TwitterFetcher::new(self.twitter())
    }

    /// Compatibility wrapper for the former catalog-oriented API name.
    pub fn catalog(&self) -> [amagi_core::PlatformApiSpec; 5] {
        self.api_specs()
    }

    /// Return the full API metadata exposed by the client.
    pub fn api_specs(&self) -> [amagi_core::PlatformApiSpec; 5] {
        spec::all_platform_api_specs()
    }
}

/// Convenience constructor mirroring the original TypeScript `createAmagiClient`.
pub fn create_amagi_client(options: ClientOptions) -> AmagiClient {
    AmagiClient::new(options)
}
