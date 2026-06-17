use amagi::xiaohongshu::XiaohongshuFetcher;

use crate::env::TestResult;

#[cfg(feature = "client")]
pub fn fetcher_from_env(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<XiaohongshuFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.xiaohongshu_fetcher())
}

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

#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> XiaohongshuFetcher {
    crate::client::unauthenticated_client().xiaohongshu_fetcher()
}
