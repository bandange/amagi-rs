use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{error::AppError, platforms::http::JsonTransport};

use super::{EMOJI_LIST_OPERATION_NAME, EMOJI_LIST_QUERY, KuaishouFetcher};

impl KuaishouFetcher {
    pub(super) async fn send_graphql_data<TRequest>(
        &self,
        request: &TRequest,
    ) -> Result<Value, AppError>
    where
        TRequest: Serialize + ?Sized,
    {
        let transport = JsonTransport::new(self.request_profile.clone())?;
        let response: GraphqlResponse<Value> = transport
            .send_json(self.graphql_endpoint.as_ref(), request)
            .await?;

        if let Some(errors) = response.errors {
            let message = errors
                .into_iter()
                .map(|error| error.message)
                .collect::<Vec<_>>()
                .join("; ");
            return Err(AppError::UpstreamResponse {
                status: None,
                message: format!("kuaishou graphql returned errors: {message}"),
            });
        }

        response.data.ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: "kuaishou graphql response missing data".into(),
        })
    }
}

#[derive(Debug, Serialize)]
pub(super) struct EmojiListRequest {
    #[serde(rename = "operationName")]
    operation_name: &'static str,
    variables: EmojiListVariables,
    query: &'static str,
}

impl Default for EmojiListRequest {
    fn default() -> Self {
        Self {
            operation_name: EMOJI_LIST_OPERATION_NAME,
            variables: EmojiListVariables,
            query: EMOJI_LIST_QUERY,
        }
    }
}

#[derive(Debug, Default, Serialize)]
struct EmojiListVariables;

#[derive(Debug, Deserialize)]
struct GraphqlResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphqlError>>,
}

#[derive(Debug, Deserialize)]
struct GraphqlError {
    message: String,
}
