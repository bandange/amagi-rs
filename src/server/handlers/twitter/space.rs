use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, fetch_error_response, twitter_fetcher};
use crate::platforms::twitter::TwitterSpaceDetail;
use crate::server::state::AppState;

/// Fetch one Twitter/X Space detail through the web API.
pub async fn twitter_space_detail(
    Path(space_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterSpaceDetail> {
    twitter_fetcher(&state, &headers)
        .fetch_space_detail(&space_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
