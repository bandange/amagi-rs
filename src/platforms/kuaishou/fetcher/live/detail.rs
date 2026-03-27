use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Map, Value};

use super::super::{
    super::types::{
        KuaishouLiveRoomEmojiState, KuaishouLiveRoomGameInfo, KuaishouLiveRoomInfo,
        KuaishouLiveRoomPlayItem, KuaishouLiveStreamInfo, KuaishouUserProfileUserInfo,
        empty_kuaishou_banned_status, empty_kuaishou_verified_status,
    },
    support::{bool_value, number_value, pick_first_non_empty_string, string_value},
};

pub(super) fn empty_live_room_info(principal_id: &str) -> KuaishouLiveRoomInfo {
    KuaishouLiveRoomInfo {
        principal_id: principal_id.to_owned(),
        active_index: 0,
        current: None,
        play_list: Vec::new(),
        websocket_urls: Vec::new(),
        token: String::new(),
        notice_list: Vec::new(),
        loading: false,
        emoji: KuaishouLiveRoomEmojiState {
            icon_urls: BTreeMap::new(),
            gift_list: Vec::new(),
            gift_panel_list: Vec::new(),
            token: String::new(),
            panel_token: String::new(),
            long_send_gift_type: None,
        },
        upstream_payload: Value::Null,
    }
}

pub(super) fn resolve_live_detail_data(payload: &Value) -> Option<&Map<String, Value>> {
    let data = payload.get("data")?.as_object()?;
    (number_value(data.get("result")).unwrap_or_default() == 1).then_some(data)
}

#[derive(Debug, Clone)]
pub(super) struct KuaishouLiveDetailWebsocketMeta {
    pub websocket_urls: Vec<String>,
    pub token: String,
}

pub(super) fn resolve_live_detail_websocket_meta(
    detail_data: &Map<String, Value>,
) -> KuaishouLiveDetailWebsocketMeta {
    let websocket_info = detail_data
        .get("websocketInfo")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();

    KuaishouLiveDetailWebsocketMeta {
        websocket_urls: websocket_info
            .get("websocketUrls")
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
        token: websocket_info
            .get("token")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
    }
}

