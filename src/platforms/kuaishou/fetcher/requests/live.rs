use crate::error::AppError;

use super::super::super::sign::{
    KuaishouJsonObject, KuaishouJsonValue, KuaishouLiveApiMethod, KuaishouLiveApiRequest,
};
use super::shared::create_live_api_request;

const DEFAULT_PAGE_SIZE: u32 = 12;

pub(crate) fn playback_list(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "playbackList",
        "/live_api/playback/list",
        &[
            ("principalId", principal_id.to_owned()),
            ("count", DEFAULT_PAGE_SIZE.to_string()),
            ("cursor", String::new()),
            ("hasMore", "true".to_owned()),
        ],
    )?
    .with_requires_sign(false))
}

pub(crate) fn interest_mask_list(base_url: &str) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "interestMaskList",
        "/live_api/interestMask/list",
        &[],
    )?
    .with_requires_sign(false))
}

pub(crate) fn category_config(base_url: &str) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(
        create_live_api_request(base_url, "categoryConfig", "/live_api/category/config", &[])?
            .with_requires_sign(false),
    )
}

pub(crate) fn category_data(base_url: &str) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(
        create_live_api_request(base_url, "categoryData", "/live_api/category/data", &[])?
            .with_requires_sign(false),
    )
}

pub(crate) fn category_classify(base_url: &str) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "categoryClassify",
        "/live_api/category/classify",
        &[
            ("type", "4".to_owned()),
            ("source", "2".to_owned()),
            ("page", "1".to_owned()),
            ("pageSize", "20".to_owned()),
        ],
    )?
    .with_requires_sign(false))
}

pub(crate) fn live_detail(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    live_detail_with_auth_token(base_url, principal_id, None)
}

pub(crate) fn live_detail_with_auth_token(
    base_url: &str,
    principal_id: &str,
    auth_token: Option<&str>,
) -> Result<KuaishouLiveApiRequest, AppError> {
    let mut query = vec![("principalId", principal_id.to_owned())];

    if let Some(auth_token) = auth_token.filter(|value| !value.trim().is_empty()) {
        query.push(("authToken", auth_token.trim().to_owned()));
    }

    Ok(create_live_api_request(
        base_url,
        "liveDetail",
        "/live_api/liveroom/livedetail",
        &query,
    )?
    .with_requires_sign(false))
}

pub(crate) fn live_gift_list(
    base_url: &str,
    live_stream_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "liveGiftList",
        "/live_api/emoji/gift-list",
        &[("liveStreamId", live_stream_id.to_owned())],
    )?
    .with_requires_sign(false))
}

pub(crate) fn live_websocket_info(
    base_url: &str,
    live_stream_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "liveWebsocketInfo",
        "/live_api/liveroom/websocketinfo",
        &[
            ("caver", "2".to_owned()),
            ("liveStreamId", live_stream_id.to_owned()),
        ],
    )?
    .with_sign_path("/rest/k/live/websocket/info"))
}

pub(crate) fn live_reco(
    base_url: &str,
    game_id: Option<&str>,
) -> Result<KuaishouLiveApiRequest, AppError> {
    let normalized_game_id = game_id
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(1001);

    let mut following_param = KuaishouJsonObject::new();
    following_param.insert("queryFollowing", true);
    following_param.insert("followingWeight", 50);

    let mut game_favour_entry = KuaishouJsonObject::new();
    game_favour_entry.insert("gameId", normalized_game_id);
    game_favour_entry.insert("totalStayLength", 100);

    let mut body = KuaishouJsonObject::new();
    body.insert("followingParam", following_param);
    body.insert(
        "gameFavour",
        KuaishouJsonValue::Array(vec![KuaishouJsonValue::Object(game_favour_entry)]),
    );

    Ok(
        create_live_api_request(base_url, "liveReco", "/live_api/liveroom/reco", &[])?
            .with_method(KuaishouLiveApiMethod::Post)
            .with_requires_sign(false)
            .with_body(body),
    )
}
