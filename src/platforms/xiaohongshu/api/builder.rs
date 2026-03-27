//! Public Xiaohongshu request builders migrated from the TypeScript platform layer.

use crate::error::AppError;

use super::super::fetcher::requests::{XiaohongshuApiEndpoints, XiaohongshuRequestBuilder};
use super::types::{
    XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions, XiaohongshuNoteDetailOptions,
    XiaohongshuRequestSpec, XiaohongshuSearchNotesOptions, XiaohongshuUserNotesOptions,
    XiaohongshuUserProfileOptions,
};

/// Public Xiaohongshu API URL builder.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias = "xiaohongshuApiUrls")]
pub struct XiaohongshuApiUrls {
    endpoints: XiaohongshuApiEndpoints,
}

impl Default for XiaohongshuApiUrls {
    fn default() -> Self {
        Self::new()
    }
}

impl XiaohongshuApiUrls {
    /// Create a Xiaohongshu API builder with the default upstream endpoints.
    pub fn new() -> Self {
        Self {
            endpoints: XiaohongshuApiEndpoints::default(),
        }
    }

    /// Create a Xiaohongshu API builder with explicit upstream endpoints.
    pub fn with_base_urls(
        api_base_url: impl Into<String>,
        web_base_url: impl Into<String>,
    ) -> Self {
        Self {
            endpoints: XiaohongshuApiEndpoints {
                api_base_url: api_base_url.into(),
                web_base_url: web_base_url.into(),
            },
        }
    }

    /// Build the Xiaohongshu home-feed request.
    #[doc(alias = "homeFeed")]
    pub fn home_feed(
        &self,
        options: &XiaohongshuHomeFeedOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().home_feed(options)
    }

    /// Build the Xiaohongshu note-detail request.
    #[doc(alias = "noteDetail")]
    pub fn note_detail(
        &self,
        options: &XiaohongshuNoteDetailOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().note_detail(options)
    }

    /// Build the Xiaohongshu note-comments request.
    #[doc(alias = "noteComments")]
    pub fn note_comments(
        &self,
        options: &XiaohongshuCommentsOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().note_comments(options)
    }

    /// Build the Xiaohongshu user-profile page request.
    #[doc(alias = "userProfile")]
    pub fn user_profile(
        &self,
        options: &XiaohongshuUserProfileOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().user_profile(options)
    }

    /// Build the Xiaohongshu user-note-list request.
    #[doc(alias = "userNoteList")]
    pub fn user_note_list(
        &self,
        options: &XiaohongshuUserNotesOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().user_note_list(options)
    }

    /// Build the Xiaohongshu emoji-list request.
    #[doc(alias = "emojiList")]
    pub fn emoji_list(&self) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().emoji_list()
    }

    /// Build the Xiaohongshu note-search request.
    #[doc(alias = "searchNotes")]
    pub fn search_notes(
        &self,
        options: &XiaohongshuSearchNotesOptions,
    ) -> Result<XiaohongshuRequestSpec, AppError> {
        self.request_builder().search_notes(options, None)
    }

    fn request_builder(&self) -> XiaohongshuRequestBuilder {
        XiaohongshuRequestBuilder::new(self.endpoints.clone())
    }
}

/// Create a public Xiaohongshu API builder.
#[doc(alias = "createXiaohongshuApiUrls")]
pub fn create_xiaohongshu_api_urls() -> XiaohongshuApiUrls {
    XiaohongshuApiUrls::new()
}