pub(super) fn resolve_live_detail_recommend_list(detail_data: &Map<String, Value>) -> Vec<Value> {
    detail_data
        .get("recommendList")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

pub(super) fn map_live_detail_to_live_room_play_item(
    detail_data: &Map<String, Value>,
    author: KuaishouUserProfileUserInfo,
) -> KuaishouLiveRoomPlayItem {
    let live_stream = detail_data
        .get("liveStream")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let config = detail_data
        .get("config")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let game_info = detail_data
        .get("gameInfo")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let live_stream_id = pick_first_non_empty_string(&[
        string_value(live_stream.get("id")),
        string_value(config.get("liveStreamId")),
    ]);
    let cover_url = pick_first_non_empty_string(&[
        string_value(config.get("coverUrl")),
        string_value(config.get("rtCoverUrl")),
        string_value(live_stream.get("poster")),
    ]);
    let hls_play_url = pick_first_non_empty_string(&[
        string_value(config.get("hlsPlayUrl")),
        string_value(live_stream.get("hlsPlayUrl")),
    ]);
    let play_urls = live_stream
        .get("playUrls")
        .cloned()
        .or_else(|| config.get("multiResolutionPlayUrls").cloned())
        .unwrap_or_else(|| Value::Object(Map::new()));
    let mut config_value = Value::Object(config.clone());

    if let Some(config_object) = config_value.as_object_mut() {
        config_object.insert(
            "liveStreamId".to_owned(),
            Value::String(live_stream_id.clone()),
        );
        config_object.insert("hlsPlayUrl".to_owned(), Value::String(hls_play_url.clone()));
        config_object.insert("coverUrl".to_owned(), Value::String(cover_url.clone()));
        config_object.insert(
            "rtCoverUrl".to_owned(),
            Value::String(
                string_value(config.get("rtCoverUrl"))
                    .or_else(|| string_value(live_stream.get("poster")))
                    .unwrap_or_default(),
            ),
        );
        config_object.insert("gameInfo".to_owned(), Value::Object(game_info.clone()));
        config_object.insert("multiResolutionPlayUrls".to_owned(), play_urls.clone());
    }

    KuaishouLiveRoomPlayItem {
        live_stream: KuaishouLiveStreamInfo {
            id: live_stream_id.clone(),
            poster: pick_first_non_empty_string(&[
                string_value(live_stream.get("poster")),
                Some(cover_url.clone()),
            ]),
            play_urls,
            url: string_value(live_stream.get("url")).unwrap_or_default(),
            hls_play_url,
            location: string_value(live_stream.get("location")),
            stream_type: string_value(live_stream.get("type")).unwrap_or_else(|| "live".to_owned()),
            live_guess: bool_value(live_stream.get("liveGuess")).unwrap_or(false),
            exp_tag: pick_first_non_empty_string(&[
                string_value(live_stream.get("expTag")),
                string_value(config.get("expTag")),
            ]),
            private_live: bool_value(live_stream.get("privateLive"))
                .or_else(|| bool_value(config.get("privateLive")))
                .unwrap_or(false),
        },
        author,
        game_info: KuaishouLiveRoomGameInfo {
            id: pick_first_non_empty_string(&[
                string_value(game_info.get("id")),
                string_value(game_info.get("gameId")),
            ]),
            name: string_value(game_info.get("name")).unwrap_or_default(),
            poster: pick_first_non_empty_string(&[
                string_value(game_info.get("poster")),
                string_value(game_info.get("coverUrl")),
            ]),
            description: string_value(game_info.get("description")).unwrap_or_default(),
            category_abbr: pick_first_non_empty_string(&[
                string_value(game_info.get("categoryAbbr")),
                string_value(game_info.get("category")),
            ]),
            category_name: string_value(game_info.get("categoryName")).unwrap_or_default(),
            watching_count: pick_first_non_empty_string(&[
                string_value(config.get("watchingCount")),
                string_value(detail_data.get("watchingCount")),
                string_value(game_info.get("watchingCount")),
            ]),
            room_count: string_value(game_info.get("roomCount")).unwrap_or_default(),
        },
        is_living: bool_value(detail_data.get("isLiving")).unwrap_or(!live_stream_id.is_empty()),
        auth_token: string_value(detail_data.get("authToken")),
        config: config_value,
        websocket_info: detail_data
            .get("websocketInfo")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new())),
        status: detail_data.get("status").cloned().unwrap_or_else(|| {
            let mut status = Map::new();
            status.insert(
                "forbiddenState".to_owned(),
                Value::Number(number_value(detail_data.get("result")).unwrap_or(0).into()),
            );
            Value::Object(status)
        }),
    }
}

pub(super) fn map_reco_item_to_live_room_play_item(
    reco_item: &Map<String, Value>,
) -> KuaishouLiveRoomPlayItem {
    let live_stream = reco_item
        .get("liveStream")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let config = reco_item
        .get("config")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let game_info = reco_item
        .get("gameInfo")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut config_value = Value::Object(config.clone());

    if let Some(config_object) = config_value.as_object_mut() {
        config_object.insert(
            "liveStreamId".to_owned(),
            Value::String(
                string_value(config.get("liveStreamId"))
                    .or_else(|| string_value(live_stream.get("id")))
                    .unwrap_or_default(),
            ),
        );
    }

    KuaishouLiveRoomPlayItem {
        live_stream: KuaishouLiveStreamInfo {
            id: string_value(live_stream.get("id")).unwrap_or_default(),
            poster: pick_first_non_empty_string(&[
                string_value(live_stream.get("poster")),
                string_value(config.get("coverUrl")),
            ]),
            play_urls: live_stream
                .get("playUrls")
                .cloned()
                .unwrap_or_else(|| Value::Object(Map::new())),
            url: string_value(live_stream.get("url")).unwrap_or_default(),
            hls_play_url: pick_first_non_empty_string(&[
                string_value(live_stream.get("hlsPlayUrl")),
                string_value(config.get("hlsPlayUrl")),
            ]),
            location: string_value(live_stream.get("location")),
            stream_type: string_value(live_stream.get("type")).unwrap_or_else(|| "live".to_owned()),
            live_guess: bool_value(live_stream.get("liveGuess")).unwrap_or(false),
            exp_tag: string_value(live_stream.get("expTag")).unwrap_or_default(),
            private_live: bool_value(live_stream.get("privateLive"))
                .or_else(|| bool_value(config.get("privateLive")))
                .unwrap_or(false),
        },
        author: normalize_reco_author(reco_item.get("author")),
        game_info: KuaishouLiveRoomGameInfo {
            id: string_value(game_info.get("id")).unwrap_or_default(),
            name: string_value(game_info.get("name")).unwrap_or_default(),
            poster: string_value(game_info.get("poster")).unwrap_or_default(),
            description: string_value(game_info.get("description")).unwrap_or_default(),
            category_abbr: string_value(game_info.get("categoryAbbr")).unwrap_or_default(),
            category_name: string_value(game_info.get("categoryName")).unwrap_or_default(),
            watching_count: string_value(game_info.get("watchingCount")).unwrap_or_default(),
            room_count: string_value(game_info.get("roomCount")).unwrap_or_default(),
        },
        is_living: bool_value(reco_item.get("isLiving")).unwrap_or(false),
        auth_token: string_value(reco_item.get("authToken")),
        config: config_value,
        websocket_info: reco_item
            .get("websocketInfo")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new())),
        status: reco_item
            .get("status")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new())),
    }
}

