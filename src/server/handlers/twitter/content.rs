use axum::{
    Json,
    extract::{Path, Query, State},
};

use super::super::support::{FetchResult, fetch_error_response, twitter_fetcher};
use super::types::{TwitterRepliesQuery, TwitterSearchQuery, TwitterTimelineQuery};
use crate::platforms::twitter::{
    TwitterTweet, TwitterTweetPage, TwitterTweetSearchPage, TwitterUserPage,
};
use crate::server::state::AppState;

/// Search Twitter/X tweets through the web API.
pub async fn twitter_search_tweets(
    Query(query): Query<TwitterSearchQuery>,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetSearchPage> {
    twitter_fetcher(&state)
        .search_tweets(
            &query.query,
            query.search_type,
            query.count,
            query.cursor.as_deref(),
        )
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch one Twitter/X tweet detail through the web API.
pub async fn twitter_tweet_detail(
    Path(tweet_id): Path<String>,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweet> {
    twitter_fetcher(&state)
        .fetch_tweet_detail(&tweet_id)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch replies to one Twitter/X tweet through the web API.
pub async fn twitter_tweet_replies(
    Path(tweet_id): Path<String>,
    Query(query): Query<TwitterRepliesQuery>,
    State(state): State<AppState>,
) -> FetchResult<TwitterTweetPage> {
    twitter_fetcher(&state)
        .fetch_tweet_replies(&tweet_id, query.cursor.as_deref(), query.sort_by)
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch users who liked one Twitter/X tweet through the web API.
pub async fn twitter_tweet_likers(
    Path(tweet_id): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserPage> {
    twitter_fetcher(&state)
        .fetch_tweet_likers(&tweet_id, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}

/// Fetch users who retweeted one Twitter/X tweet through the web API.
pub async fn twitter_tweet_retweeters(
    Path(tweet_id): Path<String>,
    Query(query): Query<TwitterTimelineQuery>,
    State(state): State<AppState>,
) -> FetchResult<TwitterUserPage> {
    twitter_fetcher(&state)
        .fetch_tweet_retweeters(&tweet_id, query.count, query.cursor.as_deref())
        .await
        .map(Json)
        .map_err(fetch_error_response)
}
