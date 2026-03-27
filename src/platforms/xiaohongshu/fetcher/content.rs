use crate::error::AppError;

use super::super::{
    XiaohongshuMethod,
    api::{XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions, XiaohongshuNoteDetailOptions},
    types::{
        XiaohongshuEmojiList, XiaohongshuHomeFeed, XiaohongshuNoteComments, XiaohongshuNoteDetail,
    },
};
use super::XiaohongshuFetcher;

impl XiaohongshuFetcher {
    /// Fetch the Xiaohongshu home feed.
    #[doc(alias = "fetchHomeFeed")]
    pub async fn fetch_home_feed(
        &self,
        options: &XiaohongshuHomeFeedOptions,
    ) -> Result<XiaohongshuHomeFeed, AppError> {
        let request = self.request_builder().home_feed(options)?;
        self.fetch_signed_json(
            XiaohongshuMethod::Post,
            &request.api_path,
            &request.url,
            request.params.as_ref(),
            request.body.as_ref(),
        )
        .await
    }

    /// Fetch one Xiaohongshu note detail payload.
    #[doc(alias = "fetchNoteDetail")]
    pub async fn fetch_note_detail(
        &self,
        options: &XiaohongshuNoteDetailOptions,
    ) -> Result<XiaohongshuNoteDetail, AppError> {
        let request = self.request_builder().note_detail(options)?;
        self.fetch_signed_json(
            XiaohongshuMethod::Post,
            &request.api_path,
            &request.url,
            request.params.as_ref(),
            request.body.as_ref(),
        )
        .await
    }

    /// Fetch one page of Xiaohongshu note comments.
    #[doc(alias = "fetchNoteComments")]
    pub async fn fetch_note_comments(
        &self,
        options: &XiaohongshuCommentsOptions,
    ) -> Result<XiaohongshuNoteComments, AppError> {
        let request = self.request_builder().note_comments(options)?;
        self.fetch_signed_json(
            XiaohongshuMethod::Get,
            &request.api_path,
            &request.url,
            request.params.as_ref(),
            request.body.as_ref(),
        )
        .await
    }

    /// Fetch the Xiaohongshu emoji catalog.
    #[doc(alias = "fetchEmojiList")]
    pub async fn fetch_emoji_list(&self) -> Result<XiaohongshuEmojiList, AppError> {
        let request = self.request_builder().emoji_list()?;
        self.fetch_signed_json(
            XiaohongshuMethod::Get,
            &request.api_path,
            &request.url,
            request.params.as_ref(),
            request.body.as_ref(),
        )
        .await
    }
}