fn normalize_reco_author(author: Option<&Value>) -> KuaishouUserProfileUserInfo {
    let object = author.and_then(Value::as_object);

    KuaishouUserProfileUserInfo {
        id: pick_first_non_empty_string(&[
            string_value(object.and_then(|value| value.get("id"))),
            string_value(object.and_then(|value| value.get("principalId"))),
            string_value(object.and_then(|value| value.get("kwaiId"))),
        ]),
        name: pick_first_non_empty_string(&[
            string_value(object.and_then(|value| value.get("name"))),
            string_value(object.and_then(|value| value.get("user_name"))),
        ]),
        description: pick_first_non_empty_string(&[
            string_value(object.and_then(|value| value.get("description"))),
            string_value(object.and_then(|value| value.get("user_text"))),
        ]),
        avatar: pick_first_non_empty_string(&[
            string_value(object.and_then(|value| value.get("avatar"))),
            string_value(object.and_then(|value| value.get("headurl"))),
        ]),
        sex: pick_first_non_empty_string(&[
            string_value(object.and_then(|value| value.get("sex"))),
            string_value(object.and_then(|value| value.get("user_sex"))),
        ]),
        living: bool_value(object.and_then(|value| value.get("living")))
            .or_else(|| bool_value(object.and_then(|value| value.get("live"))))
            .unwrap_or(false),
        follow_status: string_value(object.and_then(|value| value.get("followStatus")))
            .unwrap_or_else(|| {
                if bool_value(object.and_then(|value| value.get("following"))).unwrap_or(false) {
                    "FOLLOWING".to_owned()
                } else {
                    "UN_FOLLOWED".to_owned()
                }
            }),
        constellation: string_value(object.and_then(|value| value.get("constellation")))
            .unwrap_or_default(),
        city_name: string_value(object.and_then(|value| value.get("cityName"))).unwrap_or_default(),
        origin_user_id: number_value(object.and_then(|value| value.get("originUserId")))
            .or_else(|| number_value(object.and_then(|value| value.get("user_id"))))
            .unwrap_or(0),
        privacy: bool_value(object.and_then(|value| value.get("privacy"))).unwrap_or(false),
        is_new: bool_value(object.and_then(|value| value.get("isNew"))).unwrap_or(false),
        timestamp: number_value(object.and_then(|value| value.get("timestamp"))).unwrap_or(0),
        verified_status: object
            .and_then(|value| value.get("verifiedStatus"))
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
            .unwrap_or_else(empty_kuaishou_verified_status),
        banned_status: object
            .and_then(|value| value.get("bannedStatus"))
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
            .unwrap_or_else(empty_kuaishou_banned_status),
        counts: object
            .and_then(|value| value.get("counts"))
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new())),
    }
}

pub(super) fn dedupe_live_room_play_list(
    items: Vec<KuaishouLiveRoomPlayItem>,
) -> Vec<KuaishouLiveRoomPlayItem> {
    let mut seen_live_stream_ids = BTreeSet::new();
    let mut normalized_items = Vec::new();

    for item in items {
        let live_stream_id = item.live_stream.id.clone();

        if !live_stream_id.is_empty() && !seen_live_stream_ids.insert(live_stream_id) {
            continue;
        }

        normalized_items.push(item);
    }

    normalized_items
}
