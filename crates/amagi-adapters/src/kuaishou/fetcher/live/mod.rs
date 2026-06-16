use serde_json::{Map, Value};

use self::detail::{
    dedupe_live_room_play_list, empty_live_room_info, map_live_detail_to_live_room_play_item,
    map_reco_item_to_live_room_play_item, resolve_live_detail_data,
    resolve_live_detail_recommend_list, resolve_live_detail_websocket_meta,
};
use super::{
    super::types::{KuaishouEmojiList, KuaishouLiveRoomEmojiState, KuaishouLiveRoomInfo},
    profile::{KuaishouUserProfileSources, build_kuaishou_user_profile},
};

mod detail;

#[derive(Debug)]
pub(crate) struct KuaishouLiveRoomSources<'a> {
    pub user_info_payload: Option<&'a Value>,
    pub sensitive_payload: Option<&'a Value>,
    pub live_detail_payload: &'a Value,
    pub emoji_payload: Option<&'a KuaishouEmojiList>,
    pub gift_payload: Option<&'a Value>,
    pub websocket_payload: Option<&'a Value>,
    pub reco_payload: Option<&'a Value>,
}

pub(crate) fn build_kuaishou_live_room_info(
    principal_id: &str,
    sources: KuaishouLiveRoomSources<'_>,
) -> KuaishouLiveRoomInfo {
    let mut fallback = empty_live_room_info(principal_id);
    fallback.upstream_payload = build_live_room_upstream_payload(&sources);
    let Some(detail_data) = resolve_live_detail_data(sources.live_detail_payload) else {
        return fallback;
    };

    let empty_user_info_payload = Value::Object(Map::new());
    let profile = build_kuaishou_user_profile(
        principal_id,
        KuaishouUserProfileSources {
            user_info_payload: sources
                .user_info_payload
                .unwrap_or(&empty_user_info_payload),
            sensitive_payload: sources.sensitive_payload,
            public_payload: None,
            private_payload: None,
            liked_payload: None,
            playback_payload: None,
            interest_list_payload: None,
            interest_mask_payload: None,
            category_config_payload: None,
            category_data_payload: None,
            category_classify_payload: None,
            live_detail_payload: Some(sources.live_detail_payload),
        },
    );

    let current_author = profile.author.user_info;
    let current_item = map_live_detail_to_live_room_play_item(detail_data, current_author);
    let live_stream_id = current_item.live_stream.id.clone();
    let detail_websocket_meta = resolve_live_detail_websocket_meta(detail_data);
    let detail_recommend_list = resolve_live_detail_recommend_list(detail_data);
    let resolved_recommend_list = sources
        .reco_payload
        .and_then(|payload| payload.get("data"))
        .and_then(|value| value.get("list"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or(detail_recommend_list);
    let reco_play_list = resolved_recommend_list
        .iter()
        .filter_map(Value::as_object)
        .map(map_reco_item_to_live_room_play_item)
        .collect::<Vec<_>>();
    let mut play_list = dedupe_live_room_play_list(
        std::iter::once(current_item.clone())
            .chain(reco_play_list)
            .collect(),
    );

    if play_list.is_empty() {
        play_list.push(current_item.clone());
    }

    let websocket_urls = sources
        .websocket_payload
        .and_then(|payload| payload.get("data"))
        .and_then(|value| value.get("websocketUrls"))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| detail_websocket_meta.websocket_urls.clone());
    let token = sources
        .websocket_payload
        .and_then(|payload| payload.get("data"))
        .and_then(|value| value.get("token"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .unwrap_or(detail_websocket_meta.token);
    let notice_list = detail_data
        .get("noticeList")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let emoji = KuaishouLiveRoomEmojiState {
        icon_urls: sources
            .emoji_payload
            .map(|payload| payload.data.vision_base_emoticons.icon_urls.clone())
            .unwrap_or_default(),
        gift_list: sources
            .gift_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| value.get("gifts"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        gift_panel_list: sources
            .gift_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| value.get("giftPanelList"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        token: sources
            .gift_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| value.get("token"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        panel_token: sources
            .gift_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| value.get("panelToken"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        long_send_gift_type: sources
            .gift_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| value.get("longSendGiftType"))
            .cloned(),
    };

    if live_stream_id.is_empty() {
        return KuaishouLiveRoomInfo {
            principal_id: principal_id.to_owned(),
            active_index: 0,
            current: Some(current_item),
            play_list,
            websocket_urls,
            token,
            notice_list,
            loading: false,
            emoji,
            upstream_payload: build_live_room_upstream_payload(&sources),
        };
    }

    KuaishouLiveRoomInfo {
        principal_id: principal_id.to_owned(),
        active_index: 0,
        current: Some(current_item),
        play_list,
        websocket_urls,
        token,
        notice_list,
        loading: false,
        emoji,
        upstream_payload: build_live_room_upstream_payload(&sources),
    }
}

fn build_live_room_upstream_payload(sources: &KuaishouLiveRoomSources<'_>) -> Value {
    let mut object = Map::new();
    insert_source_payload(&mut object, "user_info", sources.user_info_payload);
    insert_source_payload(&mut object, "sensitive", sources.sensitive_payload);
    object.insert(
        "live_detail".to_owned(),
        super::value::unwrap_data_payload(sources.live_detail_payload),
    );
    if let Some(emoji) = sources.emoji_payload {
        object.insert("emoji".to_owned(), emoji.upstream_payload.clone());
    }
    insert_source_payload(&mut object, "gift", sources.gift_payload);
    insert_source_payload(&mut object, "websocket", sources.websocket_payload);
    insert_source_payload(&mut object, "reco", sources.reco_payload);
    Value::Object(object)
}

fn insert_source_payload(object: &mut Map<String, Value>, key: &str, payload: Option<&Value>) {
    if let Some(payload) = payload {
        object.insert(key.to_owned(), super::value::unwrap_data_payload(payload));
    }
}
