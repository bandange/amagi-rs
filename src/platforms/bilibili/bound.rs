use crate::client::RequestConfig;

use super::BilibiliFetcher;

/// A Bilibili fetcher with cookie and request configuration already bound.
pub type BoundBilibiliFetcher = BilibiliFetcher;

/// Create a Bilibili fetcher with a pre-bound cookie and optional request overrides.
#[doc(alias = "createBoundBilibiliFetcher")]
pub fn create_bound_bilibili_fetcher(
    cookie: impl Into<String>,
    request: Option<RequestConfig>,
) -> BoundBilibiliFetcher {
    BilibiliFetcher::from_cookie(cookie, request.unwrap_or_default())
}
