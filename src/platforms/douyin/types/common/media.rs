use serde::{Deserialize, Serialize};

use super::{DouyinExtraFields, deserialize_null_default_vec};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinImageUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub url_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_cs: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinShareInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_desc_info: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_link_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_title_myself: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_title_other: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_weibo_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_quote: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_signature_desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_signature_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_image_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_qrcode_url: Option<DouyinImageUrl>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinSearchImpression {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
