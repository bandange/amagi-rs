//! Kuaishou fetcher constructors for integration tests.

use amagi::kuaishou::KuaishouFetcher;

use crate::env::TestResult;

/// Build an authenticated Kuaishou fetcher from local test environment files.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env(manifest_dir: impl AsRef<std::path::Path>) -> TestResult<KuaishouFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.kuaishou_fetcher())
}

/// Build a Kuaishou fetcher only when `AMAGI_KUAISHOU_COOKIE` is set.
///
/// Returns `Ok(None)` and prints a skip message when the cookie is unavailable.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env_if_cookie(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<Option<KuaishouFetcher>> {
    let client = crate::client::client_from_env(manifest_dir)?;
    let cookie = client
        .options()
        .cookies
        .kuaishou
        .as_deref()
        .unwrap_or_default();
    if cookie.trim().is_empty() {
        eprintln!("skipped: AMAGI_KUAISHOU_COOKIE is not set");
        return Ok(None);
    }

    Ok(Some(client.kuaishou_fetcher()))
}

/// Build an unauthenticated Kuaishou fetcher with default client options.
#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> KuaishouFetcher {
    crate::client::unauthenticated_client().kuaishou_fetcher()
}
