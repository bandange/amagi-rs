use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::TwitterTweet;

/// Search result mode supported by the Twitter/X web search timeline.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "lowercase")]
pub enum TwitterTweetSearchMode {
    /// Return latest matching tweets.
    #[default]
    Latest,
    /// Return top matching tweets.
    Top,
}

impl TwitterTweetSearchMode {
    pub(crate) const fn graphql_product(self) -> &'static str {
        match self {
            Self::Latest => "Latest",
            Self::Top => "Top",
        }
    }
}

/// One page of tweet-search results from Twitter/X.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterTweetSearchPage {
    /// Search query that produced the current result page.
    pub query: String,
    /// Search mode used for the current page.
    pub search_type: TwitterTweetSearchMode,
    /// Tweets returned in upstream order.
    pub tweets: Vec<TwitterTweet>,
    /// Cursor that can be used to request newer items.
    pub previous_cursor: Option<String>,
    /// Cursor that can be used to request older items.
    pub next_cursor: Option<String>,
    /// Wrapper-stripped copy of the upstream page payload with remaining fields preserved.
    pub upstream_payload: Value,
}
