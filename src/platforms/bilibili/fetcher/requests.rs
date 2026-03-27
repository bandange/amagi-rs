use reqwest::Url;
use serde_json::json;

use crate::error::AppError;

const USER_DYNAMIC_FEATURES: &str = "itemOpusStyle,listOnlyfans,opusBigCover,onlyfansVote,forwardListHidden,decorationCard,commentsNewVersion,onlyfansAssetsV2,ugcDelete,onlyfansQaCard,avatarAutoTheme,sunflowerStyle,eva3CardOpus,eva3CardVideo,eva3CardComment";
const DYNAMIC_DETAIL_FEATURES: &str = "itemOpusStyle,opusBigCover,onlyfansVote,endFooterHidden,decorationCard,onlyfansAssetsV2,ugcDelete,onlyfansQaCard,editable,opusPrivateVisible,avatarAutoTheme";

fn normalize_base_url(base_url: &str) -> &str {
    base_url.trim_end_matches('/')
}

fn with_query(base_url: &str, path: &str, pairs: &[(&str, String)]) -> Result<String, AppError> {
    let mut url =
        Url::parse(&format!("{}{}", normalize_base_url(base_url), path)).map_err(|error| {
            AppError::InvalidRequestConfig(format!("invalid bilibili url: {error}"))
        })?;
    {
        let mut query = url.query_pairs_mut();
        for (key, value) in pairs {
            query.append_pair(key, value);
        }
    }
    Ok(url.to_string())
}

fn normalize_prefixed_id(value: &str, prefix: &str) -> String {
    value.trim_start_matches(prefix).to_owned()
}

fn url_without_query(base_url: &str, path: &str) -> Result<String, AppError> {
    Url::parse(&format!("{}{}", normalize_base_url(base_url), path))
        .map(|url| url.to_string())
        .map_err(|error| AppError::InvalidRequestConfig(format!("invalid bilibili url: {error}")))
}

pub(crate) fn video_info(base_url: &str, bvid: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/web-interface/view",
        &[("bvid", bvid.to_owned())],
    )
}

pub(crate) fn video_stream(base_url: &str, aid: u64, cid: u64) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/player/playurl",
        &[("avid", aid.to_string()), ("cid", cid.to_string())],
    )
}

pub(crate) fn video_danmaku(
    base_url: &str,
    cid: u64,
    segment_index: Option<u32>,
) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/v2/dm/web/seg.so",
        &[
            ("type", "1".to_owned()),
            ("oid", cid.to_string()),
            ("segment_index", segment_index.unwrap_or(1).to_string()),
        ],
    )
}

pub(crate) fn emoji_list(base_url: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/emote/user/panel/web",
        &[
            ("business", "reply".to_owned()),
            ("web_location", "0.0".to_owned()),
        ],
    )
}

pub(crate) fn article_content(base_url: &str, article_id: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/article/view",
        &[("id", normalize_prefixed_id(article_id, "cv"))],
    )
}

pub(crate) fn article_cards(base_url: &str, ids: &str) -> Result<String, AppError> {
    with_query(base_url, "/x/article/cards", &[("ids", ids.to_owned())])
}

pub(crate) fn article_info(base_url: &str, article_id: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/article/viewinfo",
        &[("id", normalize_prefixed_id(article_id, "cv"))],
    )
}

pub(crate) fn article_list_info(base_url: &str, list_id: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/article/list/web/articles",
        &[("id", list_id.to_owned())],
    )
}

pub(crate) fn captcha_from_voucher(base_url: &str) -> Result<String, AppError> {
    url_without_query(base_url, "/x/gaia-vgate/v1/register")
}

pub(crate) fn validate_captcha(base_url: &str) -> Result<String, AppError> {
    url_without_query(base_url, "/x/gaia-vgate/v1/validate")
}

pub(crate) fn bangumi_info(base_url: &str, bangumi_id: &str) -> Result<String, AppError> {
    let (key, value) = if bangumi_id.starts_with("ep") {
        ("ep_id", normalize_prefixed_id(bangumi_id, "ep"))
    } else {
        ("season_id", normalize_prefixed_id(bangumi_id, "ss"))
    };

    with_query(base_url, "/pgc/view/web/season", &[(key, value)])
}

