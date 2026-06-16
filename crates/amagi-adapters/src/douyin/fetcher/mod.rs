use crate::AdapterContext;
use amagi_core::RequestProfile;

mod content;
mod payload;
pub(crate) mod requests;
mod search;
mod transport;

use super::sign::generate_verify_fp;

const DANMAKU_SEGMENT_MS: u64 = 32_000;

#[derive(Debug, Clone, Copy)]
enum DouyinSignType {
    None,
    ABogus,
    XBogus,
}

/// Rust-native Douyin fetcher backed by the migrated signing algorithms.
#[derive(Debug, Clone)]
pub struct DouyinFetcher {
    request_profile: RequestProfile,
    verify_fp: String,
    endpoints: requests::DouyinApiEndpoints,
}

impl DouyinFetcher {
    /// Create a fetcher from a Douyin-scoped [`AdapterContext`].
    pub fn new(client: AdapterContext) -> Self {
        Self {
            request_profile: client.request_profile(),
            verify_fp: generate_verify_fp(),
            endpoints: requests::DouyinApiEndpoints::default(),
        }
    }

    /// Create a fetcher from a raw Douyin cookie and optional request overrides.
    #[doc(alias = "createBoundDouyinFetcher")]
    pub fn from_cookie(cookie: impl Into<String>, request: amagi_core::RequestConfig) -> Self {
        Self::new(AdapterContext {
            platform: amagi_core::Platform::Douyin,
            cookie: Some(cookie.into()),
            request,
        })
    }

    /// Return the resolved request profile bound to this fetcher.
    pub fn request_profile(&self) -> &RequestProfile {
        &self.request_profile
    }
}
