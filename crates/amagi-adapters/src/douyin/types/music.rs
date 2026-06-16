use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    aweme::DouyinMusic,
    common::{DouyinExtraFields, DouyinResponseMeta},
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinMusicInfo {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub music_info: Option<DouyinMusic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rec_list: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_effects: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_musics: Option<Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub small_banner: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trends_infos: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
