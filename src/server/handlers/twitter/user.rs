use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, fetch_error_response, twitter_fetcher};
use super::types::{TwitterTimelineQuery, TwitterUserSearchQuery};
use crate::platforms::twitter::{
    TwitterTweetPage, TwitterUserListPage, TwitterUserPage, TwitterUserProfile, TwitterUserTimeline,
};
use crate::server::state::AppState;

/// Fetch a Twitter/X user profile through the web API.
pub async fn twitter_user_profile(
    Path(screen_name): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserProfile> {
    twitter_fetcher(&state, &headers)
        .fetch_user_profile(&screen_name)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X user timeline through the web API.
pub async fn twitter_user_timeline(
    Path(screen_name): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserTimeline> {
    twitter_fetcher(&state, &headers)
        .fetch_user_timeline(&screen_name, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X user replies timeline through the web API.
pub async fn twitter_user_replies(
    Path(screen_name): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserTimeline> {
    twitter_fetcher(&state, &headers)
        .fetch_user_replies(&screen_name, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X user media timeline through the web API.
pub async fn twitter_user_media(
    Path(screen_name): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserTimeline> {
    twitter_fetcher(&state, &headers)
        .fetch_user_media(&screen_name, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X user's followers through the web API.
pub async fn twitter_user_followers(
    Path(screen_name): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserListPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_followers(&screen_name, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch a Twitter/X user's following list through the web API.
pub async fn twitter_user_following(
    Path(screen_name): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserListPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_following(&screen_name, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch liked tweets for the authenticated Twitter/X account through the web API.
pub async fn twitter_user_likes(
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_likes(query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch authenticated Twitter/X bookmarks through the web API.
pub async fn twitter_user_bookmarks(
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_bookmarks(query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch authenticated Twitter/X followed feed through the web API.
pub async fn twitter_user_followed(
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_followed(query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch authenticated Twitter/X recommended feed through the web API.
pub async fn twitter_user_recommended(
    Query(query): Query<TwitterTimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetPage> {
    twitter_fetcher(&state, &headers)
        .fetch_user_recommended(query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Search Twitter/X users through the web API.
pub async fn twitter_search_users(
    Query(query): Query<TwitterUserSearchQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserPage> {
    twitter_fetcher(&state, &headers)
        .search_users(&query.query, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
