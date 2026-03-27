use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, douyin_fetcher, fetch_error_response};
use super::types::{DouyinCommentQuery, DouyinSearchQuery};
use crate::platforms::douyin::{
    DouyinCommentReplies, DouyinSearchResult, DouyinSuggestWords, DouyinWorkComments,
};
use crate::server::state::AppState;

/// Fetch comments for one Douyin work through the web API.
pub async fn douyin_work_comments(
    Path(aweme_id): Path<String>,
    Query(query): Query<DouyinCommentQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinWorkComments> {
    douyin_fetcher(&state)
        .fetch_work_comments(&aweme_id, query.number, query.cursor)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch replies for one Douyin comment through the web API.
pub async fn douyin_comment_replies(
    Path((aweme_id, comment_id)): Path<(String, String)>,
    Query(query): Query<DouyinCommentQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinCommentReplies> {
    douyin_fetcher(&state)
        .fetch_comment_replies(&aweme_id, &comment_id, query.number, query.cursor)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Search Douyin content through the web API.
pub async fn douyin_search(
    Query(query): Query<DouyinSearchQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinSearchResult> {
    douyin_fetcher(&state)
        .search_content(
            &query.query,
            query.search_type,
            query.number,
            query.search_id.as_deref(),
        )
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch Douyin suggestion keywords through the web API.
pub async fn douyin_suggest_words(
    Query(query): Query<DouyinSearchQuery>,
    State(state): State<AppState>,
) -> FetchResult<DouyinSuggestWords> {
    douyin_fetcher(&state)
        .fetch_suggest_words(&query.query)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
