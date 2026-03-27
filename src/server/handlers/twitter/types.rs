use serde::Deserialize;

use crate::platforms::twitter::{TwitterTweetRepliesSortMode, TwitterTweetSearchMode};

/// Query parameters accepted by the Twitter timeline handler.
#[derive(Debug, Default, Deserialize)]
pub struct TwitterTimelineQuery {
    /// Optional page size.
    pub count: Option<u32>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

/// Query parameters accepted by the Twitter tweet-search handler.
#[derive(Debug, Deserialize)]
pub struct TwitterSearchQuery {
    /// Search keyword or advanced query.
    pub query: String,
    /// Optional search result type.
    pub search_type: Option<TwitterTweetSearchMode>,
    /// Optional page size.
    pub count: Option<u32>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

/// Query parameters accepted by the Twitter user-search handler.
#[derive(Debug, Deserialize)]
pub struct TwitterUserSearchQuery {
    /// Search keyword or screen name fragment.
    pub query: String,
    /// Optional page size.
    pub count: Option<u32>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

/// Query parameters accepted by the Twitter tweet-replies handler.
#[derive(Debug, Default, Deserialize)]
pub struct TwitterRepliesQuery {
    /// Optional pagination cursor.
    pub cursor: Option<String>,
    /// Optional reply sorting mode.
    pub sort_by: Option<TwitterTweetRepliesSortMode>,
}
