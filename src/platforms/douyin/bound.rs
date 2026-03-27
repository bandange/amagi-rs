use crate::client::RequestConfig;

use super::DouyinFetcher;

/// A Douyin fetcher with cookie and request configuration already bound.
pub type BoundDouyinFetcher = DouyinFetcher;

/// Create a Douyin fetcher with a pre-bound cookie and optional request overrides.
#[doc(alias = "createBoundDouyinFetcher")]
pub fn create_bound_douyin_fetcher(
    cookie: impl Into<String>,
    request: Option<RequestConfig>,
) -> BoundDouyinFetcher {
    DouyinFetcher::from_cookie(cookie, request.unwrap_or_default())
}
