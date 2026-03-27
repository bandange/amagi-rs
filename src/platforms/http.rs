use std::time::Duration;

use reqwest::{
    Client, Method, Response,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::{catalog::HttpMethod, client::RequestProfile, error::AppError};

/// Shared JSON transport used by migrated platform fetchers.
#[derive(Debug, Clone)]
pub(super) struct JsonTransport {
    client: Client,
    method: Method,
    headers: HeaderMap,
    max_retries: u32,
}

impl JsonTransport {
    /// Build a JSON transport from a resolved [`RequestProfile`].
    pub(super) fn new(profile: RequestProfile) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(profile.timeout_ms))
            .build()?;

        Ok(Self {
            client,
            method: http_method_to_reqwest(profile.method),
            headers: header_map_from_profile(&profile)?,
            max_retries: profile.max_retries,
        })
    }

    /// Send a JSON request and decode a JSON response with the configured retry policy.
    pub(super) async fn send_json<T, B>(&self, url: &str, body: &B) -> Result<T, AppError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let attempt_count = self.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            let request = self
                .client
                .request(self.method.clone(), url)
                .headers(self.headers.clone())
                .json(body);

            match request.send().await {
                Ok(response) => return decode_response(url, response).await,
                Err(error) if attempt + 1 < attempt_count && is_retryable(&error) => {
                    last_error = Some(error);
                }
                Err(error) => return Err(error.into()),
            }
        }

        Err(last_error
            .map(AppError::from)
            .unwrap_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("request to {url} did not complete"),
            }))
    }
}

fn http_method_to_reqwest(method: HttpMethod) -> Method {
    match method {
        HttpMethod::Get => Method::GET,
        HttpMethod::Post => Method::POST,
        HttpMethod::Put => Method::PUT,
        HttpMethod::Delete => Method::DELETE,
        HttpMethod::Patch => Method::PATCH,
    }
}

fn header_map_from_profile(profile: &RequestProfile) -> Result<HeaderMap, AppError> {
    let mut headers = HeaderMap::new();

    for (name, value) in &profile.headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| AppError::InvalidRequestConfig(format!("invalid header name `{name}`")))?;
        let header_value = HeaderValue::from_str(value).map_err(|_| {
            AppError::InvalidRequestConfig(format!("invalid header value for `{name}`"))
        })?;
        headers.insert(header_name, header_value);
    }

    Ok(headers)
}

async fn decode_response<T>(url: &str, response: Response) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::UpstreamResponse {
            status: Some(status),
            message: format!("request to {url} returned `{body}`"),
        });
    }

    Ok(response.json::<T>().await?)
}

fn is_retryable(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}
