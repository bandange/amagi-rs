use serde::{Deserialize, Serialize};

use super::common::{DouyinExtraFields, DouyinImageUrl, DouyinResponseMeta};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinEmojiItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hide: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_uri: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinEmojiList {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, rename = "emoji_list", skip_serializing_if = "Vec::is_empty")]
    pub emoji_list: Vec<DouyinEmojiItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
