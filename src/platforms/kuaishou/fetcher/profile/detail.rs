use serde_json::{Map, Value};

use super::super::{
    super::types::KuaishouUserProfileUserInfo,
    support::{bool_value, number_value, pick_first_non_empty_string, string_value},
};

pub(super) fn map_live_detail_to_user_profile_live_info(
    detail_data: &Map<String, Value>,
    author: &KuaishouUserProfileUserInfo,
) -> Value {
    let live_stream = detail_data.get("liveStream").and_then(Value::as_object);
    let config = detail_data.get("config").and_then(Value::as_object);
    let game_info = detail_data
        .get("gameInfo")
        .cloned()
        .unwrap_or_else(super::super::support::empty_object);
    let live_stream_id = pick_first_non_empty_string(&[
        live_stream.and_then(|value| string_value(value.get("id"))),
        config.and_then(|value| string_value(value.get("liveStreamId"))),
    ]);
    let poster = pick_first_non_empty_string(&[
        live_stream.and_then(|value| string_value(value.get("poster"))),
        config.and_then(|value| string_value(value.get("coverUrl"))),
        config.and_then(|value| string_value(value.get("rtCoverUrl"))),
    ]);
    let play_urls = live_stream
        .and_then(|value| value.get("playUrls"))
        .cloned()
        .or_else(|| {
            config
                .and_then(|value| value.get("multiResolutionPlayUrls"))
                .cloned()
        })
        .unwrap_or_else(|| Value::Array(Vec::new()));
    let caption = pick_first_non_empty_string(&[
        config.and_then(|value| string_value(value.get("caption"))),
        string_value(detail_data.get("caption")),
    ]);
    let start_time = number_value(detail_data.get("startTime"))
        .or_else(|| config.and_then(|value| number_value(value.get("startTime"))))
        .or_else(|| live_stream.and_then(|value| number_value(value.get("startTime"))))
        .unwrap_or(0);
    let has_red_pack = bool_value(detail_data.get("hasRedPack"))
        .or_else(|| config.and_then(|value| bool_value(value.get("hasRedPack"))))
        .unwrap_or(false);
    let has_bet = bool_value(detail_data.get("hasBet"))
        .or_else(|| config.and_then(|value| bool_value(value.get("hasBet"))))
        .unwrap_or(false);
    let exp_tag = pick_first_non_empty_string(&[
        live_stream.and_then(|value| string_value(value.get("expTag"))),
        config.and_then(|value| string_value(value.get("expTag"))),
    ]);
    let hot_icon = pick_first_non_empty_string(&[
        config.and_then(|value| string_value(value.get("hotIcon"))),
        string_value(detail_data.get("hotIcon")),
    ]);
    let living = bool_value(detail_data.get("isLiving")).unwrap_or(!live_stream_id.is_empty());
    let quality = config
        .and_then(|value| string_value(value.get("quality")))
        .unwrap_or_default();
    let quality_label = config
        .and_then(|value| string_value(value.get("qualityLabel")))
        .unwrap_or_default();
    let watching_count = config
        .and_then(|value| value.get("watchingCount").cloned())
        .or_else(|| detail_data.get("watchingCount").cloned())
        .or_else(|| {
            detail_data
                .get("gameInfo")
                .and_then(Value::as_object)
                .and_then(|value| value.get("watchingCount").cloned())
        })
        .unwrap_or(Value::String(String::new()));
    let landscape = config
        .and_then(|value| bool_value(value.get("landscape")))
        .unwrap_or(false);
    let like_count = config
        .and_then(|value| value.get("likeCount").cloned())
        .or_else(|| detail_data.get("likeCount").cloned())
        .unwrap_or(Value::String(String::new()));
    let live_type = live_stream
        .and_then(|value| string_value(value.get("type")))
        .unwrap_or_else(|| "live".to_owned());

    let mut value = detail_data.clone();
    value.insert("id".to_owned(), Value::String(live_stream_id));
    value.insert("poster".to_owned(), Value::String(poster));
    value.insert("playUrls".to_owned(), play_urls);
    value.insert("caption".to_owned(), Value::String(caption));
    value.insert("statrtTime".to_owned(), Value::Number(start_time.into()));
    value.insert(
        "author".to_owned(),
        serde_json::to_value(author).unwrap_or_else(|_| super::super::support::empty_object()),
    );
    value.insert("gameInfo".to_owned(), game_info);
    value.insert("hasRedPack".to_owned(), Value::Bool(has_red_pack));
    value.insert("hasBet".to_owned(), Value::Bool(has_bet));
    value.insert(
        "followed".to_owned(),
        Value::Bool(author.follow_status == "FOLLOWING"),
    );
    value.insert("expTag".to_owned(), Value::String(exp_tag));
    value.insert("hotIcon".to_owned(), Value::String(hot_icon));
    value.insert("living".to_owned(), Value::Bool(living));
    value.insert("quality".to_owned(), Value::String(quality));
    value.insert("qualityLabel".to_owned(), Value::String(quality_label));
    value.insert("watchingCount".to_owned(), watching_count);
    value.insert("landscape".to_owned(), Value::Bool(landscape));
    value.insert("likeCount".to_owned(), like_count);
    value.insert("type".to_owned(), Value::String(live_type));

    Value::Object(value)
}
