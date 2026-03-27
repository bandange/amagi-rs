use serde_json::{Map, Value};

use crate::{error::AppError, platforms::bilibili::fetcher::requests};

use super::types::BilibiliJsonPostRequest;

const BILIBILI_API_BASE_URL: &str = "https://api.bilibili.com";
const BILIBILI_VC_BASE_URL: &str = "https://api.vc.bilibili.com";
const BILIBILI_LIVE_BASE_URL: &str = "https://api.live.bilibili.com";
const BILIBILI_PASSPORT_BASE_URL: &str = "https://passport.bilibili.com";

/// Public Bilibili API URL builder.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias = "bilibiliApiUrls")]
pub struct BilibiliApiUrls {
    api_base_url: String,
    vc_base_url: String,
    live_base_url: String,
    passport_base_url: String,
}

impl Default for BilibiliApiUrls {
    fn default() -> Self {
        Self::new()
    }
}

impl BilibiliApiUrls {
    /// Create a Bilibili API builder with the default platform endpoints.
    pub fn new() -> Self {
        Self::with_base_urls(
            BILIBILI_API_BASE_URL,
            BILIBILI_VC_BASE_URL,
            BILIBILI_LIVE_BASE_URL,
            BILIBILI_PASSPORT_BASE_URL,
        )
    }

    /// Create a Bilibili API builder with explicit platform endpoints.
    pub fn with_base_urls(
        api_base_url: impl Into<String>,
        vc_base_url: impl Into<String>,
        live_base_url: impl Into<String>,
        passport_base_url: impl Into<String>,
    ) -> Self {
        Self {
            api_base_url: api_base_url.into(),
            vc_base_url: vc_base_url.into(),
            live_base_url: live_base_url.into(),
            passport_base_url: passport_base_url.into(),
        }
    }

    /// Build the login-status URL.
    #[doc(alias = "getLoginStatus")]
    pub fn login_status(&self) -> Result<String, AppError> {
        requests::login_status(&self.api_base_url)
    }

    /// Build the video-info URL.
    #[doc(alias = "getVideoInfo")]
    pub fn video_info(&self, bvid: &str) -> Result<String, AppError> {
        requests::video_info(&self.api_base_url, bvid)
    }

    /// Build the video-stream URL.
    #[doc(alias = "getVideoStream")]
    pub fn video_stream(&self, aid: u64, cid: u64) -> Result<String, AppError> {
        requests::video_stream(&self.api_base_url, aid, cid)
    }

    /// Build the comments URL.
    #[doc(alias = "getComments")]
    pub fn comments(
        &self,
        oid: u64,
        comment_type: u32,
        mode: Option<u32>,
        pagination_offset: Option<&str>,
    ) -> Result<String, AppError> {
        requests::comments(
            &self.api_base_url,
            oid,
            comment_type,
            mode,
            pagination_offset,
        )
    }

    /// Build the comment-status URL.
    #[doc(alias = "getCommentStatus")]
    pub fn comment_status(&self, oid: u64, comment_type: u32) -> Result<String, AppError> {
        requests::comment_status(&self.api_base_url, oid, comment_type)
    }

    /// Build the comment-replies URL.
    #[doc(alias = "getCommentReplies")]
    pub fn comment_replies(
        &self,
        oid: u64,
        comment_type: u32,
        root: u64,
        number: Option<u32>,
    ) -> Result<String, AppError> {
        requests::comment_replies(&self.api_base_url, oid, comment_type, root, number)
    }

    /// Build the emoji-list URL.
    #[doc(alias = "getEmojiList")]
    pub fn emoji_list(&self) -> Result<String, AppError> {
        requests::emoji_list(&self.api_base_url)
    }

    /// Build the bangumi-info URL.
    #[doc(alias = "getBangumiInfo")]
    pub fn bangumi_info(&self, bangumi_id: &str) -> Result<String, AppError> {
        requests::bangumi_info(&self.api_base_url, bangumi_id)
    }

    /// Build the bangumi-stream URL.
    #[doc(alias = "getBangumiStream")]
    pub fn bangumi_stream(&self, ep_id: &str, cid: u64) -> Result<String, AppError> {
        requests::bangumi_stream(&self.api_base_url, cid, ep_id)
    }

    /// Build the user-dynamic-list URL.
    #[doc(alias = "getUserDynamicList")]
    pub fn user_dynamic_list(&self, host_mid: u64) -> Result<String, AppError> {
        requests::user_dynamic_list(&self.api_base_url, host_mid)
    }

    /// Build the dynamic-detail URL.
    #[doc(alias = "getDynamicDetail")]
    pub fn dynamic_detail(&self, dynamic_id: &str) -> Result<String, AppError> {
        requests::dynamic_detail(&self.api_base_url, dynamic_id)
    }

    /// Build the dynamic-card URL.
    #[doc(alias = "getDynamicCard")]
    pub fn dynamic_card(&self, dynamic_id: &str) -> Result<String, AppError> {
        requests::dynamic_card(&self.vc_base_url, dynamic_id)
    }