pub(crate) fn bangumi_stream(base_url: &str, cid: u64, ep_id: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/pgc/player/web/playurl",
        &[
            ("cid", cid.to_string()),
            ("ep_id", normalize_prefixed_id(ep_id, "ep")),
        ],
    )
}

pub(crate) fn user_card(base_url: &str, host_mid: u64) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/web-interface/card",
        &[("mid", host_mid.to_string()), ("photo", "true".to_owned())],
    )
}

pub(crate) fn user_dynamic_list(base_url: &str, host_mid: u64) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/polymer/web-dynamic/v1/feed/space",
        &[
            ("host_mid", host_mid.to_string()),
            ("offset", String::new()),
            ("platform", "web".to_owned()),
            ("features", USER_DYNAMIC_FEATURES.to_owned()),
        ],
    )
}

pub(crate) fn user_space_info(base_url: &str, host_mid: u64) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/space/wbi/acc/info",
        &[("mid", host_mid.to_string())],
    )
}

pub(crate) fn uploader_total_views(base_url: &str, host_mid: u64) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/space/upstat",
        &[("mid", host_mid.to_string())],
    )
}

pub(crate) fn login_status(base_url: &str) -> Result<String, AppError> {
    url_without_query(base_url, "/x/web-interface/nav")
}

pub(crate) fn login_qrcode(passport_base_url: &str) -> Result<String, AppError> {
    url_without_query(passport_base_url, "/x/passport-login/web/qrcode/generate")
}

pub(crate) fn qrcode_status(passport_base_url: &str, qrcode_key: &str) -> Result<String, AppError> {
    with_query(
        passport_base_url,
        "/x/passport-login/web/qrcode/poll",
        &[("qrcode_key", qrcode_key.to_owned())],
    )
}

pub(crate) fn comments(
    base_url: &str,
    oid: u64,
    comment_type: u32,
    mode: Option<u32>,
    pagination_offset: Option<&str>,
) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/v2/reply/wbi/main",
        &[
            ("oid", oid.to_string()),
            ("type", comment_type.to_string()),
            ("mode", mode.unwrap_or(3).to_string()),
            (
                "pagination_str",
                json!({ "offset": pagination_offset.unwrap_or_default() }).to_string(),
            ),
            ("plat", "1".to_owned()),
            ("seek_rpid", String::new()),
            ("web_location", "1315875".to_owned()),
        ],
    )
}

pub(crate) fn comment_status(
    base_url: &str,
    oid: u64,
    comment_type: u32,
) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/v2/reply/subject/description",
        &[("type", comment_type.to_string()), ("oid", oid.to_string())],
    )
}

pub(crate) fn comment_replies(
    base_url: &str,
    oid: u64,
    comment_type: u32,
    root: u64,
    number: Option<u32>,
) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/v2/reply/reply",
        &[
            ("type", comment_type.to_string()),
            ("oid", oid.to_string()),
            ("root", root.to_string()),
            ("ps", number.unwrap_or(20).to_string()),
        ],
    )
}

pub(crate) fn dynamic_detail(base_url: &str, dynamic_id: &str) -> Result<String, AppError> {
    with_query(
        base_url,
        "/x/polymer/web-dynamic/v1/detail",
        &[
            ("id", dynamic_id.to_owned()),
            ("features", DYNAMIC_DETAIL_FEATURES.to_owned()),
        ],
    )
}

pub(crate) fn dynamic_card(vc_base_url: &str, dynamic_id: &str) -> Result<String, AppError> {
    with_query(
        vc_base_url,
        "/dynamic_svr/v1/dynamic_svr/get_dynamic_detail",
        &[("dynamic_id", dynamic_id.to_owned())],
    )
}

pub(crate) fn live_room_info(live_base_url: &str, room_id: u64) -> Result<String, AppError> {
    with_query(
        live_base_url,
        "/room/v1/Room/get_info",
        &[("room_id", room_id.to_string())],
    )
}

pub(crate) fn live_room_init(live_base_url: &str, room_id: u64) -> Result<String, AppError> {
    with_query(
        live_base_url,
        "/room/v1/Room/room_init",
        &[("id", room_id.to_string())],
    )
}
