use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, douyin_fetcher, fetch_error_response};
use super::types::DouyinLiveRoomQuery;
use crate::platforms::douyin::DouyinLiveRoomInfo;
use crate::server::state::AppState;

/// Fetch Douyin live room information through the web API.
pub async fn douyin_live_room_info(
    Path(room_id): Path<String>,
    Query(query): Query<DouyinLiveRoomQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinLiveRoomInfo> {
    douyin_fetcher(&state)
        .fetch_live_room_info(&room_id, &query.web_rid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
