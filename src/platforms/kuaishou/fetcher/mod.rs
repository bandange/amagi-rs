use std::borrow::Cow;

use crate::client::{PlatformClient, RequestProfile};

mod emoji;
mod graphql;
mod live;
mod live_api;
mod live_fetch;
mod profile;
mod profile_fetch;
pub(crate) mod requests;
mod support;
mod value;
mod work;


const KUAISHOU_GRAPHQL_ENDPOINT: &str = "https://www.kuaishou.com/graphql";
const KUAISHOU_LIVE_BASE_URL: &str = "https://live.kuaishou.com";
const EMOJI_LIST_OPERATION_NAME: &str = "visionBaseEmoticons";
const EMOJI_LIST_QUERY: &str =
    "query visionBaseEmoticons {\n  visionBaseEmoticons {\n    iconUrls\n    __typename\n  }\n}\n";

/// Rust-native Kuaishou fetcher bound to a resolved client request profile.
#[derive(Debug, Clone)]
pub struct KuaishouFetcher {
    request_profile: RequestProfile,
    graphql_endpoint: Cow<'static, str>,
    live_base_url: Cow<'static, str>,
}

impl KuaishouFetcher {
    /// Create a fetcher from a Kuaishou-scoped [`PlatformClient`].
    pub fn new(client: PlatformClient) -> Self {
        Self {
            request_profile: client.request_profile(),
            graphql_endpoint: Cow::Borrowed(KUAISHOU_GRAPHQL_ENDPOINT),
            live_base_url: Cow::Borrowed(KUAISHOU_LIVE_BASE_URL),
        }
    }

    /// Create a fetcher from a raw Kuaishou cookie and optional request overrides.
    #[doc(alias = "createBoundKuaishouFetcher")]
    pub fn from_cookie(cookie: impl Into<String>, request: crate::client::RequestConfig) -> Self {
        Self::new(PlatformClient {
            platform: crate::catalog::Platform::Kuaishou,
            cookie: Some(cookie.into()),
            request,
        })
    }

    /// Return the resolved request profile bound to this fetcher.
    pub fn request_profile(&self) -> &RequestProfile {
        &self.request_profile
    }
}
