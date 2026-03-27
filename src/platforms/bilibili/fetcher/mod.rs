use std::borrow::Cow;

use crate::client::{PlatformClient, RequestProfile};

mod auth;
mod comments;
mod content;
mod helper;
pub(crate) mod requests;
mod transport;
mod user;


const BILIBILI_API_BASE_URL: &str = "https://api.bilibili.com";
const BILIBILI_VC_BASE_URL: &str = "https://api.vc.bilibili.com";
const BILIBILI_LIVE_BASE_URL: &str = "https://api.live.bilibili.com";
const BILIBILI_PASSPORT_BASE_URL: &str = "https://passport.bilibili.com";

/// Rust-native Bilibili fetcher backed by migrated signing algorithms.
#[derive(Debug, Clone)]
pub struct BilibiliFetcher {
    request_profile: RequestProfile,
    api_base_url: Cow<'static, str>,
    vc_base_url: Cow<'static, str>,
    live_base_url: Cow<'static, str>,
    passport_base_url: Cow<'static, str>,
}

impl BilibiliFetcher {
    /// Create a fetcher from a Bilibili-scoped [`PlatformClient`].
    pub fn new(client: PlatformClient) -> Self {
        Self {
            request_profile: client.request_profile(),
            api_base_url: Cow::Borrowed(BILIBILI_API_BASE_URL),
            vc_base_url: Cow::Borrowed(BILIBILI_VC_BASE_URL),
            live_base_url: Cow::Borrowed(BILIBILI_LIVE_BASE_URL),
            passport_base_url: Cow::Borrowed(BILIBILI_PASSPORT_BASE_URL),
        }
    }

    /// Create a fetcher from a raw Bilibili cookie and optional request overrides.
    #[doc(alias = "createBoundBilibiliFetcher")]
    pub fn from_cookie(cookie: impl Into<String>, request: crate::client::RequestConfig) -> Self {
        Self::new(PlatformClient {
            platform: crate::catalog::Platform::Bilibili,
            cookie: Some(cookie.into()),
            request,
        })
    }

    /// Return the resolved request profile bound to this fetcher.
    pub fn request_profile(&self) -> &RequestProfile {
        &self.request_profile
    }
}
