use serde::{Deserialize, Serialize};

use super::{
    common::{
        DouyinExtraFields, DouyinImageUrl, DouyinResponseMeta, DouyinTextExtra,
        deserialize_null_default_vec,
    },
    user::DouyinUser,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinCommentLabel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinCommentImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crop_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub medium_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_url: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<DouyinImageUrl>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinFastResponseComment {
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub constant_response_words: Vec<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub timed_response_words: Vec<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinComment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_comment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_reply_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_user_sec_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_userid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_reply_total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_comment_total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_digged: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_share: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_author_digged: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_folded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<DouyinUser>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub text_extra: Vec<DouyinTextExtra>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_list: Vec<DouyinCommentImage>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub label_list: Vec<DouyinCommentLabel>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub reply_comment: Vec<DouyinComment>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinCommentPage {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub comments: Vec<DouyinComment>,
    #[serde(default)]
    pub cursor: u64,
    #[serde(default)]
    pub has_more: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fast_response_comment: Option<DouyinFastResponseComment>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

pub type DouyinWorkComments = DouyinCommentPage;
pub type DouyinCommentReplies = DouyinCommentPage;
