use axum::{
    Json,
    extract::{Path, State},
};

use super::super::support::{FetchResult, fetch_error_response, kuaishou_fetcher};
use crate::platforms::kuaishou::{KuaishouEmojiList, KuaishouVideoWork, KuaishouWorkComments};
use crate::server::state::AppState;

/// Fetch one Kuaishou work through the web API.
pub async fn kuaishou_video_work(
    Path(photo_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<KuaishouVideoWork> {
    kuaishou_fetcher(&state)
        .fetch_video_work(&photo_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch comments for one Kuaishou work through the web API.
pub async fn kuaishou_work_comments(
    Path(photo_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<KuaishouWorkComments> {
    kuaishou_fetcher(&state)
        .fetch_work_comments(&photo_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch the Kuaishou emoji catalog through the web API.
pub async fn kuaishou_emoji_list(State(state): State<AppState>) -> FetchResult<KuaishouEmojiList> {
    kuaishou_fetcher(&state)
        .fetch_emoji_list()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
