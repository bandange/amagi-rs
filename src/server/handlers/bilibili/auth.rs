use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use super::{
    BilibiliCaptchaFromVoucherRequest, BilibiliQrcodeStatusQuery, BilibiliValidateCaptchaRequest,
};
use crate::platforms::bilibili::{
    BilibiliCaptchaFromVoucher, BilibiliEmojiList, BilibiliLoginQrcode, BilibiliLoginStatus,
    BilibiliQrcodeStatus, BilibiliValidateCaptcha,
};
use crate::server::state::AppState;

/// Request a Bilibili captcha challenge from a voucher through the web API.
pub async fn bilibili_captcha_from_voucher(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<BilibiliCaptchaFromVoucherRequest>,
) -> FetchResult<BilibiliCaptchaFromVoucher> {
    bilibili_fetcher(&state, &headers)
        .request_captcha_from_voucher(&payload.v_voucher, payload.csrf.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Validate a Bilibili captcha result through the web API.
pub async fn bilibili_validate_captcha(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(payload): Json<BilibiliValidateCaptchaRequest>,
) -> FetchResult<BilibiliValidateCaptcha> {
    bilibili_fetcher(&state, &headers)
        .validate_captcha_result(
            &payload.challenge,
            &payload.token,
            &payload.validate,
            &payload.seccode,
            payload.csrf.as_deref(),
        )
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch the current Bilibili login status through the web API.
pub async fn bilibili_login_status(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliLoginStatus> {
    bilibili_fetcher(&state, &headers)
        .fetch_login_status()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Request a Bilibili login QR code through the web API.
pub async fn bilibili_login_qrcode(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliLoginQrcode> {
    bilibili_fetcher(&state, &headers)
        .request_login_qrcode()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Poll one Bilibili login QR code through the web API.
pub async fn bilibili_qrcode_status(
    Query(query): Query<BilibiliQrcodeStatusQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliQrcodeStatus> {
    bilibili_fetcher(&state, &headers)
        .check_qrcode_status(&query.qrcode_key)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch the Bilibili emoji catalog through the web API.
pub async fn bilibili_emoji_list(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliEmojiList> {
    bilibili_fetcher(&state, &headers)
        .fetch_emoji_list()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
