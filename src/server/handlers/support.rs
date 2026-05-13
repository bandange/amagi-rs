use axum::{
    Json,
    http::{HeaderMap, StatusCode},
};

use super::super::state::AppState;
use super::{CatalogErrorResponse, FetchErrorResponse};
use crate::catalog::Platform;
use crate::error::AppError;
use crate::platforms::bilibili::BilibiliFetcher;
use crate::platforms::douyin::DouyinFetcher;
use crate::platforms::kuaishou::KuaishouFetcher;
use crate::platforms::twitter::TwitterFetcher;
use crate::platforms::xiaohongshu::XiaohongshuFetcher;

pub(super) type CatalogResult<T> = Result<Json<T>, (StatusCode, Json<CatalogErrorResponse>)>;
pub(super) type FetchResult<T> = Result<Json<T>, (StatusCode, Json<FetchErrorResponse>)>;

const GENERIC_COOKIE_HEADER: &str = "x-amagi-cookie";
const BILIBILI_COOKIE_HEADER: &str = "x-amagi-bilibili-cookie";
const DOUYIN_COOKIE_HEADER: &str = "x-amagi-douyin-cookie";
const KUAISHOU_COOKIE_HEADER: &str = "x-amagi-kuaishou-cookie";
const TWITTER_COOKIE_HEADER: &str = "x-amagi-twitter-cookie";
const XIAOHONGSHU_COOKIE_HEADER: &str = "x-amagi-xiaohongshu-cookie";

enum RequestCookieOverride {
    UseConfigured,
    Override(String),
    Clear,
}

pub(super) fn fetch_error_response(error: AppError) -> (StatusCode, Json<FetchErrorResponse>) {
    let status = match &error {
        AppError::Io(_) | AppError::InvalidRequestConfig(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::Json(_)
        | AppError::Http(_)
        | AppError::UpstreamResponse { .. }
        | AppError::UpstreamReconnect { .. } => StatusCode::BAD_GATEWAY,
    };

    (
        status,
        Json(FetchErrorResponse {
            error: "fetch_failed",
            detail: error.to_string(),
        }),
    )
}

pub(super) fn bilibili_fetcher(state: &AppState, headers: &HeaderMap) -> BilibiliFetcher {
    BilibiliFetcher::new(platform_client_for_request(
        state,
        Platform::Bilibili,
        headers,
    ))
}

pub(super) fn douyin_fetcher(state: &AppState, headers: &HeaderMap) -> DouyinFetcher {
    DouyinFetcher::new(platform_client_for_request(
        state,
        Platform::Douyin,
        headers,
    ))
}

pub(super) fn kuaishou_fetcher(state: &AppState, headers: &HeaderMap) -> KuaishouFetcher {
    KuaishouFetcher::new(platform_client_for_request(
        state,
        Platform::Kuaishou,
        headers,
    ))
}

pub(super) fn xiaohongshu_fetcher(state: &AppState, headers: &HeaderMap) -> XiaohongshuFetcher {
    XiaohongshuFetcher::new(platform_client_for_request(
        state,
        Platform::Xiaohongshu,
        headers,
    ))
}

pub(super) fn twitter_fetcher(state: &AppState, headers: &HeaderMap) -> TwitterFetcher {
    TwitterFetcher::new(platform_client_for_request(
        state,
        Platform::Twitter,
        headers,
    ))
}

fn platform_client_for_request(
    state: &AppState,
    platform: Platform,
    headers: &HeaderMap,
) -> crate::client::PlatformClient {
    let mut client = state.client.platform(platform);

    match request_cookie_override(headers, platform) {
        RequestCookieOverride::UseConfigured => client,
        RequestCookieOverride::Override(cookie) => {
            strip_cookie_headers(&mut client.request.headers);
            client.cookie = Some(cookie);
            client
        }
        RequestCookieOverride::Clear => {
            strip_cookie_headers(&mut client.request.headers);
            client.cookie = None;
            client
        }
    }
}

fn request_cookie_override(headers: &HeaderMap, platform: Platform) -> RequestCookieOverride {
    for header_name in [platform_cookie_header(platform), GENERIC_COOKIE_HEADER] {
        if let Some(value) = headers.get(header_name) {
            let Ok(value) = value.to_str() else {
                continue;
            };

            let trimmed = value.trim();
            return if trimmed.is_empty() {
                RequestCookieOverride::Clear
            } else {
                RequestCookieOverride::Override(trimmed.to_owned())
            };
        }
    }

    RequestCookieOverride::UseConfigured
}

fn platform_cookie_header(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => BILIBILI_COOKIE_HEADER,
        Platform::Douyin => DOUYIN_COOKIE_HEADER,
        Platform::Kuaishou => KUAISHOU_COOKIE_HEADER,
        Platform::Twitter => TWITTER_COOKIE_HEADER,
        Platform::Xiaohongshu => XIAOHONGSHU_COOKIE_HEADER,
    }
}

fn strip_cookie_headers(headers: &mut std::collections::BTreeMap<String, String>) {
    headers.remove("Cookie");
    headers.remove("cookie");
}
