use serde::{Deserialize, Serialize};

use super::common::{
    DouyinAwemeControl, DouyinExtraFields, DouyinImageUrl, DouyinResponseMeta, DouyinShareInfo,
    deserialize_null_default_vec,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinUserPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinUserTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_cert_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sec_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub short_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_verify: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise_verify_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_thumb: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_medium: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_larger: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_168x168: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_300x300: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_icon: Option<DouyinImageUrl>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cover_url: Vec<DouyinImageUrl>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub white_cover_url: Vec<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_info: Option<DouyinShareInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_control: Option<DouyinAwemeControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub favoriting_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_count_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_follow_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_favorited: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_ad_fake: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_block: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocked_v2: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocking_v2: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_star: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prevent_download: Option<bool>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_permissions: Vec<DouyinUserPermission>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub user_tags: Vec<DouyinUserTag>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinUserProfile {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<DouyinUser>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
