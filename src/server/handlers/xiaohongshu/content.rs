use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, fetch_error_response, xiaohongshu_fetcher};
use super::types::{XiaohongshuHomeFeedQuery, XiaohongshuNoteCommentsQuery, XiaohongshuNoteQuery};
use crate::platforms::xiaohongshu::{
    XiaohongshuCommentsOptions, XiaohongshuEmojiList, XiaohongshuHomeFeed,
    XiaohongshuHomeFeedOptions, XiaohongshuNoteComments, XiaohongshuNoteDetail,
    XiaohongshuNoteDetailOptions,
};
use crate::server::state::AppState;

/// Fetch the Xiaohongshu home feed through the web API.
pub async fn xiaohongshu_home_feed(
    Query(query): Query<XiaohongshuHomeFeedQuery>,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuHomeFeed> {
    xiaohongshu_fetcher(&state)
        .fetch_home_feed(&XiaohongshuHomeFeedOptions {
            cursor_score: query.cursor_score,
            num: query.num,
            refresh_type: query.refresh_type,
            note_index: query.note_index,
            category: query.category,
            search_key: query.search_key,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Xiaohongshu note detail through the web API.
pub async fn xiaohongshu_note_detail(
    Path(note_id): Path<String>,
    Query(query): Query<XiaohongshuNoteQuery>,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuNoteDetail> {
    xiaohongshu_fetcher(&state)
        .fetch_note_detail(&XiaohongshuNoteDetailOptions {
            note_id,
            xsec_token: query.xsec_token,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one page of Xiaohongshu note comments through the web API.
pub async fn xiaohongshu_note_comments(
    Path(note_id): Path<String>,
    Query(query): Query<XiaohongshuNoteCommentsQuery>,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuNoteComments> {
    xiaohongshu_fetcher(&state)
        .fetch_note_comments(&XiaohongshuCommentsOptions {
            note_id,
            cursor: query.cursor,
            xsec_token: query.xsec_token,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch the Xiaohongshu emoji catalog through the web API.
pub async fn xiaohongshu_emoji_list(
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuEmojiList> {
    xiaohongshu_fetcher(&state)
        .fetch_emoji_list()
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
