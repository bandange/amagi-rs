use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use super::{BilibiliCidQuery, BilibiliDanmakuQuery};
use crate::platforms::bilibili::{
    BilibiliBangumiInfo, BilibiliBangumiStream, BilibiliDanmakuList, BilibiliVideoInfo,
    BilibiliVideoStream,
};
use crate::server::state::AppState;

/// Fetch one Bilibili video payload through the web API.
pub async fn bilibili_video_info(
    Path(bvid): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliVideoInfo> {
    bilibili_fetcher(&state)
        .fetch_video_info(&bvid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili video playurl payload through the web API.
pub async fn bilibili_video_stream(
    Path(aid): Path<u64>,
    Query(query): Query<BilibiliCidQuery>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliVideoStream> {
    bilibili_fetcher(&state)
        .fetch_video_stream(aid, query.cid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili video danmaku segment through the web API.
pub async fn bilibili_video_danmaku(
    Path(cid): Path<u64>,
    Query(query): Query<BilibiliDanmakuQuery>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliDanmakuList> {
    bilibili_fetcher(&state)
        .fetch_video_danmaku(cid, query.segment_index)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili bangumi detail payload through the web API.
pub async fn bilibili_bangumi_info(
    Path(bangumi_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliBangumiInfo> {
    bilibili_fetcher(&state)
        .fetch_bangumi_info(&bangumi_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili bangumi playurl payload through the web API.
pub async fn bilibili_bangumi_stream(
    Path(ep_id): Path<String>,
    Query(query): Query<BilibiliCidQuery>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliBangumiStream> {
    bilibili_fetcher(&state)
        .fetch_bangumi_stream(&ep_id, query.cid)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
