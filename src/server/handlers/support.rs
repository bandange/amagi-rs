use axum::{Json, http::StatusCode};

use super::super::state::AppState;
use super::{CatalogErrorResponse, FetchErrorResponse};
use crate::error::AppError;
use crate::platforms::bilibili::BilibiliFetcher;
use crate::platforms::douyin::DouyinFetcher;
use crate::platforms::kuaishou::KuaishouFetcher;
use crate::platforms::twitter::TwitterFetcher;
use crate::platforms::xiaohongshu::XiaohongshuFetcher;

pub(super) type CatalogResult<T> = Result<Json<T>, (StatusCode, Json<CatalogErrorResponse>)>;
pub(super) type FetchResult<T> = Result<Json<T>, (StatusCode, Json<FetchErrorResponse>)>;

pub(super) fn fetch_error_response(error: AppError) -> (StatusCode, Json<FetchErrorResponse>) {
    let status = match &error {
        AppError::Io(_) | AppError::InvalidRequestConfig(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::Json(_) | AppError::Http(_) | AppError::UpstreamResponse { .. } => {
            StatusCode::BAD_GATEWAY
        }
    };

    (
        status,
        Json(FetchErrorResponse {
            error: "fetch_failed",
            detail: error.to_string(),
        }),
    )
}

pub(super) fn bilibili_fetcher(state: &AppState) -> BilibiliFetcher {
    state.client.bilibili_fetcher()
}

pub(super) fn douyin_fetcher(state: &AppState) -> DouyinFetcher {
    state.client.douyin_fetcher()
}

pub(super) fn kuaishou_fetcher(state: &AppState) -> KuaishouFetcher {
    state.client.kuaishou_fetcher()
}

pub(super) fn xiaohongshu_fetcher(state: &AppState) -> XiaohongshuFetcher {
    state.client.xiaohongshu_fetcher()
}

pub(super) fn twitter_fetcher(state: &AppState) -> TwitterFetcher {
    state.client.twitter_fetcher()
}
