use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::TwitterUserProfile;
use crate::platforms::twitter::TwitterUserSummary;

/// Reply sorting mode supported by the Twitter/X conversation endpoint.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[serde(rename_all = "lowercase")]
pub enum TwitterTweetRepliesSortMode {
    /// Return replies ranked by recency.
    #[default]
    Latest,
    /// Return replies ranked by total likes.
    Likes,
    /// Return replies ranked by X relevance ordering.
    Relevance,
}

impl TwitterTweetRepliesSortMode {
    pub(crate) const fn graphql_ranking_mode(self) -> &'static str {
        match self {
            Self::Latest => "Recency",
            Self::Likes => "Likes",
            Self::Relevance => "Relevance",
        }
    }
}

/// One media entity attached to a tweet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterMediaEntity {
    /// Stable media type reported by X such as `photo` or `video`.
    pub media_type: String,
    /// Preferred HTTPS media URL when available.
    pub media_url: Option<String>,
    /// Preview image URL for video or gif media.
    pub preview_image_url: Option<String>,
    /// Expanded external URL exposed by the entity.
    pub expanded_url: Option<String>,
}

/// One URL entity attached to tweet text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUrlEntity {
    /// Original shortened URL present in tweet text.
    pub url: Option<String>,
    /// Expanded target URL when present.
    pub expanded_url: Option<String>,
    /// Display URL rendered by X.
    pub display_url: Option<String>,
    /// Fully resolved URL when X exposes it.
    pub unwound_url: Option<String>,
    /// Inclusive start and exclusive end offsets inside the text.
    pub indices: Option<[u32; 2]>,
}

/// One mentioned user entity attached to tweet text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserMentionEntity {
    /// Stable numeric user id when present.
    pub id: Option<String>,
    /// Mentioned screen name without the leading `@`.
    pub screen_name: Option<String>,
    /// Display name returned by X when present.
    pub name: Option<String>,
    /// Inclusive start and exclusive end offsets inside the text.
    pub indices: Option<[u32; 2]>,
}

/// One hashtag entity attached to tweet text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterHashtagEntity {
    /// Hashtag text without the leading `#`.
    pub text: String,
    /// Inclusive start and exclusive end offsets inside the text.
    pub indices: Option<[u32; 2]>,
}

/// One cashtag or symbol entity attached to tweet text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterSymbolEntity {
    /// Symbol text without the leading `$`.
    pub text: String,
    /// Inclusive start and exclusive end offsets inside the text.
    pub indices: Option<[u32; 2]>,
}

/// One timestamp entity attached to tweet text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterTimestampEntity {
    /// Timestamp text rendered in the tweet body.
    pub text: String,
    /// Inclusive start and exclusive end offsets inside the text.
    pub indices: Option<[u32; 2]>,
}

/// Text entities extracted from the tweet payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterTweetEntities {
    /// URL entities attached to the text body.
    pub urls: Vec<TwitterUrlEntity>,
    /// Mentioned users attached to the text body.
    pub user_mentions: Vec<TwitterUserMentionEntity>,
    /// Hashtag entities attached to the text body.
    pub hashtags: Vec<TwitterHashtagEntity>,
    /// Cashtag entities attached to the text body.
    pub symbols: Vec<TwitterSymbolEntity>,
    /// Timestamp entities attached to the text body.
    pub timestamps: Vec<TwitterTimestampEntity>,
}

/// Public Twitter/X tweet payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterTweet {
    /// Stable tweet id.
    pub id: String,
    /// Root conversation id for the tweet.
    pub conversation_id: Option<String>,
    /// Author summary for the tweet.
    pub author: TwitterUserSummary,
    /// Canonical tweet URL.
    pub url: String,
    /// Creation time as an RFC3339 or upstream timestamp string when available.
    pub created_at: Option<String>,
    /// Full text with note-tweet fallback applied.
    pub full_text: String,
    /// Language code reported by X.
    pub language: Option<String>,
    /// Raw source HTML fragment reported by X.
    pub source: Option<String>,
    /// Reply target tweet id when the tweet is a reply.
    pub reply_to_tweet_id: Option<String>,
    /// Quoted tweet when the response embeds it.
    pub quoted_tweet: Option<Box<TwitterTweet>>,
    /// Retweeted tweet when the response embeds it.
    pub retweeted_tweet: Option<Box<TwitterTweet>>,
    /// Wrapper-stripped copy of the upstream tweet payload with remaining fields preserved.
    pub upstream_payload: Value,
    /// Text entities extracted from note-tweet or legacy tweet payloads.
    pub entities: TwitterTweetEntities,
    /// Attached media entities.
    pub media: Vec<TwitterMediaEntity>,
    /// Reply count.
    pub reply_count: u64,
    /// Retweet count.
    pub retweet_count: u64,
    /// Quote count.
    pub quote_count: u64,
    /// Like count.
    pub favorite_count: u64,
    /// Bookmark count when present.
    pub bookmark_count: Option<u64>,
    /// View count when present.
    pub view_count: Option<u64>,
}

/// One paginated Twitter/X tweet collection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterTweetPage {
    /// Tweets returned in upstream order.
    pub tweets: Vec<TwitterTweet>,
    /// Cursor that can be used to request newer items.
    pub previous_cursor: Option<String>,
    /// Cursor that can be used to request older items.
    pub next_cursor: Option<String>,
    /// Wrapper-stripped copy of the upstream page payload with remaining fields preserved.
    pub upstream_payload: Value,
}

/// Public Twitter/X user timeline payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserTimeline {
    /// Profile information resolved for the requested user.
    pub user: TwitterUserProfile,
    /// Timeline tweets in upstream order.
    pub tweets: Vec<TwitterTweet>,
    /// Cursor that can be used to request newer items.
    pub previous_cursor: Option<String>,
    /// Cursor that can be used to request older items.
    pub next_cursor: Option<String>,
    /// Wrapper-stripped copy of the upstream page payload with remaining fields preserved.
    pub upstream_payload: Value,
}
