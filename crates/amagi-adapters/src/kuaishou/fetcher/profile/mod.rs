use serde_json::{Map, Value};

use super::super::types::{KuaishouCategoryMask, KuaishouUserProfile};

mod author;
mod detail;
mod helpers;
mod tabs;

use self::{
    author::{create_follow_button_state, create_follow_state, merge_live_author},
    detail::map_live_detail_to_user_profile_live_info,
    helpers::{create_empty_user_profile, resolve_live_detail_data},
    tabs::{resolve_public_show_playback, resolve_public_tab_data, resolve_tab_data},
};

#[derive(Debug)]
pub(crate) struct KuaishouUserProfileSources<'a> {
    pub user_info_payload: &'a Value,
    pub sensitive_payload: Option<&'a Value>,
    pub public_payload: Option<&'a Value>,
    pub private_payload: Option<&'a Value>,
    pub liked_payload: Option<&'a Value>,
    pub playback_payload: Option<&'a Value>,
    pub interest_list_payload: Option<&'a Value>,
    pub interest_mask_payload: Option<&'a Value>,
    pub category_config_payload: Option<&'a Value>,
    pub category_data_payload: Option<&'a Value>,
    pub category_classify_payload: Option<&'a Value>,
    pub live_detail_payload: Option<&'a Value>,
}

pub(crate) fn build_kuaishou_user_profile(
    principal_id: &str,
    sources: KuaishouUserProfileSources<'_>,
) -> KuaishouUserProfile {
    let mut profile = create_empty_user_profile(principal_id);
    let user_info = sources
        .user_info_payload
        .get("data")
        .and_then(|value| super::support::value_field(value, "userInfo"));
    let sensitive_info = sources
        .sensitive_payload
        .and_then(|payload| payload.get("data"))
        .and_then(|value| super::support::value_field(value, "sensitiveUserInfo"));
    let live_detail_data = resolve_live_detail_data(sources.live_detail_payload);
    let normalized_author = merge_live_author(
        live_detail_data.and_then(|value| value.get("author")),
        user_info,
        sensitive_info,
    );
    let mut public_data =
        resolve_public_tab_data(sources.public_payload, &profile.profile.public_data);
    let private_data = resolve_tab_data(sources.private_payload, &profile.profile.private_data);
    let liked_data = resolve_tab_data(sources.liked_payload, &profile.profile.liked_data);
    let playback_data = resolve_tab_data(sources.playback_payload, &profile.profile.playback_data);

    if public_data.live.is_none() {
        if let Some(detail_data) = live_detail_data {
            public_data.live = Some(map_live_detail_to_user_profile_live_info(
                detail_data,
                &normalized_author,
            ));
        }
    }

    profile.author.user_info = normalized_author;
    profile.author.sensitive_info = sensitive_info.cloned();
    profile.profile.show_playback = resolve_public_show_playback(sources.public_payload)
        .unwrap_or(!playback_data.list.is_empty());
    profile.profile.public_data = public_data;
    profile.profile.private_data = private_data;
    profile.profile.liked_data = liked_data;
    profile.profile.playback_data = playback_data;
    profile.profile.interest_list = sources
        .interest_list_payload
        .and_then(|payload| payload.get("data"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    profile.follow = create_follow_state(user_info, sensitive_info);
    profile.follow_button = create_follow_button_state(user_info, sensitive_info);
    profile.interest_mask = sources
        .interest_mask_payload
        .and_then(|payload| payload.get("data"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    profile.category_mask = KuaishouCategoryMask {
        config: sources
            .category_config_payload
            .and_then(|payload| payload.get("data"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        list: sources
            .category_classify_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| super::support::value_field(value, "list"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        hot_list: sources
            .category_data_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| super::support::value_field(value, "list"))
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default(),
        has_more: sources
            .category_classify_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| super::support::value_field(value, "hasMore"))
            .and_then(Value::as_bool)
            .unwrap_or(false),
        has_more_hot: sources
            .category_data_payload
            .and_then(|payload| payload.get("data"))
            .and_then(|value| super::support::value_field(value, "hasMore"))
            .and_then(Value::as_bool)
            .unwrap_or(false),
    };
    profile.upstream_payload = build_profile_upstream_payload(&sources);

    profile
}

fn build_profile_upstream_payload(sources: &KuaishouUserProfileSources<'_>) -> Value {
    let mut object = Map::new();
    object.insert(
        "user_info".to_owned(),
        super::value::unwrap_data_payload(sources.user_info_payload),
    );
    insert_source_payload(&mut object, "sensitive", sources.sensitive_payload);
    insert_source_payload(&mut object, "public", sources.public_payload);
    insert_source_payload(&mut object, "private", sources.private_payload);
    insert_source_payload(&mut object, "liked", sources.liked_payload);
    insert_source_payload(&mut object, "playback", sources.playback_payload);
    insert_source_payload(&mut object, "interest_list", sources.interest_list_payload);
    insert_source_payload(&mut object, "interest_mask", sources.interest_mask_payload);
    insert_source_payload(
        &mut object,
        "category_config",
        sources.category_config_payload,
    );
    insert_source_payload(&mut object, "category_data", sources.category_data_payload);
    insert_source_payload(
        &mut object,
        "category_classify",
        sources.category_classify_payload,
    );
    insert_source_payload(&mut object, "live_detail", sources.live_detail_payload);

    Value::Object(object)
}

fn insert_source_payload(object: &mut Map<String, Value>, key: &str, payload: Option<&Value>) {
    if let Some(payload) = payload {
        object.insert(key.to_owned(), super::value::unwrap_data_payload(payload));
    }
}
