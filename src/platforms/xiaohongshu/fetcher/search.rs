use crate::error::AppError;

use super::super::{
    XiaohongshuMethod, api::XiaohongshuSearchNotesOptions, types::XiaohongshuSearchNotes,
};
use super::XiaohongshuFetcher;

impl XiaohongshuFetcher {
    /// Search Xiaohongshu notes.
    #[doc(alias = "searchNotes")]
    pub async fn search_notes(
        &self,
        options: &XiaohongshuSearchNotesOptions,
    ) -> Result<XiaohongshuSearchNotes, AppError> {
        let request = self.request_builder().search_notes(options, None)?;
        self.fetch_signed_json(
            XiaohongshuMethod::Post,
            &request.api_path,
            &request.url,
            request.params.as_ref(),
            request.body.as_ref(),
        )
        .await
    }
}
