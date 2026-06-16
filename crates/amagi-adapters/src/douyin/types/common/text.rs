use serde::{Deserialize, Serialize};

use super::{DouyinExtraFields, deserialize_null_default_vec};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinTextExtra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_start: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption_end: Option<u64>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashtag_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashtag_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sec_uid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_commerce: Option<bool>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinInlineSuggestWord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_id: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinInlineSuggestWordEntry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hint_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub words: Vec<DouyinInlineSuggestWord>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinInlineSuggestWords {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_display_bar_inner: Option<i64>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub suggest_words: Vec<DouyinInlineSuggestWordEntry>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
