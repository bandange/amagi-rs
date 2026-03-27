use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, fetch_error_response, kuaishou_fetcher};
use super::types::KuaishouWorkListQuery;
use crate::platforms::kuaishou::{KuaishouUserProfile, KuaishouUserWorkList};
use crate::server::state::AppState;

/// Fetch the aggregated Kuaishou user profile through the web API.
pub async fn kuaishou_user_profile(
    Path(principal_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<KuaishouUserProfile> {
    kuaishou_fetcher(&state)
        .fetch_user_profile(&principal_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one page of public Kuaishou works for a user through the web API.
pub async fn kuaishou_user_work_list(
    Path(principal_id): Path<String>,
    Query(query): Query<KuaishouWorkListQuery>,
    State(state): State<AppState>,
) -> FetchResult<KuaishouUserWorkList> {
    kuaishou_fetcher(&state)
        .fetch_user_work_list(&principal_id, query.count, query.pcursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
