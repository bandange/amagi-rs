use std::{collections::BTreeMap, time::Duration};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::error::AppError;

use super::super::{OrderedJson, XiaohongshuMethod, types::XiaohongshuJsonResponse};
use super::{XiaohongshuFetcher, requests::XiaohongshuRequestBuilder};

impl XiaohongshuFetcher {
    pub(super) fn request_builder(&self) -> XiaohongshuRequestBuilder {
        XiaohongshuRequestBuilder::new(super::requests::XiaohongshuApiEndpoints {
            api_base_url: self.api_base_url.to_string(),
            web_base_url: self.web_base_url.to_string(),
        })
    }

    pub(super) async fn fetch_signed_json<T>(
        &self,
        method: XiaohongshuMethod,
        sign_path: &str,
        url: &str,
        params: Option<&OrderedJson>,
        body: Option<&OrderedJson>,
    ) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .send_signed_text_request(method, sign_path, url, params, body)
            .await?;
        let value = inject_upstream_payload(self.validate_json_response(url, &response)?);
        Ok(serde_json::from_value(value)?)
    }

    pub(super) async fn fetch_signed_text(
        &self,
        method: XiaohongshuMethod,
        sign_path: &str,
        url: &str,
        params: Option<&OrderedJson>,
        body: Option<&OrderedJson>,
    ) -> Result<String, AppError> {
        self.send_signed_text_request(method, sign_path, url, params, body)
            .await
    }

    async fn send_signed_text_request(
        &self,
        method: XiaohongshuMethod,
        sign_path: &str,
        url: &str,
        params: Option<&OrderedJson>,
        body: Option<&OrderedJson>,
    ) -> Result<String, AppError> {
        let signed_headers = self.build_signed_headers(method, sign_path, params, body)?;
        let header_map = header_map_from_headers(&signed_headers)?;
        let client = Client::builder()
            .timeout(Duration::from_millis(self.request_profile.timeout_ms))
            .build()?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let body_string = body.map(OrderedJson::to_json_string).transpose()?;
        let mut last_error = None;

        for attempt in 0..attempt_count {
            let request = match method {
                XiaohongshuMethod::Get => client.get(url).headers(header_map.clone()),
                XiaohongshuMethod::Post => {
                    let request = client.post(url).headers(header_map.clone());
                    match body_string.as_deref() {
                        Some(value) => request.body(value.to_owned()),
                        None => request,
                    }
                }
            };

            match request.send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.text().await.unwrap_or_default();

                    if !status.is_success() {
                        return Err(AppError::UpstreamResponse {
                            status: Some(status),
                            message: format!("request to {url} returned `{body}`"),
                        });
                    }

                    return Ok(body);
                }
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

    fn build_signed_headers(
        &self,
        method: XiaohongshuMethod,
        sign_path: &str,
        params: Option<&OrderedJson>,
        body: Option<&OrderedJson>,
    ) -> Result<BTreeMap<String, String>, AppError> {
        let mut headers = self.request_profile.headers.clone();
        let sign_params = match method {
            // The verified TypeScript flow signs GET requests against the API path
            // only. Query parameters are still sent on the wire, but they are not
            // mixed into the `x-s` input material.
            XiaohongshuMethod::Get => None,
            XiaohongshuMethod::Post => params,
        };
        let mut signing = self.signing.lock().map_err(|_| {
            AppError::InvalidRequestConfig("xiaohongshu signer lock poisoned".into())
        })?;
        let signed = {
            let super::XiaohongshuSigningState {
                signer,
                session,
                cookies,
            } = &mut *signing;
            let cookies = cookies.clone();

            signer.sign_headers(
                method,
                sign_path,
                &cookies,
                Some("xhs-pc-web"),
                sign_params,
                body,
                None,
                Some(session),
            )?
        };

        headers.insert("x-s".into(), signed.x_s);
        headers.insert("x-s-common".into(), signed.x_s_common);
        headers.insert("x-t".into(), signed.x_t);
        headers.insert("x-b3-traceid".into(), signed.x_b3_traceid);
        headers.insert("x-xray-traceid".into(), signed.x_xray_traceid);

        Ok(headers)
    }

    fn validate_json_response(&self, url: &str, body: &str) -> Result<Value, AppError> {
        let value: Value = serde_json::from_str(body)?;
        let envelope: XiaohongshuJsonResponse<Value> = serde_json::from_value(value.clone())?;

        if envelope.code != 0 {
            return Err(AppError::UpstreamResponse {
                status: None,
                message: format!(
                    "xiaohongshu request to {url} failed with code {}: {}",
                    envelope.code, envelope.msg
                ),
            });
        }

        Ok(value)
    }
}

fn header_map_from_headers(headers: &BTreeMap<String, String>) -> Result<HeaderMap, AppError> {
    let mut header_map = HeaderMap::new();

    for (name, value) in headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| AppError::InvalidRequestConfig(format!("invalid header name `{name}`")))?;
        let header_value = HeaderValue::from_str(value).map_err(|_| {
            AppError::InvalidRequestConfig(format!("invalid header value for `{name}`"))
        })?;
        header_map.insert(header_name, header_value);
    }

    Ok(header_map)
}

fn is_retryable(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}

fn inject_upstream_payload(value: Value) -> Value {
    match value {
        Value::Object(mut object) => {
            let upstream_payload = normalize_upstream_payload(&object);
            object.insert("upstream_payload".to_owned(), upstream_payload);
            Value::Object(object)
        }
        other => other,
    }
}

fn normalize_upstream_payload(object: &Map<String, Value>) -> Value {
    object.get("data").cloned().unwrap_or(Value::Null)
}
