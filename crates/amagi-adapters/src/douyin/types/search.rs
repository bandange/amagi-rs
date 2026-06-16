use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{DouyinExtraFields, DouyinResponseMeta};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSearchDataItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_info: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_unique_name: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSearchUserItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_info: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSearchResult {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<DouyinSearchDataItem>,
    #[serde(default, rename = "user_list", skip_serializing_if = "Vec::is_empty")]
    pub user_list: Vec<DouyinSearchUserItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i64>,
    #[serde(default)]
    pub has_more: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rid: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordParamExtra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel_info: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<DouyinSuggestWordParamExtra>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_gid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestedWord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<DouyinSuggestWordParams>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordsExtraInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qrec_channel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qrec_channel_is_aweme: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_reason: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordsDataParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<DouyinSuggestWordsExtraInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_gid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impr_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordsDataItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<DouyinSuggestWordsDataParams>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub words: Vec<DouyinSuggestedWord>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordsTimeCost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_rpc_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_engine_cost: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_inner: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWordsExtra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_per_refresh: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qrec_extra: Option<String>,
    #[serde(default, rename = "RespFrom", skip_serializing_if = "Option::is_none")]
    pub resp_from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_cost: Option<DouyinSuggestWordsTimeCost>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSuggestWords {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<DouyinSuggestWordsDataItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errno: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<DouyinSuggestWordsExtra>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub real_log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
    #[serde(
        default,
        rename = "StabilityStatistics",
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub stability_statistics: BTreeMap<String, Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
