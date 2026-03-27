use crate::client::RequestConfig;

use super::TwitterFetcher;

/// A Twitter fetcher with request configuration already bound.
pub type BoundTwitterFetcher = TwitterFetcher;

/// Create a Twitter fetcher with an optional cookie and request overrides.
#[doc(alias = "createBoundTwitterFetcher")]
pub fn create_bound_twitter_fetcher(
    cookie: impl Into<String>,
    request: Option<RequestConfig>,
) -> BoundTwitterFetcher {
    TwitterFetcher::from_cookie(cookie, request.unwrap_or_default())
}
