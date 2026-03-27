use serde_json::Value;

use crate::error::AppError;

use super::super::types::{BilibiliCommentReplies, BilibiliComments};
use super::{
    BilibiliFetcher,
    helper::{dedupe_comment_replies, set_json_array_field},
    requests,
};

impl BilibiliFetcher {
    /// Fetch Bilibili comments for one subject and merge pages until the target count.
    ///
    /// # Errors
    ///
    /// Returns an error when WBI keys cannot be resolved, the upstream request
    /// fails, or the response body contains a non-zero API status code.
    #[doc(alias = "fetchComments")]
    pub async fn fetch_comments(
        &self,
        oid: u64,
        comment_type: u32,
        number: Option<u32>,
        mode: Option<u32>,
    ) -> Result<BilibiliComments, AppError> {
        let target = number.unwrap_or(20) as usize;
        let mut merged_replies = Vec::new();
        let mut next_offset: Option<String> = None;
        let mut is_end = false;
        let mut last_response = Value::Null;

        while merged_replies.len() < target && !is_end {
            let signed_url = self
                .sign_wbi_url(&requests::comments(
                    self.api_base_url.as_ref(),
                    oid,
                    comment_type,
                    mode,
                    next_offset.as_deref(),
                )?)
                .await?;
            let response = self.fetch_json_value(&signed_url).await?;
            let current_replies = response
                .get("data")
                .and_then(|value| value.get("replies"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            merged_replies.extend(current_replies);
            merged_replies = dedupe_comment_replies(merged_replies);

            next_offset = response
                .get("data")
                .and_then(|value| value.get("cursor"))
                .and_then(|value| value.get("pagination_reply"))
                .and_then(|value| value.get("next_offset"))
                .and_then(|value| match value {
                    Value::String(value) => Some(value.clone()),
                    Value::Object(_) | Value::Array(_) => serde_json::to_string(value).ok(),
                    _ => None,
                });
            is_end = response
                .get("data")
                .and_then(|value| value.get("cursor"))
                .and_then(|value| value.get("is_end"))
                .and_then(Value::as_bool)
                .unwrap_or(true);
            last_response = response;

            if next_offset.is_none() {
                break;
            }
        }

        set_json_array_field(
            &mut last_response,
            &["data", "replies"],
            merged_replies,
            Some(target),
        );
        serde_json::from_value(last_response).map_err(AppError::from)
    }

    /// Fetch replies for one Bilibili root comment.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero API status code.
    #[doc(alias = "fetchCommentReplies")]
    pub async fn fetch_comment_replies(
        &self,
        oid: u64,
        comment_type: u32,
        root: u64,
        number: Option<u32>,
    ) -> Result<BilibiliCommentReplies, AppError> {
        self.fetch_json(&requests::comment_replies(
            self.api_base_url.as_ref(),
            oid,
            comment_type,
            root,
            number,
        )?)
        .await
    }
}
