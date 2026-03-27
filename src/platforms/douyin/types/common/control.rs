use serde::{Deserialize, Serialize};

use super::DouyinExtraFields;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinAwemeControl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_forward: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_show_comment: Option<bool>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinCommentPermissionInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_comment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_permission_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_detail_entry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub press_entry: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub toast_guide: Option<bool>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinStatistics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admire_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collect_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forward_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub live_watch_count: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinReviewResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_status: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_comment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_share: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reviewing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_prohibited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub part_see: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_result: Option<DouyinReviewResult>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
