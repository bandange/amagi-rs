use reqwest::Client;
use serde_json::Value;

use crate::{error::AppError, platforms::internal::random::now_unix_secs};

use super::super::{WbiKeys, build_wbi_query_from_url, extract_wbi_keys_from_nav_body};
use super::{
    types::{BilibiliPlayurlQuery, BilibiliPlayurlStatus},
    urls::BilibiliApiUrls,
};

/// Build the playback query suffix used by Bilibili video and bangumi playurl requests.
///
/// This function mirrors the original TypeScript `qtparam` utility while
/// returning a typed Rust result.
#[doc(alias = "qtparam")]
pub async fn build_playurl_query(
    base_url: &str,
    cookie: &str,
) -> Result<BilibiliPlayurlQuery, AppError> {
    let cookie = cookie.trim();

    if cookie.is_empty() {
        return Ok(BilibiliPlayurlQuery {
            query_suffix: "platform=html5".to_owned(),
            status: BilibiliPlayurlStatus::Guest,
        });
    }

    let nav_body = fetch_nav_body(cookie).await?;
    build_playurl_query_from_nav_body(base_url, &nav_body)
}

/// Backwards-compatible alias for [`build_playurl_query`].
pub async fn qtparam(base_url: &str, cookie: &str) -> Result<BilibiliPlayurlQuery, AppError> {
    build_playurl_query(base_url, cookie).await
}

/// Build a Bilibili playback query suffix from a previously fetched nav body.
pub fn build_playurl_query_from_nav_body(
    base_url: &str,
    nav_body: &str,
) -> Result<BilibiliPlayurlQuery, AppError> {
    let keys = extract_wbi_keys_from_nav_body(nav_body)?;
    build_playurl_query_from_nav_body_with_keys(base_url, nav_body, &keys)
}

fn build_playurl_query_from_nav_body_with_keys(
    base_url: &str,
    nav_body: &str,
    keys: &WbiKeys,
) -> Result<BilibiliPlayurlQuery, AppError> {
    let signed_query = build_wbi_query_from_url(base_url, keys, now_unix_secs())?;
    let nav_value: Value = serde_json::from_str(nav_body)?;
    let is_vip = nav_value
        .get("data")
        .and_then(|value| value.get("vipStatus"))
        .and_then(Value::as_i64)
        == Some(1);

    let mut segments = if is_vip {
        vec!["fnval=4048".to_owned(), "fourk=1".to_owned()]
    } else {
        vec!["qn=64".to_owned(), "fnval=16".to_owned()]
    };
    segments.push(extract_signature_suffix_from_query(&signed_query));

    Ok(BilibiliPlayurlQuery {
        query_suffix: segments
            .into_iter()
            .filter(|segment| !segment.is_empty())
            .collect::<Vec<_>>()
            .join("&"),
        status: BilibiliPlayurlStatus::LoggedIn { is_vip },
    })
}

async fn fetch_nav_body(cookie: &str) -> Result<String, AppError> {
    let url = BilibiliApiUrls::new().login_status()?;
    let body = Client::new()
        .get(url)
        .header("Cookie", cookie)
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}

fn extract_signature_suffix_from_query(query: &str) -> String {
    query
        .split('&')
        .filter(|segment| segment.starts_with("wts=") || segment.starts_with("w_rid="))
        .collect::<Vec<_>>()
        .join("&")
}
