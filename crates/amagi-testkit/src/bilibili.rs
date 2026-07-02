//! Bilibili fetcher constructors for integration tests.

use amagi::bilibili::BilibiliFetcher;

use crate::env::TestResult;

/// Build an authenticated Bilibili fetcher from local test environment files.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env(manifest_dir: impl AsRef<std::path::Path>) -> TestResult<BilibiliFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.bilibili_fetcher())
}

/// Build a Bilibili fetcher only when `AMAGI_BILIBILI_COOKIE` is set.
///
/// Returns `Ok(None)` and prints a skip message when the cookie is unavailable.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env_if_cookie(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<Option<BilibiliFetcher>> {
    let client = crate::client::client_from_env(manifest_dir)?;
    let cookie = client
        .options()
        .cookies
        .bilibili
        .as_deref()
        .unwrap_or_default();
    if cookie.trim().is_empty() {
        eprintln!("skipped: AMAGI_BILIBILI_COOKIE is not set");
        return Ok(None);
    }

    Ok(Some(client.bilibili_fetcher()))
}

/// Build an unauthenticated Bilibili fetcher with default client options.
#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> BilibiliFetcher {
    crate::client::unauthenticated_client().bilibili_fetcher()
}
