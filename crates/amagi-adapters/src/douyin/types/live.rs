use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    common::{DouyinExtraFields, DouyinResponseMeta},
    user::DouyinUser,
};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinLiveWebStreamUrl {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_resolution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hls_pull_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_orientation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flv_pull_url: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hls_pull_url_map: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_datas: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinLiveRoomData {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qrcode_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room_status: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub similar_rooms: Vec<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<DouyinUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_stream_url: Option<DouyinLiveWebStreamUrl>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinLiveRoomInfo {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<DouyinLiveRoomData>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}
