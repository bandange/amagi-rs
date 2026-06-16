use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{DouyinExtraFields, DouyinResponseMeta};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinStickerQuickReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_name: Option<String>,
    #[serde(
        default,
        rename = "sticker_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub sticker_type: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiVariant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub random_weight: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_quick_reply: Vec<DouyinStickerQuickReply>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiStaticFrame {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_url: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub static_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_info_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_start_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resource_variant_list: Vec<DouyinDynamicEmojiVariant>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub static_url_list: Vec<DouyinDynamicEmojiStaticFrame>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiSpecialResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relation_name: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub special_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_advance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animate_icon: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interactive_resources: Vec<DouyinDynamicEmojiResource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub special_resources: Vec<DouyinDynamicEmojiSpecialResource>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDynamicEmojiList {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_trees: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diverter_tags: Option<Value>,
    #[serde(default)]
    pub do_not_retry: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flame_achieve_dashboard: Option<Value>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub interactive_resource_config: BTreeMap<String, DouyinDynamicEmojiConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_toggles: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
