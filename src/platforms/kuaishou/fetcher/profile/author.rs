use serde_json::Value;

use super::super::{
    super::types::{
        KuaishouBannedStatus, KuaishouFollowButtonState, KuaishouFollowState,
        KuaishouUserProfileUserInfo, KuaishouVerifiedStatus,
    },
    support::{
        as_object, bool_value, i64_value, is_populated_object, number_value, object_or_empty,
        pick_first_non_empty_string, string_value, value_from_object,
    },
};

fn verified_status_from_value(
    value: Option<&Value>,
    verified_detail: Option<&Value>,
) -> KuaishouVerifiedStatus {
    let value_object = as_object(value);
    let detail_object = as_object(verified_detail);

    KuaishouVerifiedStatus {
        verified: bool_value(value_from_object(value_object, "verified"))
            .or_else(|| bool_value(value_from_object(detail_object, "verified")))
            .or_else(|| bool_value(value_from_object(detail_object, "newVerified")))
            .unwrap_or(false),
        description: string_value(value_from_object(value_object, "description"))
            .or_else(|| string_value(value_from_object(detail_object, "description")))
            .unwrap_or_default(),
        type_id: i64_value(value_from_object(value_object, "type"))
            .or_else(|| i64_value(value_from_object(detail_object, "type")))
            .unwrap_or(0),
        is_new: bool_value(value_from_object(value_object, "new"))
            .or_else(|| bool_value(value_from_object(detail_object, "newVerified")))
            .unwrap_or(false),
        icon_url: string_value(value_from_object(value_object, "iconUrl"))
            .or_else(|| string_value(value_from_object(detail_object, "iconUrl")))
            .unwrap_or_default(),
    }
}

fn banned_status_from_value(value: Option<&Value>) -> KuaishouBannedStatus {
    let object = as_object(value);

    KuaishouBannedStatus {
        banned: bool_value(value_from_object(object, "banned")).unwrap_or(false),
        social_banned: bool_value(value_from_object(object, "socialBanned")).unwrap_or(false),
        isolate: bool_value(value_from_object(object, "isolate")).unwrap_or(false),
        defriend: bool_value(value_from_object(object, "defriend")).unwrap_or(false),
    }
}

fn normalize_live_author(author: Option<&Value>) -> KuaishouUserProfileUserInfo {
    let object = as_object(author);

    KuaishouUserProfileUserInfo {
        id: pick_first_non_empty_string(&[
            string_value(value_from_object(object, "id")),
            string_value(value_from_object(object, "principalId")),
            string_value(value_from_object(object, "kwaiId")),
        ]),
        name: pick_first_non_empty_string(&[
            string_value(value_from_object(object, "name")),
            string_value(value_from_object(object, "user_name")),
        ]),
        description: pick_first_non_empty_string(&[
            string_value(value_from_object(object, "description")),
            string_value(value_from_object(object, "user_text")),
        ]),
        avatar: pick_first_non_empty_string(&[
            string_value(value_from_object(object, "avatar")),
            string_value(value_from_object(object, "headurl")),
        ]),
        sex: pick_first_non_empty_string(&[
            string_value(value_from_object(object, "sex")),
            string_value(value_from_object(object, "user_sex")),
        ]),
        living: bool_value(value_from_object(object, "living"))
            .or_else(|| bool_value(value_from_object(object, "live")))
            .unwrap_or(false),
        follow_status: string_value(value_from_object(object, "followStatus")).unwrap_or_else(
            || {
                if bool_value(value_from_object(object, "following")).unwrap_or(false) {
                    "FOLLOWING".to_owned()
                } else {
                    "UN_FOLLOWED".to_owned()
                }
            },
        ),
        constellation: string_value(value_from_object(object, "constellation")).unwrap_or_default(),
        city_name: string_value(value_from_object(object, "cityName")).unwrap_or_default(),
        origin_user_id: number_value(value_from_object(object, "originUserId"))
            .or_else(|| number_value(value_from_object(object, "user_id")))
            .unwrap_or(0),
        privacy: bool_value(value_from_object(object, "privacy")).unwrap_or(false),
        is_new: bool_value(value_from_object(object, "isNew")).unwrap_or(false),
        timestamp: number_value(value_from_object(object, "timestamp")).unwrap_or(0),
        verified_status: verified_status_from_value(
            value_from_object(object, "verifiedStatus"),
            value_from_object(object, "verifiedDetail"),
        ),
        banned_status: banned_status_from_value(value_from_object(object, "bannedStatus")),
        counts: if is_populated_object(value_from_object(object, "counts")) {
            object_or_empty(value_from_object(object, "counts"))
        } else {
            super::super::support::empty_object()
        },
    }
}

