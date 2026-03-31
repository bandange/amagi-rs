use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use super::{BilibiliCommentQuery, BilibiliCommentRepliesQuery};
use crate::platforms::bilibili::{
    BilibiliCommentReplies, BilibiliComments, BilibiliDynamicCard, BilibiliDynamicDetail,
};
use crate::server::state::AppState;

/// Fetch Bilibili comments through the web API.
pub async fn bilibili_comments(
    Path(oid): Path<u64>,
    Query(query): Query<BilibiliCommentQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliComments> {
    bilibili_fetcher(&state, &headers)
        .fetch_comments(oid, query.comment_type, query.number, query.mode)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch replies for one Bilibili root comment through the web API.
pub async fn bilibili_comment_replies(
    Path((oid, root)): Path<(u64, u64)>,
    Query(query): Query<BilibiliCommentRepliesQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliCommentReplies> {
    bilibili_fetcher(&state, &headers)
        .fetch_comment_replies(oid, query.comment_type, root, query.number)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili dynamic detail payload through the web API.
pub async fn bilibili_dynamic_detail(
    Path(dynamic_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliDynamicDetail> {
    bilibili_fetcher(&state, &headers)
        .fetch_dynamic_detail(&dynamic_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili dynamic card payload through the web API.
pub async fn bilibili_dynamic_card(
    Path(dynamic_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliDynamicCard> {
    bilibili_fetcher(&state, &headers)
        .fetch_dynamic_card(&dynamic_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
