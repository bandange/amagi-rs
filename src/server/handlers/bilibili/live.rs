use axum::{
    Json,
    extract::{Path, State},
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use crate::platforms::bilibili::{BilibiliLiveRoomInfo, BilibiliLiveRoomInit};
use crate::server::state::AppState;

/// Fetch one Bilibili live room detail payload through the web API.
pub async fn bilibili_live_room_info(
    Path(room_id): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliLiveRoomInfo> {
    bilibili_fetcher(&state)
        .fetch_live_room_info(room_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili live room init payload through the web API.
pub async fn bilibili_live_room_init(
    Path(room_id): Path<u64>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliLiveRoomInit> {
    bilibili_fetcher(&state)
        .fetch_live_room_init(room_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
