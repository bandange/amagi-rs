use serde::{Deserialize, Serialize};

/// Query parameters for Bilibili comment requests.
#[derive(Debug, Deserialize)]
pub struct BilibiliCommentQuery {
    /// Required Bilibili comment type.
    #[serde(rename = "type")]
    pub comment_type: u32,
    /// Optional page size.
    pub number: Option<u32>,
    /// Optional comment sorting mode.
    pub mode: Option<u32>,
}

/// Query parameters for Bilibili comment-reply requests.
#[derive(Debug, Deserialize)]
pub struct BilibiliCommentRepliesQuery {
    /// Required Bilibili comment type.
    #[serde(rename = "type")]
    pub comment_type: u32,
    /// Optional page size.
    pub number: Option<u32>,
}

/// Query parameters for Bilibili playurl requests.
#[derive(Debug, Deserialize)]
pub struct BilibiliCidQuery {
    /// Required content id for the target playurl request.
    pub cid: u64,
}

/// Query parameters for Bilibili danmaku requests.
#[derive(Debug, Default, Deserialize)]
pub struct BilibiliDanmakuQuery {
    /// Optional danmaku segment index. Defaults to the first segment.
    pub segment_index: Option<u32>,
}

/// Query parameters for Bilibili article-card requests.
#[derive(Debug, Deserialize)]
pub struct BilibiliArticleCardsQuery {
    /// Comma-separated ids accepted by the Bilibili article-cards API.
    pub ids: String,
}

/// Query parameters for Bilibili QR-code status requests.
#[derive(Debug, Deserialize)]
pub struct BilibiliQrcodeStatusQuery {
    /// QR-code key returned by the generate endpoint.
    pub qrcode_key: String,
}

/// Request body for Bilibili captcha register requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct BilibiliCaptchaFromVoucherRequest {
    /// Voucher returned by the upstream risk-control flow.
    pub v_voucher: String,
    /// Optional csrf token.
    pub csrf: Option<String>,
}

/// Request body for Bilibili captcha validation requests.
#[derive(Debug, Deserialize, Serialize)]
pub struct BilibiliValidateCaptchaRequest {
    /// Challenge returned by the captcha provider.
    pub challenge: String,
    /// Token returned by the captcha provider.
    pub token: String,
    /// Validate token returned by the captcha provider.
    pub validate: String,
    /// Seccode returned by the captcha provider.
    pub seccode: String,
    /// Optional csrf token.
    pub csrf: Option<String>,
}
