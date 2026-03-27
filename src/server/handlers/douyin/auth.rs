use axum::{
    Json,
    extract::{Query, State},
};

use super::super::support::{FetchResult, douyin_fetcher, fetch_error_response};
use super::types::DouyinLoginQrcodeQuery;
use crate::platforms::douyin::{DouyinDynamicEmojiList, DouyinEmojiList, DouyinLoginQrcode};
use crate::server::state::AppState;

/// Fetch the Douyin emoji catalog through the web API.
pub async fn douyin_emoji_list(State(state): State<AppState>) -> FetchResult<DouyinEmojiList> {
    douyin_fetcher(&state)
        .fetch_emoji_list()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch the Douyin animated emoji configuration through the web API.
pub async fn douyin_dynamic_emoji_list(
    State(state): State<AppState>,
) -> FetchResult<DouyinDynamicEmojiList> {
    douyin_fetcher(&state)
        .fetch_dynamic_emoji_list()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Request a Douyin login QR code through the web API.
pub async fn douyin_login_qrcode(
    Query(query): Query<DouyinLoginQrcodeQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinLoginQrcode> {
    douyin_fetcher(&state)
        .request_login_qrcode(query.verify_fp.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
