use serde::{Deserialize, Serialize};

use super::super::common::{DouyinExtraFields, DouyinImageUrl, deserialize_null_default_vec};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideoBitRate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, rename = "FPS", skip_serializing_if = "Option::is_none")]
    pub fps: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gear_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_bytevc1: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_h265: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_addr: Option<DouyinImageUrl>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideoBigThumb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fext: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_num: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub img_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_x_len: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_x_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_y_len: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub img_y_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub uris: Vec<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideoControlLevelInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideoControl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_download: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_duet: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_music: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_react: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_record: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_share: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_stitch: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_record_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prevent_download_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timer_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_info: Option<DouyinVideoControlLevelInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duet_info: Option<DouyinVideoControlLevelInfo>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_watermark: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_cover: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gaussian_cover: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_cover: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_cover: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_addr: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_addr_265: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub play_addr_h264: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_addr: Option<DouyinImageUrl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_suffix_logo_addr: Option<DouyinImageUrl>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub bit_rate: Vec<DouyinVideoBitRate>,
    #[serde(
        default,
        deserialize_with = "deserialize_null_default_vec",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub big_thumbs: Vec<DouyinVideoBigThumb>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinVideoTag {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinDanmakuControl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub danmaku_cnt: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_danmaku: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_post_denied: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_denied_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_privilege_level: Option<i64>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
