use crate::error::AppError;

use super::{
    super::types::{
        KuaishouVideoWork, KuaishouVideoWorkData, KuaishouWorkComments, KuaishouWorkCommentsData,
    },
    KuaishouFetcher, requests,
};

impl KuaishouFetcher {
    /// Fetch one Kuaishou video work through the platform GraphQL endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails, the GraphQL response
    /// contains errors, or the payload cannot be decoded.
    #[doc(alias = "fetchVideoWork")]
    pub async fn fetch_video_work(&self, photo_id: &str) -> Result<KuaishouVideoWork, AppError> {
        let data = self
            .send_graphql_data(&requests::video_work_request(photo_id))
            .await?;
        let parsed = serde_json::from_value::<KuaishouVideoWorkData>(data.clone())?;
        Ok(KuaishouVideoWork {
            data: parsed,
            upstream_payload: data,
        })
    }

    /// Fetch comments for one Kuaishou work through the GraphQL endpoint.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails, the GraphQL response
    /// contains errors, or the payload cannot be decoded.
    #[doc(alias = "fetchWorkComments")]
    pub async fn fetch_work_comments(
        &self,
        photo_id: &str,
    ) -> Result<KuaishouWorkComments, AppError> {
        let data = self
            .send_graphql_data(&requests::work_comments_request(photo_id))
            .await?;
        let parsed = serde_json::from_value::<KuaishouWorkCommentsData>(data.clone())?;
        Ok(KuaishouWorkComments {
            data: parsed,
            upstream_payload: data,
        })
    }
}
