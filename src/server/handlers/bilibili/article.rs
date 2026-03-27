use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, bilibili_fetcher, fetch_error_response};
use super::BilibiliArticleCardsQuery;
use crate::platforms::bilibili::{
    BilibiliArticleCards, BilibiliArticleContent, BilibiliArticleInfo, BilibiliArticleListInfo,
};
use crate::server::state::AppState;

/// Fetch one Bilibili article content payload through the web API.
pub async fn bilibili_article_content(
    Path(article_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliArticleContent> {
    bilibili_fetcher(&state)
        .fetch_article_content(&article_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili article-card payload through the web API.
pub async fn bilibili_article_cards(
    Query(query): Query<BilibiliArticleCardsQuery>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliArticleCards> {
    bilibili_fetcher(&state)
        .fetch_article_cards(
            query
                .ids
                .split(',')
                .map(str::trim)
                .filter(|value| !value.is_empty()),
        )
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili article metadata payload through the web API.
pub async fn bilibili_article_info(
    Path(article_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliArticleInfo> {
    bilibili_fetcher(&state)
        .fetch_article_info(&article_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Bilibili article-list payload through the web API.
pub async fn bilibili_article_list_info(
    Path(list_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<BilibiliArticleListInfo> {
    bilibili_fetcher(&state)
        .fetch_article_list_info(&list_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
