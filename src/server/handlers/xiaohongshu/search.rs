use axum::{
    Json,
    extract::{Query, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, fetch_error_response, xiaohongshu_fetcher};
use super::types::XiaohongshuSearchQuery;
use crate::platforms::xiaohongshu::{XiaohongshuSearchNotes, XiaohongshuSearchNotesOptions};
use crate::server::state::AppState;

/// Search Xiaohongshu notes through the web API.
pub async fn xiaohongshu_search_notes(
    Query(query): Query<XiaohongshuSearchQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuSearchNotes> {
    xiaohongshu_fetcher(&state, &headers)
        .search_notes(&XiaohongshuSearchNotesOptions {
            keyword: query.keyword,
            page: query.page,
            page_size: query.page_size,
            sort: query.sort,
            note_type: query.note_type,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
