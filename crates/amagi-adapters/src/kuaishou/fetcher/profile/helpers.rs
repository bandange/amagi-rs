use serde_json::{Map, Value};

use super::super::{
    super::types::{
        KuaishouCategoryMask, KuaishouUserProfile, KuaishouUserProfileAuthor,
        KuaishouUserProfilePage, KuaishouUserProfilePublicTabData, KuaishouUserProfileTabData,
        KuaishouUserProfileUserInfo, empty_kuaishou_banned_status, empty_kuaishou_verified_status,
    },
    support::{empty_object, i64_value},
};

pub(super) const PROFILE_TAB_TYPE_MAP: [(&str, &str); 4] = [
    ("public", "public"),
    ("private", "private"),
    ("liked", "liked"),
    ("playback", "playback"),
];

pub(super) const BAN_STATE_MAP: [(&str, &str); 4] = [
    ("banned", "BANNED"),
    ("socialBanned", "SOCIALBANNED"),
    ("isolate", "ISOLATE"),
    ("cleanState", "CLEAN"),
];

pub(super) fn create_empty_tab() -> KuaishouUserProfileTabData {
    KuaishouUserProfileTabData {
        list: Vec::new(),
        pcursor: String::new(),
    }
}

pub(super) fn create_empty_public_tab() -> KuaishouUserProfilePublicTabData {
    KuaishouUserProfilePublicTabData {
        live: None,
        list: Vec::new(),
        pcursor: String::new(),
    }
}

pub(super) fn create_empty_user_info() -> KuaishouUserProfileUserInfo {
    KuaishouUserProfileUserInfo {
        id: String::new(),
        name: String::new(),
        description: String::new(),
        avatar: String::new(),
        sex: String::new(),
        living: false,
        follow_status: String::new(),
        constellation: String::new(),
        city_name: String::new(),
        origin_user_id: 0,
        privacy: false,
        is_new: false,
        timestamp: 0,
        verified_status: empty_kuaishou_verified_status(),
        banned_status: empty_kuaishou_banned_status(),
        counts: empty_object(),
    }
}

pub(super) fn create_empty_user_profile(principal_id: &str) -> KuaishouUserProfile {
    KuaishouUserProfile {
        principal_id: principal_id.to_owned(),
        author: KuaishouUserProfileAuthor {
            principal_id: principal_id.to_owned(),
            user_info: create_empty_user_info(),
            sensitive_info: None,
            follow_info: empty_object(),
            ban_state_map: BAN_STATE_MAP
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect(),
        },
        profile: KuaishouUserProfilePage {
            current_tab: "public".to_owned(),
            page_size: 12,
            tab_type_map: PROFILE_TAB_TYPE_MAP
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value.to_owned()))
                .collect(),
            show_playback: false,
            public_data: create_empty_public_tab(),
            private_data: create_empty_tab(),
            liked_data: create_empty_tab(),
            playback_data: create_empty_tab(),
            interest_list: Vec::new(),
            current_product: empty_object(),
        },
        follow: None,
        follow_button: None,
        interest_mask: Vec::new(),
        category_mask: KuaishouCategoryMask {
            config: Vec::new(),
            list: Vec::new(),
            hot_list: Vec::new(),
            has_more: false,
            has_more_hot: false,
        },
        upstream_payload: Value::Null,
    }
}

pub(super) fn resolve_live_detail_data(payload: Option<&Value>) -> Option<&Map<String, Value>> {
    let data = payload
        .and_then(|value| super::super::support::value_field(value, "data"))?
        .as_object()?;
    (i64_value(data.get("result")).unwrap_or_default() == 1).then_some(data)
}