pub(super) fn merge_live_author(
    fallback_author: Option<&Value>,
    user_info: Option<&Value>,
    sensitive_info: Option<&Value>,
) -> KuaishouUserProfileUserInfo {
    let mut merged = normalize_live_author(fallback_author);
    let user_object = as_object(user_info);
    let sensitive_object = as_object(sensitive_info);

    if let Some(object) = user_object {
        merged.id = string_value(value_from_object(Some(object), "id")).unwrap_or(merged.id);
        merged.name = string_value(value_from_object(Some(object), "name")).unwrap_or(merged.name);
        merged.description = string_value(value_from_object(Some(object), "description"))
            .unwrap_or(merged.description);
        merged.avatar =
            string_value(value_from_object(Some(object), "avatar")).unwrap_or(merged.avatar);
        merged.sex = string_value(value_from_object(Some(object), "sex")).unwrap_or(merged.sex);
        merged.living =
            bool_value(value_from_object(Some(object), "living")).unwrap_or(merged.living);
        merged.origin_user_id = number_value(value_from_object(Some(object), "originUserId"))
            .unwrap_or(merged.origin_user_id);
        merged.privacy =
            bool_value(value_from_object(Some(object), "privacy")).unwrap_or(merged.privacy);
        merged.is_new =
            bool_value(value_from_object(Some(object), "isNew")).unwrap_or(merged.is_new);
        merged.timestamp =
            number_value(value_from_object(Some(object), "timestamp")).unwrap_or(merged.timestamp);
    }

    merged.follow_status = pick_first_non_empty_string(&[
        string_value(value_from_object(user_object, "followStatus")),
        string_value(value_from_object(sensitive_object, "followStatus")),
        Some(merged.follow_status.clone()),
    ]);
    merged.constellation = pick_first_non_empty_string(&[
        string_value(value_from_object(user_object, "constellation")),
        string_value(value_from_object(sensitive_object, "constellation")),
        Some(merged.constellation.clone()),
    ]);
    merged.city_name = pick_first_non_empty_string(&[
        string_value(value_from_object(user_object, "cityName")),
        string_value(value_from_object(sensitive_object, "cityName")),
        Some(merged.city_name.clone()),
    ]);
    merged.verified_status = if value_from_object(user_object, "verifiedStatus").is_some() {
        verified_status_from_value(value_from_object(user_object, "verifiedStatus"), None)
    } else if value_from_object(sensitive_object, "verifiedStatus").is_some() {
        verified_status_from_value(value_from_object(sensitive_object, "verifiedStatus"), None)
    } else {
        merged.verified_status
    };
    merged.banned_status = if value_from_object(user_object, "bannedStatus").is_some() {
        banned_status_from_value(value_from_object(user_object, "bannedStatus"))
    } else if value_from_object(sensitive_object, "bannedStatus").is_some() {
        banned_status_from_value(value_from_object(sensitive_object, "bannedStatus"))
    } else {
        merged.banned_status
    };
    merged.counts = if is_populated_object(value_from_object(user_object, "counts")) {
        object_or_empty(value_from_object(user_object, "counts"))
    } else if is_populated_object(value_from_object(sensitive_object, "counts")) {
        object_or_empty(value_from_object(sensitive_object, "counts"))
    } else {
        merged.counts
    };

    merged
}

pub(super) fn create_follow_state(
    user_info: Option<&Value>,
    sensitive_info: Option<&Value>,
) -> Option<KuaishouFollowState> {
    let user_object = as_object(user_info);
    let sensitive_object = as_object(sensitive_info);
    let follow_status = pick_first_non_empty_string(&[
        string_value(value_from_object(user_object, "followStatus")),
        string_value(value_from_object(sensitive_object, "followStatus")),
    ]);

    if follow_status.is_empty() {
        return None;
    }

    Some(KuaishouFollowState {
        current_follow_status: follow_status,
        need_to_follow: false,
        author_id: string_value(value_from_object(user_object, "id")).unwrap_or_default(),
        data: 0,
    })
}

pub(super) fn create_follow_button_state(
    user_info: Option<&Value>,
    sensitive_info: Option<&Value>,
) -> Option<KuaishouFollowButtonState> {
    let user_object = as_object(user_info);
    let sensitive_object = as_object(sensitive_info);
    let follow_status = pick_first_non_empty_string(&[
        string_value(value_from_object(user_object, "followStatus")),
        string_value(value_from_object(sensitive_object, "followStatus")),
    ]);

    (!follow_status.is_empty()).then_some(KuaishouFollowButtonState { follow_status })
}
