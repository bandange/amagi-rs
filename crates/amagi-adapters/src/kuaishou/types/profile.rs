use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Aggregated response for the Kuaishou user profile page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfile {
    /// Requested profile principal id.
    #[serde(rename = "principalId")]
    pub principal_id: String,
    /// Aggregated author metadata.
    pub author: KuaishouUserProfileAuthor,
    /// Aggregated profile tabs and page metadata.
    pub profile: KuaishouUserProfilePage,
    /// Derived follow state when available.
    pub follow: Option<KuaishouFollowState>,
    /// Derived follow button state when available.
    #[serde(rename = "followButton")]
    pub follow_button: Option<KuaishouFollowButtonState>,
    /// Interest mask list returned by the platform.
    #[serde(rename = "interestMask")]
    pub interest_mask: Vec<Value>,
    /// Category masks returned by the platform.
    #[serde(rename = "categoryMask")]
    pub category_mask: KuaishouCategoryMask,
    /// Complete upstream payload snapshot grouped by contributing sources.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Author metadata embedded in a Kuaishou user profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfileAuthor {
    /// Requested author principal id.
    #[serde(rename = "principalId")]
    pub principal_id: String,
    /// Normalized user info block.
    #[serde(rename = "userInfo")]
    pub user_info: KuaishouUserProfileUserInfo,
    /// Optional sensitive info block from the platform.
    #[serde(rename = "sensitiveInfo")]
    pub sensitive_info: Option<Value>,
    /// Placeholder follow info block kept for shape compatibility.
    #[serde(rename = "followInfo")]
    pub follow_info: Value,
    /// Stable ban-state map used by the original page.
    #[serde(rename = "banStateMap")]
    pub ban_state_map: BTreeMap<String, String>,
}

/// Normalized user info block for a Kuaishou profile author.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfileUserInfo {
    /// Author id.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Author description.
    pub description: String,
    /// Avatar URL.
    pub avatar: String,
    /// Sex label returned by the platform.
    pub sex: String,
    /// Whether the author is currently living.
    pub living: bool,
    /// Follow status.
    #[serde(rename = "followStatus")]
    pub follow_status: String,
    /// Constellation label.
    pub constellation: String,
    /// City name.
    #[serde(rename = "cityName")]
    pub city_name: String,
    /// Original numeric user id.
    #[serde(rename = "originUserId")]
    pub origin_user_id: u64,
    /// Whether the profile is private.
    pub privacy: bool,
    /// Whether the profile is marked as new.
    #[serde(rename = "isNew")]
    pub is_new: bool,
    /// Platform timestamp field.
    pub timestamp: u64,
    /// Verified status details.
    #[serde(rename = "verifiedStatus")]
    pub verified_status: KuaishouVerifiedStatus,
    /// Banned status details.
    #[serde(rename = "bannedStatus")]
    pub banned_status: KuaishouBannedStatus,
    /// Dynamic counts object published by the platform.
    pub counts: Value,
}

/// Verified status block for a Kuaishou user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouVerifiedStatus {
    /// Whether the account is verified.
    pub verified: bool,
    /// Verification description.
    pub description: String,
    /// Verification type id.
    #[serde(rename = "type")]
    pub type_id: i64,
    /// Whether the verification is marked as new.
    #[serde(rename = "new")]
    pub is_new: bool,
    /// Verification icon URL.
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

pub(crate) fn empty_kuaishou_verified_status() -> KuaishouVerifiedStatus {
    KuaishouVerifiedStatus {
        verified: false,
        description: String::new(),
        type_id: 0,
        is_new: false,
        icon_url: String::new(),
    }
}

/// Banned status block for a Kuaishou user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouBannedStatus {
    /// Whether the account is banned.
    pub banned: bool,
    /// Whether the social graph is banned.
    #[serde(rename = "socialBanned")]
    pub social_banned: bool,
    /// Whether the account is isolated.
    pub isolate: bool,
    /// Whether the account is defriended.
    pub defriend: bool,
}

pub(crate) fn empty_kuaishou_banned_status() -> KuaishouBannedStatus {
    KuaishouBannedStatus {
        banned: false,
        social_banned: false,
        isolate: false,
        defriend: false,
    }
}

/// Profile page tabs and metadata for a Kuaishou user profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfilePage {
    /// Current active tab.
    #[serde(rename = "currentTab")]
    pub current_tab: String,
    /// Page size used for tab requests.
    #[serde(rename = "pageSize")]
    pub page_size: u32,
    /// Stable tab type map.
    #[serde(rename = "tabTypeMap")]
    pub tab_type_map: BTreeMap<String, String>,
    /// Whether playback should be shown.
    #[serde(rename = "showPlayback")]
    pub show_playback: bool,
    /// Public tab data.
    #[serde(rename = "publicData")]
    pub public_data: KuaishouUserProfilePublicTabData,
    /// Private tab data.
    #[serde(rename = "privateData")]
    pub private_data: KuaishouUserProfileTabData,
    /// Liked tab data.
    #[serde(rename = "likedData")]
    pub liked_data: KuaishouUserProfileTabData,
    /// Playback tab data.
    #[serde(rename = "playbackData")]
    pub playback_data: KuaishouUserProfileTabData,
    /// Interest list entries.
    #[serde(rename = "interestList")]
    pub interest_list: Vec<Value>,
    /// Placeholder current product block.
    #[serde(rename = "currentProduct")]
    pub current_product: Value,
}

/// Shared list tab data for a Kuaishou user profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfileTabData {
    /// List items returned by the tab endpoint.
    pub list: Vec<Value>,
    /// Pagination cursor.
    pub pcursor: String,
}

/// Public tab data for a Kuaishou user profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserProfilePublicTabData {
    /// Live info block when the author is currently living.
    pub live: Option<Value>,
    /// List items returned by the public tab endpoint.
    pub list: Vec<Value>,
    /// Pagination cursor.
    pub pcursor: String,
}

/// Derived follow state for a Kuaishou profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouFollowState {
    /// Current follow status.
    #[serde(rename = "currentFollowStatus")]
    pub current_follow_status: String,
    /// Whether the user still needs to follow.
    #[serde(rename = "needToFollow")]
    pub need_to_follow: bool,
    /// Author id this follow state belongs to.
    #[serde(rename = "authorId")]
    pub author_id: String,
    /// Placeholder page state integer.
    pub data: i32,
}

/// Derived follow-button state for a Kuaishou profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouFollowButtonState {
    /// Current follow status.
    #[serde(rename = "followStatus")]
    pub follow_status: String,
}

/// Category-mask payload returned by Kuaishou profile endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouCategoryMask {
    /// Category config list.
    pub config: Vec<Value>,
    /// Category list.
    pub list: Vec<Value>,
    /// Hot category list.
    #[serde(rename = "hotList")]
    pub hot_list: Vec<Value>,
    /// Whether more category list data exists.
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Whether more hot category data exists.
    #[serde(rename = "hasMoreHot")]
    pub has_more_hot: bool,
}
