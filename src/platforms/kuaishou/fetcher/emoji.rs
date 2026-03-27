use crate::error::AppError;

use super::{
    super::types::{KuaishouEmojiList, KuaishouEmojiListData},
    KuaishouFetcher,
    graphql::EmojiListRequest,
};

impl KuaishouFetcher {
    /// Fetch the Kuaishou emoji catalog from the platform GraphQL endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails, the GraphQL response
    /// contains errors, or the payload cannot be decoded.
    #[doc(alias = "fetchEmojiList")]
    pub async fn fetch_emoji_list(&self) -> Result<KuaishouEmojiList, AppError> {
        let data = self.send_graphql_data(&EmojiListRequest::default()).await?;
        let parsed = serde_json::from_value::<KuaishouEmojiListData>(data.clone())?;
        Ok(KuaishouEmojiList {
            data: parsed,
            upstream_payload: data,
        })
    }
}
