//! Public Douyin request URL builders migrated from the TypeScript platform layer.

use crate::error::AppError;

use super::super::{
    fetcher::requests::{DouyinApiEndpoints, DouyinRequestBuilder},
    generate_verify_fp,
    types::DouyinSearchType,
};

/// Public Douyin API URL builder.
#[derive(Debug, Clone)]
#[doc(alias = "douyinApiUrls")]
pub struct DouyinApiUrls {
    user_agent: Option<String>,
    verify_fp: String,
}

impl Default for DouyinApiUrls {
    fn default() -> Self {
        Self::new(None)
    }
}

impl DouyinApiUrls {
    /// Create a Douyin URL builder using an optional browser user-agent.
    pub fn new(user_agent: Option<&str>) -> Self {
        Self::with_verify_fp(user_agent, generate_verify_fp())
    }

    /// Create a Douyin URL builder with an explicit `verify_fp`.
    pub fn with_verify_fp(user_agent: Option<&str>, verify_fp: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.map(str::to_owned),
            verify_fp: verify_fp.into(),
        }
    }

    /// Return the `verify_fp` currently bound to this builder.
    pub fn verify_fp(&self) -> &str {
        &self.verify_fp
    }

    /// Build the work-detail URL for a Douyin aweme.
    #[doc(alias = "getWorkDetail")]
    pub fn work_detail(&self, aweme_id: &str) -> Result<String, AppError> {
        self.request_builder().work_detail(aweme_id)
    }

    /// Build the comments URL for a Douyin aweme.
    #[doc(alias = "getComments")]
    pub fn comments(
        &self,
        aweme_id: &str,
        cursor: Option<u64>,
        count: Option<u32>,
    ) -> Result<String, AppError> {
        self.request_builder()
            .comments(aweme_id, cursor.unwrap_or(0), count.unwrap_or(50))
    }

    /// Build the comment-replies URL for a Douyin aweme comment.
    #[doc(alias = "getCommentReplies")]
    pub fn comment_replies(
        &self,
        aweme_id: &str,
        comment_id: &str,
        cursor: Option<u64>,
        count: Option<u32>,
    ) -> Result<String, AppError> {
        self.request_builder().comment_replies(
            aweme_id,
            comment_id,
            cursor.unwrap_or(0),
            count.unwrap_or(3),
        )
    }

    /// Build the slides-info URL for a Douyin aweme.
    #[doc(alias = "getSlidesInfo")]
    pub fn slides_info(&self, aweme_id: &str) -> Result<String, AppError> {
        self.request_builder().slides_info(aweme_id)
    }

    /// Return the emoji-list URL.
    #[doc(alias = "getEmojiList")]
    pub fn emoji_list(&self) -> String {
        self.request_builder().emoji_list()
    }

    /// Build the user-video-list URL for a Douyin account.
    #[doc(alias = "getUserVideoList")]
    pub fn user_video_list(
        &self,
        sec_uid: &str,
        max_cursor: Option<&str>,
        count: Option<u32>,
    ) -> Result<String, AppError> {
        self.request_builder().user_video_list(
            sec_uid,
            max_cursor.unwrap_or("0"),
            count.unwrap_or(18),
        )
    }

    /// Build the user-favorite-list URL for a Douyin account.
    #[doc(alias = "getUserFavoriteList")]
    pub fn user_favorite_list(
        &self,
        sec_uid: &str,
        max_cursor: Option<&str>,
        count: Option<u32>,
    ) -> Result<String, AppError> {
        self.request_builder().user_favorite_list(
            sec_uid,
            max_cursor.unwrap_or("0"),
            count.unwrap_or(18),
        )
    }

    /// Build the user-recommend-list URL for a Douyin account.
    #[doc(alias = "getUserRecommendList")]
    pub fn user_recommend_list(
        &self,
        sec_uid: &str,
        max_cursor: Option<&str>,
        count: Option<u32>,
    ) -> Result<String, AppError> {
        self.request_builder().user_recommend_list(
            sec_uid,
            max_cursor.unwrap_or("0"),
            count.unwrap_or(18),
        )
    }

    /// Build the user-profile URL for a Douyin account.
    #[doc(alias = "getUserProfile")]
    pub fn user_profile(&self, sec_uid: &str) -> Result<String, AppError> {
        self.request_builder().user_profile(sec_uid)
    }

    /// Build the suggest-words URL for a Douyin query.
    #[doc(alias = "getSuggestWords")]
    pub fn suggest_words(&self, query: &str) -> Result<String, AppError> {
        self.request_builder().suggest_words(query)
    }

    /// Build one Douyin search URL.
    pub fn search(
        &self,
        query: &str,
        search_type: DouyinSearchType,
        count: Option<u32>,
        search_id: Option<&str>,
    ) -> Result<String, AppError> {
        self.request_builder()
            .search(query, search_type, count.unwrap_or(10), search_id)
    }

    /// Build the dynamic-emoji URL.
    #[doc(alias = "getDynamicEmojiList")]
    pub fn dynamic_emoji_list(&self) -> Result<String, AppError> {
        self.request_builder().dynamic_emoji_list()
    }

    /// Build the music-info URL for a Douyin music id.
    #[doc(alias = "getMusicInfo")]
    pub fn music_info(&self, music_id: &str) -> Result<String, AppError> {
        self.request_builder().music_info(music_id)
    }

    /// Build the live-room-info URL for a Douyin live room.
    #[doc(alias = "getLiveRoomInfo")]
    pub fn live_room_info(&self, room_id: &str, web_rid: &str) -> Result<String, AppError> {
        self.request_builder().live_room_info(room_id, web_rid)
    }

    /// Build the login-qrcode URL.
    #[doc(alias = "getLoginQrcode")]
    pub fn login_qrcode(&self, verify_fp: Option<&str>) -> Result<String, AppError> {
        self.request_builder()
            .login_qrcode(verify_fp.unwrap_or(&self.verify_fp))
    }

    /// Build the danmaku-list URL for a Douyin aweme.
    #[doc(alias = "getDanmakuList")]
    pub fn danmaku_list(
        &self,
        aweme_id: &str,
        start_time: Option<u64>,
        end_time: Option<u64>,
        duration: u64,
    ) -> Result<String, AppError> {
        self.request_builder().danmaku_list(
            aweme_id,
            start_time.unwrap_or(0),
            end_time.unwrap_or(32_000),
            duration,
        )
    }

    fn request_builder(&self) -> DouyinRequestBuilder {
        DouyinRequestBuilder::new(
            self.user_agent.as_deref(),
            self.verify_fp.clone(),
            DouyinApiEndpoints::default(),
        )
    }
}

/// Create a public Douyin URL builder.
#[doc(alias = "createDouyinApiUrls")]
pub fn create_douyin_api_urls(user_agent: Option<&str>) -> DouyinApiUrls {
    DouyinApiUrls::new(user_agent)
}
