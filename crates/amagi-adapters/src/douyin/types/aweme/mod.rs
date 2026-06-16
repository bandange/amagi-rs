use serde::{Deserialize, Serialize};

use super::{
    common::{
        DouyinAwemeControl, DouyinCommentPermissionInfo, DouyinExtraFields, DouyinImageUrl,
        DouyinInlineSuggestWords, DouyinResponseMeta, DouyinSearchImpression, DouyinShareInfo,
        DouyinStatistics, DouyinStatus, DouyinTextExtra, deserialize_null_default_string_vec,
        deserialize_null_default_vec,
    },
    user::DouyinUser,
};

mod media;
mod music;

pub use media::{
    DouyinDanmakuControl, DouyinVideo, DouyinVideoBigThumb, DouyinVideoBitRate, DouyinVideoControl,
    DouyinVideoControlLevelInfo, DouyinVideoTag,
};
pub use music::DouyinMusic;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinImpressionData {
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_string_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_id_list_a: Vec<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_string_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_id_list_b: Vec<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_string_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub group_id_list_c: Vec<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinAweme {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_digged: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_ads: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_story: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_top: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<DouyinUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aweme_control: Option<DouyinAwemeControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_permission_info: Option<DouyinCommentPermissionInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub danmaku_control: Option<DouyinDanmakuControl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impression_data: Option<DouyinImpressionData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music: Option<DouyinMusic>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_info: Option<DouyinShareInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DouyinStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<DouyinStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_words: Option<DouyinInlineSuggestWords>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub text_extra: Vec<DouyinTextExtra>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<DouyinVideo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_control: Option<DouyinVideoControl>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub video_tag: Vec<DouyinVideoTag>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub image_list: Vec<DouyinImageUrl>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub images: Vec<DouyinImageUrl>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinWorkDetail {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(
        default,
        rename = "aweme_detail",
        skip_serializing_if = "Option::is_none"
    )]
    pub aweme_detail: Option<DouyinAweme>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinUserAwemeList {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(
        default,
        rename = "aweme_list",
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub aweme_list: Vec<DouyinAweme>,
    #[serde(default, rename = "max_cursor")]
    pub max_cursor: String,
    #[serde(default)]
    pub has_more: i64,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

pub type DouyinParsedWork = DouyinWorkDetail;
pub type DouyinVideoWork = DouyinWorkDetail;
pub type DouyinImageAlbumWork = DouyinWorkDetail;
pub type DouyinSlidesWork = DouyinWorkDetail;
pub type DouyinTextWork = DouyinWorkDetail;
pub type DouyinUserVideoList = DouyinUserAwemeList;
pub type DouyinUserFavoriteList = DouyinUserAwemeList;
pub type DouyinUserRecommendList = DouyinUserAwemeList;
