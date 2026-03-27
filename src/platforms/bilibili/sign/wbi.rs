use std::collections::BTreeMap;

use reqwest::Url;
use serde::Deserialize;

use crate::error::AppError;
use crate::platforms::internal::md5::md5_hex;
use crate::platforms::internal::random::now_unix_secs;

const MIXIN_KEY_ENC_TAB: [usize; 64] = [
    46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19, 29,
    28, 14, 39, 12, 38, 41, 13, 37, 48, 7, 16, 24, 55, 40, 61, 26, 17, 0, 1, 60, 51, 30, 4, 22, 25,
    54, 21, 56, 59, 6, 63, 57, 62, 11, 36, 20, 34, 44, 52,
];

/// Extracted Bilibili WBI key pair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WbiKeys {
    /// Current image key returned by `/x/web-interface/nav`.
    pub img_key: String,
    /// Current sub key returned by `/x/web-interface/nav`.
    pub sub_key: String,
}

#[derive(Debug, Deserialize)]
struct WbiResponseData {
    data: WbiNavData,
}

#[derive(Debug, Deserialize)]
struct WbiNavData {
    wbi_img: WbiImageData,
}

#[derive(Debug, Deserialize)]
struct WbiImageData {
    img_url: String,
    sub_url: String,
}

/// Derive the 32-byte WBI mixin key from the current img/sub key pair.
pub fn derive_wbi_mixin_key(img_key: &str, sub_key: &str) -> String {
    let merged: Vec<char> = format!("{img_key}{sub_key}").chars().collect();
    MIXIN_KEY_ENC_TAB
        .iter()
        .filter_map(|index| merged.get(*index))
        .take(32)
        .collect()
}

/// Build a deterministic WBI query string for already-normalized parameters.
pub fn build_wbi_query(
    params: &BTreeMap<String, String>,
    img_key: &str,
    sub_key: &str,
    timestamp_secs: u64,
) -> String {
    let mixin_key = derive_wbi_mixin_key(img_key, sub_key);
    let filtered = params
        .iter()
        .map(|(key, value)| {
            (
                percent_encode_component(key),
                percent_encode_component(&sanitize_wbi_value(value)),
            )
        })
        .collect::<Vec<_>>();

    let mut query = filtered
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>();
    query.push(format!("wts={timestamp_secs}"));

    let query_string = query.join("&");
    let digest = md5_hex(format!("{query_string}{mixin_key}").as_bytes());
    format!("{query_string}&w_rid={digest}")
}

/// Build a deterministic WBI query string from an existing request URL.
pub fn build_wbi_query_from_url(
    base_url: &str,
    keys: &WbiKeys,
    timestamp_secs: u64,
) -> Result<String, AppError> {
    let url = Url::parse(base_url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Bilibili WBI URL {base_url}: {error}"))
    })?;
    let params = url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<BTreeMap<_, _>>();

    Ok(build_wbi_query(
        &params,
        &keys.img_key,
        &keys.sub_key,
        timestamp_secs,
    ))
}

/// Parse the Bilibili nav response and extract the current WBI keys.
pub fn extract_wbi_keys_from_nav_body(body: &str) -> Result<WbiKeys, AppError> {
    let response: WbiResponseData = serde_json::from_str(body)?;
    Ok(WbiKeys {
        img_key: file_stem_from_url(&response.data.wbi_img.img_url)?,
        sub_key: file_stem_from_url(&response.data.wbi_img.sub_url)?,
    })
}

/// Fetch the latest Bilibili WBI keys from the nav endpoint.
pub async fn fetch_wbi_keys(cookie: &str) -> Result<WbiKeys, AppError> {
    let body = reqwest::Client::new()
        .get("https://api.bilibili.com/x/web-interface/nav")
        .header(reqwest::header::COOKIE, cookie)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    extract_wbi_keys_from_nav_body(&body)
}

/// Return a TypeScript-compatible WBI query suffix prefixed with `&`.
pub async fn wbi_sign(base_url: &str, cookie: &str) -> Result<String, AppError> {
    let keys = fetch_wbi_keys(cookie).await?;
    let query = build_wbi_query_from_url(base_url, &keys, now_unix_secs())?;
    Ok(format!("&{}", extract_signature_suffix(&query)))
}

/// Append a freshly generated WBI query string to a full request URL.
pub async fn sign_wbi_url(base_url: &str, cookie: &str) -> Result<String, AppError> {
    let query = wbi_sign(base_url, cookie).await?;
    if base_url.contains('?') {
        Ok(format!("{base_url}{query}"))
    } else {
        Ok(format!("{base_url}?{}", query.trim_start_matches('&')))
    }
}

fn file_stem_from_url(url: &str) -> Result<String, AppError> {
    let parsed = Url::parse(url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Bilibili WBI asset URL {url}: {error}"))
    })?;
    let path = parsed.path();
    let start = path.rfind('/').map(|index| index + 1).unwrap_or(0);
    let end = path.rfind('.').unwrap_or(path.len());

    if start >= end {
        return Err(AppError::InvalidRequestConfig(format!(
            "Invalid Bilibili WBI asset path: {path}"
        )));
    }

    Ok(path[start..end].to_owned())
}

fn sanitize_wbi_value(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !matches!(ch, '!' | '\'' | '(' | ')' | '*'))
        .collect()
}

fn percent_encode_component(value: &str) -> String {
    let mut output = String::new();
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => output.push(*byte as char),
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

pub(super) fn extract_signature_suffix(query: &str) -> String {
    query
        .split('&')
        .filter(|segment| segment.starts_with("wts=") || segment.starts_with("w_rid="))
        .collect::<Vec<_>>()
        .join("&")
}
