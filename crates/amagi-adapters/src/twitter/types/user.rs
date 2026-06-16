use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Compact Twitter/X user information reused by tweet and space payloads.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserSummary {
    /// Stable numeric user id.
    pub id: String,
    /// Public screen name without the leading `@`.
    pub screen_name: String,
    /// Display name shown in the X UI.
    pub name: String,
    /// Preferred avatar URL when present.
    pub avatar_url: Option<String>,
    /// Whether the account is currently blue or business verified.
    pub verified: bool,
    /// Wrapper-stripped copy of the upstream user payload with remaining fields preserved.
    pub upstream_payload: Value,
}

/// Public Twitter/X user profile payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserProfile {
    /// Stable numeric user id.
    pub id: String,
    /// Public screen name without the leading `@`.
    pub screen_name: String,
    /// Display name shown in the X UI.
    pub name: String,
    /// Account creation time as an RFC3339 or upstream timestamp string when available.
    pub created_at: Option<String>,
    /// Public bio text.
    pub description: Option<String>,
    /// Free-form profile location.
    pub location: Option<String>,
    /// Preferred avatar URL when present.
    pub avatar_url: Option<String>,
    /// Profile banner URL when present.
    pub banner_url: Option<String>,
    /// Whether the account is currently blue or business verified.
    pub verified: bool,
    /// Whether the account is protected.
    pub protected: bool,
    /// Number of followers.
    pub followers_count: u64,
    /// Number of followed accounts.
    pub following_count: u64,
    /// Number of published tweets.
    pub statuses_count: u64,
    /// Number of published media tweets.
    pub media_count: u64,
    /// Number of liked tweets.
    pub favourites_count: u64,
    /// Optional pinned tweet id.
    pub pinned_tweet_id: Option<String>,
    /// Wrapper-stripped copy of the upstream user payload with remaining fields preserved.
    pub upstream_payload: Value,
}

/// One paginated Twitter/X user collection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserPage {
    /// Users returned by the current page.
    pub users: Vec<TwitterUserProfile>,
    /// Cursor that can be used to request newer items.
    pub previous_cursor: Option<String>,
    /// Cursor that can be used to request older items.
    pub next_cursor: Option<String>,
    /// Wrapper-stripped copy of the upstream page payload with remaining fields preserved.
    pub upstream_payload: Value,
}

/// One paginated Twitter/X user collection bound to a target profile.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterUserListPage {
    /// Profile information resolved for the requested user.
    pub user: TwitterUserProfile,
    /// Users returned by the current page.
    pub users: Vec<TwitterUserProfile>,
    /// Cursor that can be used to request newer items.
    pub previous_cursor: Option<String>,
    /// Cursor that can be used to request older items.
    pub next_cursor: Option<String>,
    /// Wrapper-stripped copy of the upstream page payload with remaining fields preserved.
    pub upstream_payload: Value,
}
