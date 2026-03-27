use serde_json::{Map, Value};

use crate::error::AppError;

use super::super::{
    danmaku::parse_dm_seg_mobile_reply,
    sign::{av_to_bv, bv_to_av},
    types::{
        BilibiliArticleCards, BilibiliArticleContent, BilibiliArticleInfo, BilibiliArticleListInfo,
        BilibiliAvToBv, BilibiliAvToBvData, BilibiliBangumiInfo, BilibiliBangumiStream,
        BilibiliBvToAv, BilibiliBvToAvData, BilibiliCaptchaFromVoucher, BilibiliDanmakuData,
        BilibiliDanmakuList, BilibiliEmojiList, BilibiliValidateCaptcha, BilibiliVideoInfo,
        BilibiliVideoStream,
    },
};
use super::{BilibiliFetcher, requests};

impl BilibiliFetcher {
    /// Fetch one Bilibili video payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchVideoInfo")]
    pub async fn fetch_video_info(&self, bvid: &str) -> Result<BilibiliVideoInfo, AppError> {
        self.fetch_json(&requests::video_info(self.api_base_url.as_ref(), bvid)?)
            .await
    }

    /// Fetch stream URLs for one Bilibili video.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails, the WBI signature
    /// cannot be derived for authenticated requests, or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchVideoStream")]
    #[doc(alias = "fetchVideoStreamUrl")]
    pub async fn fetch_video_stream(
        &self,
        aid: u64,
        cid: u64,
    ) -> Result<BilibiliVideoStream, AppError> {
        let url = self
            .build_playurl_request(&requests::video_stream(
                self.api_base_url.as_ref(),
                aid,
                cid,
            )?)
            .await?;
        self.fetch_json(&url).await
    }

    /// Fetch one Bilibili danmaku segment and decode its protobuf payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the protobuf body
    /// cannot be decoded.
    #[doc(alias = "fetchVideoDanmaku")]
    pub async fn fetch_video_danmaku(
        &self,
        cid: u64,
        segment_index: Option<u32>,
    ) -> Result<BilibiliDanmakuList, AppError> {
        let url = requests::video_danmaku(self.api_base_url.as_ref(), cid, segment_index)?;
        let bytes = self.send_bytes_request(&url, None).await?;
        let elems = parse_dm_seg_mobile_reply(&bytes)?;

        Ok(BilibiliDanmakuList {
            code: 0,
            message: "success".to_owned(),
            ttl: None,
            data: BilibiliDanmakuData {
                elems: elems.clone(),
            },
            upstream_payload: serde_json::to_value(BilibiliDanmakuData { elems })
                .unwrap_or(Value::Null),
        })
    }

    /// Fetch the Bilibili emoji catalog.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchEmojiList")]
    pub async fn fetch_emoji_list(&self) -> Result<BilibiliEmojiList, AppError> {
        self.fetch_json(&requests::emoji_list(self.api_base_url.as_ref())?)
            .await
    }

    /// Fetch the content payload for one Bilibili article.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchArticleContent")]
    pub async fn fetch_article_content(
        &self,
        article_id: &str,
    ) -> Result<BilibiliArticleContent, AppError> {
        self.fetch_json(&requests::article_content(
            self.api_base_url.as_ref(),
            article_id,
        )?)
        .await
    }

    /// Fetch display-card payloads for a list of Bilibili article-related ids.
    ///
    /// # Errors
    ///
    /// Returns an error when no ids are provided, the upstream request fails,
    /// or the response body contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchArticleCards")]
    pub async fn fetch_article_cards<I, S>(&self, ids: I) -> Result<BilibiliArticleCards, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined_ids = ids
            .into_iter()
            .map(|value| value.as_ref().to_owned())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>()
            .join(",");

        if joined_ids.is_empty() {
            return Err(AppError::InvalidRequestConfig(
                "bilibili article cards requires at least one id".to_owned(),
            ));
        }

