use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, douyin_fetcher, fetch_error_response};
use super::types::DouyinUserListQuery;
use crate::platforms::douyin::{
    DouyinUserFavoriteList, DouyinUserProfile, DouyinUserRecommendList, DouyinUserVideoList,
};
use crate::server::state::AppState;

/// Fetch a Douyin user profile through the web API.
pub async fn douyin_user_profile(
    Path(sec_uid): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinUserProfile> {
    douyin_fetcher(&state, &headers)
        .fetch_user_profile(&sec_uid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a page of Douyin user videos through the web API.
pub async fn douyin_user_video_list(
    Path(sec_uid): Path<String>,
    Query(query): Query<DouyinUserListQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinUserVideoList> {
    douyin_fetcher(&state, &headers)
        .fetch_user_video_list(&sec_uid, query.number, query.max_cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a page of Douyin user favorites through the web API.
pub async fn douyin_user_favorite_list(
    Path(sec_uid): Path<String>,
    Query(query): Query<DouyinUserListQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinUserFavoriteList> {
    douyin_fetcher(&state, &headers)
        .fetch_user_favorite_list(&sec_uid, query.number, query.max_cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a page of Douyin user recommendations through the web API.
pub async fn douyin_user_recommend_list(
    Path(sec_uid): Path<String>,
    Query(query): Query<DouyinUserListQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<DouyinUserRecommendList> {
    douyin_fetcher(&state, &headers)
        .fetch_user_recommend_list(&sec_uid, query.number, query.max_cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