    /// Build the user-card URL.
    #[doc(alias = "getUserCard")]
    pub fn user_card(&self, host_mid: u64) -> Result<String, AppError> {
        requests::user_card(&self.api_base_url, host_mid)
    }

    /// Build the live-room-info URL.
    #[doc(alias = "getLiveRoomInfo")]
    pub fn live_room_info(&self, room_id: u64) -> Result<String, AppError> {
        requests::live_room_info(&self.live_base_url, room_id)
    }

    /// Build the live-room-init URL.
    #[doc(alias = "getLiveRoomInit")]
    pub fn live_room_init(&self, room_id: u64) -> Result<String, AppError> {
        requests::live_room_init(&self.live_base_url, room_id)
    }

    /// Build the login-qrcode URL.
    #[doc(alias = "getLoginQrcode")]
    pub fn login_qrcode(&self) -> Result<String, AppError> {
        requests::login_qrcode(&self.passport_base_url)
    }

    /// Build the qrcode-status URL.
    #[doc(alias = "getQrcodeStatus")]
    pub fn qrcode_status(&self, qrcode_key: &str) -> Result<String, AppError> {
        requests::qrcode_status(&self.passport_base_url, qrcode_key)
    }

    /// Build the uploader-total-views URL.
    #[doc(alias = "getUploaderTotalViews")]
    pub fn uploader_total_views(&self, host_mid: u64) -> Result<String, AppError> {
        requests::uploader_total_views(&self.api_base_url, host_mid)
    }

    /// Build the article-content URL.
    #[doc(alias = "getArticleContent")]
    pub fn article_content(&self, article_id: &str) -> Result<String, AppError> {
        requests::article_content(&self.api_base_url, article_id)
    }

    /// Build the article-cards URL.
    #[doc(alias = "getArticleCards")]
    pub fn article_cards<I, S>(&self, ids: I) -> Result<String, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = ids
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>()
            .join(",");

        requests::article_cards(&self.api_base_url, &joined)
    }

    /// Build the article-info URL.
    #[doc(alias = "getArticleInfo")]
    pub fn article_info(&self, article_id: &str) -> Result<String, AppError> {
        requests::article_info(&self.api_base_url, article_id)
    }

    /// Build the article-list-info URL.
    #[doc(alias = "getArticleListInfo")]
    pub fn article_list_info(&self, list_id: &str) -> Result<String, AppError> {
        requests::article_list_info(&self.api_base_url, list_id)
    }

    /// Build the user-space-info URL.
    #[doc(alias = "getUserSpaceInfo")]
    pub fn user_space_info(&self, host_mid: u64) -> Result<String, AppError> {
        requests::user_space_info(&self.api_base_url, host_mid)
    }

    /// Build the captcha-from-voucher POST request.
    #[doc(alias = "getCaptchaFromVoucher")]
    pub fn captcha_from_voucher(
        &self,
        v_voucher: &str,
        csrf: Option<&str>,
    ) -> Result<BilibiliJsonPostRequest, AppError> {
        let url = requests::captcha_from_voucher(&self.api_base_url)?;
        let mut body =
            Map::from_iter([("v_voucher".to_owned(), Value::String(v_voucher.to_owned()))]);

        if let Some(csrf) = csrf.filter(|value| !value.is_empty()) {
            body.insert("csrf".to_owned(), Value::String(csrf.to_owned()));
        }

        Ok(BilibiliJsonPostRequest {
            url,
            body: Value::Object(body),
        })
    }

    /// Build the validate-captcha POST request.
    #[doc(alias = "validateCaptcha")]
    pub fn validate_captcha(
        &self,
        challenge: &str,
        token: &str,
        validate: &str,
        seccode: &str,
        csrf: Option<&str>,
    ) -> Result<BilibiliJsonPostRequest, AppError> {
        let url = requests::validate_captcha(&self.api_base_url)?;
        let mut body = Map::from_iter([
            ("challenge".to_owned(), Value::String(challenge.to_owned())),
            ("token".to_owned(), Value::String(token.to_owned())),
            ("validate".to_owned(), Value::String(validate.to_owned())),
            ("seccode".to_owned(), Value::String(seccode.to_owned())),
        ]);

        if let Some(csrf) = csrf.filter(|value| !value.is_empty()) {
            body.insert("csrf".to_owned(), Value::String(csrf.to_owned()));
        }

        Ok(BilibiliJsonPostRequest {
            url,
            body: Value::Object(body),
        })
    }

    /// Build the video-danmaku URL.
    #[doc(alias = "getVideoDanmaku")]
    pub fn video_danmaku(&self, cid: u64, segment_index: Option<u32>) -> Result<String, AppError> {
        requests::video_danmaku(&self.api_base_url, cid, segment_index)
    }
}

/// Create a public Bilibili API builder.
#[doc(alias = "createBilibiliApiUrls")]
pub fn create_bilibili_api_urls() -> BilibiliApiUrls {
    BilibiliApiUrls::new()
}
