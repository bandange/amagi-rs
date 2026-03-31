use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, douyin_fetcher, fetch_error_response};
use super::types::DouyinDanmakuQuery;
use crate::platforms::douyin::{
    DouyinDanmakuList, DouyinImageAlbumWork, DouyinMusicInfo, DouyinParsedWork, DouyinSlidesWork,
    DouyinTextWork, DouyinVideoWork,
};
use crate::server::state::AppState;

/// Parse one Douyin work through the web API.
pub async fn douyin_parse_work(
    Path(aweme_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinParsedWork> {
    douyin_fetcher(&state, &headers)
        .parse_work(&aweme_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Douyin video work through the web API.
pub async fn douyin_video_work(
    Path(aweme_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinVideoWork> {
    douyin_fetcher(&state, &headers)
        .fetch_video_work(&aweme_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Douyin image album work through the web API.
pub async fn douyin_image_album_work(
    Path(aweme_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinImageAlbumWork> {
    douyin_fetcher(&state, &headers)
        .fetch_image_album_work(&aweme_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Douyin slides work through the web API.
pub async fn douyin_slides_work(
    Path(aweme_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinSlidesWork> {
    douyin_fetcher(&state, &headers)
        .fetch_slides_work(&aweme_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Douyin text work through the web API.
pub async fn douyin_text_work(
    Path(aweme_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinTextWork> {
    douyin_fetcher(&state, &headers)
        .fetch_text_work(&aweme_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch Douyin music metadata through the web API.
pub async fn douyin_music_info(
    Path(music_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinMusicInfo> {
    douyin_fetcher(&state, &headers)
        .fetch_music_info(&music_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch Douyin danmaku through the web API.
pub async fn douyin_danmaku_list(
    Path(aweme_id): Path<String>,
    Query(query): Query<DouyinDanmakuQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinDanmakuList> {
    douyin_fetcher(&state, &headers)
        .fetch_danmaku_list(&aweme_id, query.duration, query.start_time, query.end_time)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
