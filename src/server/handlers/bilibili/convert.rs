use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use crate::platforms::bilibili::{BilibiliAvToBv, BilibiliBvToAv};
use crate::server::state::AppState;

/// Convert one AV identifier into its BV representation through the web API.
pub async fn bilibili_av_to_bv(
    Path(aid): Path<u64>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliAvToBv> {
    Ok(Json(
        bilibili_fetcher(&state, &headers).convert_av_to_bv(aid),
    ))
}

/// Convert one BV identifier into its AV representation through the web API.
pub async fn bilibili_bv_to_av(
    Path(bvid): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> FetchResult<BilibiliBvToAv> {
    bilibili_fetcher(&state, &headers)
        .convert_bv_to_av(&bvid)
        .map(Json)
        .map_err(fetch_error_response)
}
