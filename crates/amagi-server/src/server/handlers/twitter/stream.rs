use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, fetch_error_response, twitter_fetcher};
use crate::server::state::AppState;
use amagi_adapters::twitter::TwitterLiveRoomStream;

/// Fetch a Twitter/X live-room playback stream by broadcast id.
pub async fn twitter_live_room_stream(
    Path(broadcast_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterLiveRoomStream> {
    twitter_fetcher(&state, &headers)
        .fetch_live_room_stream(&broadcast_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X live-room playback stream by broadcast media key.
pub async fn twitter_live_room_stream_by_media_key(
    Path(media_key): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterLiveRoomStream> {
    twitter_fetcher(&state, &headers)
        .fetch_live_room_stream_by_media_key(&media_key)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X live-room playback stream from a tweet broadcast card.
pub async fn twitter_tweet_live_room_stream(
    Path(tweet_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterLiveRoomStream> {
    twitter_fetcher(&state, &headers)
        .fetch_live_room_stream_by_tweet_id(&tweet_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
