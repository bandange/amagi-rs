//! Xiaohongshu fetcher constructors for integration tests.

use amagi::xiaohongshu::XiaohongshuFetcher;

use crate::env::TestResult;

/// Build an authenticated Xiaohongshu fetcher from local test environment files.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<XiaohongshuFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.xiaohongshu_fetcher())
}

/// Build a Xiaohongshu fetcher only when `AMAGI_XIAOHONGSHU_COOKIE` is set.
///
/// Returns `Ok(None)` and prints a skip message when the cookie is unavailable.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env_if_cookie(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<Option<XiaohongshuFetcher>> {
    let client = crate::client::client_from_env(manifest_dir)?;
    let cookie = client
        .options()
        .cookies
        .xiaohongshu
        .as_deref()
        .unwrap_or_default();
    if cookie.trim().is_empty() {
        eprintln!("skipped: AMAGI_XIAOHONGSHU_COOKIE is not set");
        return Ok(None);
    }

    Ok(Some(client.xiaohongshu_fetcher()))
}

/// Build an unauthenticated Xiaohongshu fetcher with default client options.
#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> XiaohongshuFetcher {
    crate::client::unauthenticated_client().xiaohongshu_fetcher()
}
