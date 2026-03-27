use axum::{
    Json,
    extract::{Path, State},
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use crate::platforms::bilibili::{
    BilibiliUploaderTotalViews, BilibiliUserCard, BilibiliUserDynamicList, BilibiliUserSpaceInfo,
};
use crate::server::state::AppState;

/// Fetch one Bilibili user card through the web API.
pub async fn bilibili_user_card(
    Path(host_mid): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliUserCard> {
    bilibili_fetcher(&state)
        .fetch_user_card(host_mid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili user dynamic list through the web API.
pub async fn bilibili_user_dynamic_list(
    Path(host_mid): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliUserDynamicList> {
    bilibili_fetcher(&state)
        .fetch_user_dynamic_list(host_mid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili user space payload through the web API.
pub async fn bilibili_user_space_info(
    Path(host_mid): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliUserSpaceInfo> {
    bilibili_fetcher(&state)
        .fetch_user_space_info(host_mid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili uploader total-views payload through the web API.
pub async fn bilibili_uploader_total_views(
    Path(host_mid): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliUploaderTotalViews> {
    bilibili_fetcher(&state)
        .fetch_uploader_total_views(host_mid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