        self.fetch_json(&requests::article_cards(
            self.api_base_url.as_ref(),
            &joined_ids,
        )?)
        .await
    }

    /// Fetch metadata for one Bilibili article.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchArticleInfo")]
    pub async fn fetch_article_info(
        &self,
        article_id: &str,
    ) -> Result<BilibiliArticleInfo, AppError> {
        self.fetch_json(&requests::article_info(
            self.api_base_url.as_ref(),
            article_id,
        )?)
        .await
    }

    /// Fetch metadata for one Bilibili article collection.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchArticleListInfo")]
    pub async fn fetch_article_list_info(
        &self,
        list_id: &str,
    ) -> Result<BilibiliArticleListInfo, AppError> {
        self.fetch_json(&requests::article_list_info(
            self.api_base_url.as_ref(),
            list_id,
        )?)
        .await
    }

    /// Fetch metadata for one Bilibili bangumi season or episode.
    ///
    /// The input follows the TypeScript implementation and accepts either an
    /// `ep...` identifier or an `ss...` identifier. Bare numeric ids are
    /// treated as `season_id`.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchBangumiInfo")]
    pub async fn fetch_bangumi_info(
        &self,
        bangumi_id: &str,
    ) -> Result<BilibiliBangumiInfo, AppError> {
        self.fetch_json(&requests::bangumi_info(
            self.api_base_url.as_ref(),
            bangumi_id,
        )?)
        .await
    }

    /// Fetch stream URLs for one Bilibili bangumi episode.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails, the WBI signature
    /// cannot be derived for authenticated requests, or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchBangumiStream")]
    #[doc(alias = "fetchBangumiStreamUrl")]
    pub async fn fetch_bangumi_stream(
        &self,
        ep_id: &str,
        cid: u64,
    ) -> Result<BilibiliBangumiStream, AppError> {
        let url = self
            .build_playurl_request(&requests::bangumi_stream(
                self.api_base_url.as_ref(),
                cid,
                ep_id,
            )?)
            .await?;
        self.fetch_json(&url).await
    }

    /// Request a captcha challenge from one Bilibili `v_voucher`.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "requestCaptchaFromVoucher")]
    pub async fn request_captcha_from_voucher(
        &self,
        v_voucher: &str,
        csrf: Option<&str>,
    ) -> Result<BilibiliCaptchaFromVoucher, AppError> {
        let url = requests::captcha_from_voucher(self.api_base_url.as_ref())?;
        let mut body =
            Map::from_iter([("v_voucher".to_owned(), Value::String(v_voucher.to_owned()))]);

        if let Some(csrf) = csrf.filter(|value| !value.is_empty()) {
            body.insert("csrf".to_owned(), Value::String(csrf.to_owned()));
        }

        self.post_json(&url, &Value::Object(body)).await
    }

    /// Validate a Bilibili captcha challenge result.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "validateCaptchaResult")]
    pub async fn validate_captcha_result(
        &self,
        challenge: &str,
        token: &str,
        validate: &str,
        seccode: &str,
        csrf: Option<&str>,
    ) -> Result<BilibiliValidateCaptcha, AppError> {
        let url = requests::validate_captcha(self.api_base_url.as_ref())?;
        let mut body = Map::from_iter([
            ("challenge".to_owned(), Value::String(challenge.to_owned())),
            ("token".to_owned(), Value::String(token.to_owned())),
            ("validate".to_owned(), Value::String(validate.to_owned())),
            ("seccode".to_owned(), Value::String(seccode.to_owned())),
        ]);

        if let Some(csrf) = csrf.filter(|value| !value.is_empty()) {
            body.insert("csrf".to_owned(), Value::String(csrf.to_owned()));
        }

        self.post_json(&url, &Value::Object(body)).await
    }

    /// Convert a numeric AV identifier into its BV representation.
    #[doc(alias = "convertAvToBv")]
    pub fn convert_av_to_bv(&self, aid: u64) -> BilibiliAvToBv {
        let bvid = av_to_bv(aid);
        BilibiliAvToBv {
            code: 0,
            message: "success".to_owned(),
            data: BilibiliAvToBvData { bvid: bvid.clone() },
            upstream_payload: Value::Object(Map::from_iter([(
                "bvid".to_owned(),
                Value::String(bvid),
            )])),
        }
    }

    /// Convert a BV identifier into its AV representation.
    ///
    /// # Errors
    ///
    /// Returns an error when the provided BV identifier is invalid.
    #[doc(alias = "convertBvToAv")]
    pub fn convert_bv_to_av(&self, bvid: &str) -> Result<BilibiliBvToAv, AppError> {
        let aid = format!("av{}", bv_to_av(bvid)?);
        Ok(BilibiliBvToAv {
            code: 0,
            message: "success".to_owned(),
            data: BilibiliBvToAvData { aid: aid.clone() },
            upstream_payload: Value::Object(Map::from_iter([(
                "aid".to_owned(),
                Value::String(aid),
            )])),
        })
    }
}
