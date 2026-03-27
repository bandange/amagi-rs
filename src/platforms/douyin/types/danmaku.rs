use serde::{Deserialize, Serialize};

use super::common::{DouyinExtraFields, DouyinResponseMeta};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDanmakuItemExtra {
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDanmakuItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub danmaku_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<DouyinDanmakuItemExtra>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDanmakuList {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(
        default,
        rename = "danmaku_list",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub danmaku_list: Vec<DouyinDanmakuItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<usize>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
