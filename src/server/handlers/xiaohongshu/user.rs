use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, fetch_error_response, xiaohongshu_fetcher};
use super::types::{XiaohongshuUserNoteListQuery, XiaohongshuUserProfileQuery};
use crate::platforms::xiaohongshu::{
    XiaohongshuUserNoteList, XiaohongshuUserNotesOptions, XiaohongshuUserProfile,
    XiaohongshuUserProfileOptions,
};
use crate::server::state::AppState;

/// Fetch one Xiaohongshu user profile through the web API.
pub async fn xiaohongshu_user_profile(
    Path(user_id): Path<String>,
    Query(query): Query<XiaohongshuUserProfileQuery>,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuUserProfile> {
    xiaohongshu_fetcher(&state)
        .fetch_user_profile(&XiaohongshuUserProfileOptions {
            user_id,
            xsec_token: query.xsec_token,
            xsec_source: query.xsec_source,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one page of Xiaohongshu user notes through the web API.
pub async fn xiaohongshu_user_note_list(
    Path(user_id): Path<String>,
    Query(query): Query<XiaohongshuUserNoteListQuery>,
    State(state): State<AppState>,
) -> FetchResult<XiaohongshuUserNoteList> {
    xiaohongshu_fetcher(&state)
        .fetch_user_note_list(&XiaohongshuUserNotesOptions {
            user_id,
            xsec_token: query.xsec_token,
            xsec_source: query.xsec_source,
            cursor: query.cursor,
            num: query.num,
        })
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
