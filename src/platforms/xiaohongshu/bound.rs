use crate::client::RequestConfig;

use super::XiaohongshuFetcher;

/// A Xiaohongshu fetcher with cookie and request configuration already bound.
pub type BoundXiaohongshuFetcher = XiaohongshuFetcher;

/// Create a Xiaohongshu fetcher with a pre-bound cookie and optional request overrides.
#[doc(alias = "createBoundXiaohongshuFetcher")]
pub fn create_bound_xiaohongshu_fetcher(
    cookie: impl Into<String>,
    request: Option<RequestConfig>,
) -> BoundXiaohongshuFetcher {
    XiaohongshuFetcher::from_cookie(cookie, request.unwrap_or_default())
}
