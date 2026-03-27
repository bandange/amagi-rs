use crate::client::RequestConfig;

use super::KuaishouFetcher;

/// A Kuaishou fetcher with cookie and request configuration already bound.
pub type BoundKuaishouFetcher = KuaishouFetcher;

/// Create a Kuaishou fetcher with a pre-bound cookie and optional request overrides.
#[doc(alias = "createBoundKuaishouFetcher")]
pub fn create_bound_kuaishou_fetcher(
    cookie: impl Into<String>,
    request: Option<RequestConfig>,
) -> BoundKuaishouFetcher {
    KuaishouFetcher::from_cookie(cookie, request.unwrap_or_default())
}
