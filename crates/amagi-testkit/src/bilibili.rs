use amagi::bilibili::BilibiliFetcher;

use crate::env::TestResult;

#[cfg(feature = "client")]
pub fn fetcher_from_env(manifest_dir: impl AsRef<std::path::Path>) -> TestResult<BilibiliFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.bilibili_fetcher())
}

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

#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> BilibiliFetcher {
    crate::client::unauthenticated_client().bilibili_fetcher()
}
