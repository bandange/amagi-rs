use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, fetch_error_response, kuaishou_fetcher};
use crate::platforms::kuaishou::KuaishouLiveRoomInfo;
use crate::server::state::AppState;

/// Fetch aggregated Kuaishou live-room info through the web API.
pub async fn kuaishou_live_room_info(
    Path(principal_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<KuaishouLiveRoomInfo> {
    kuaishou_fetcher(&state, &headers)
        .fetch_live_room_info(&principal_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
