use std::{collections::BTreeMap, time::Duration};

use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::{error::AppError, platforms::internal::random::now_unix_secs};

use super::super::sign::{build_wbi_query_from_url, extract_wbi_keys_from_nav_body};
use super::{
    BilibiliFetcher,
    helper::{append_query_segments, ensure_bilibili_success, extract_signature_suffix_from_query},
    requests,
};

impl BilibiliFetcher {
    pub(super) async fn fetch_json<T>(&self, url: &str) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        self.fetch_json_with_referer(url, None).await
    }

    pub(super) async fn post_json<T>(&self, url: &str, body: &Value) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        let value = inject_upstream_payload(self.post_json_value(url, body).await?);
        Ok(serde_json::from_value(value)?)
    }

    pub(super) async fn fetch_json_with_referer<T>(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<T, AppError>
    where
        T: DeserializeOwned,
    {
        let value =
            inject_upstream_payload(self.fetch_json_value_with_referer(url, referer).await?);
        Ok(serde_json::from_value(value)?)
    }

    pub(super) async fn fetch_json_value(&self, url: &str) -> Result<Value, AppError> {
        self.fetch_json_value_with_referer(url, None).await
    }

    async fn post_json_value(&self, url: &str, body: &Value) -> Result<Value, AppError> {
        let response_body = self.send_post_json_request(url, body, None).await?;
        let value: Value = serde_json::from_str(&response_body)?;
        ensure_bilibili_success(url, &value)?;
        Ok(value)
    }

    async fn fetch_json_value_with_referer(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<Value, AppError> {
        let body = self.send_text_request(url, referer).await?;
        let value: Value = serde_json::from_str(&body)?;
        ensure_bilibili_success(url, &value)?;
        Ok(value)
    }

    pub(super) async fn sign_wbi_url(&self, base_url: &str) -> Result<String, AppError> {
        let nav_url = requests::login_status(self.api_base_url.as_ref())?;
        let nav_body = self.send_text_request(&nav_url, None).await?;
        let keys = extract_wbi_keys_from_nav_body(&nav_body)?;
        let query = build_wbi_query_from_url(base_url, &keys, now_unix_secs())?;
        let suffix = extract_signature_suffix_from_query(&query);

        if base_url.contains('?') {
            Ok(format!("{base_url}&{suffix}"))
        } else {
            Ok(format!("{base_url}?{suffix}"))
        }
    }

    pub(super) async fn build_playurl_request(&self, base_url: &str) -> Result<String, AppError> {
        if self.cookie_header().trim().is_empty() {
            return Ok(append_query_segments(
                base_url,
                &["platform=html5".to_owned()],
            ));
        }

        let nav_url = requests::login_status(self.api_base_url.as_ref())?;
        let nav_body = self.send_text_request(&nav_url, None).await?;
        let nav_value: Value = serde_json::from_str(&nav_body)?;
        ensure_bilibili_success(&nav_url, &nav_value)?;

        let keys = extract_wbi_keys_from_nav_body(&nav_body)?;
        let signed_query = build_wbi_query_from_url(base_url, &keys, now_unix_secs())?;
        let is_vip = nav_value
            .get("data")
            .and_then(|value| value.get("vipStatus"))
            .and_then(Value::as_i64)
            == Some(1);

        let mut segments = if is_vip {
            vec!["fnval=4048".to_owned(), "fourk=1".to_owned()]
        } else {
            vec!["qn=64".to_owned(), "fnval=16".to_owned()]
        };
        segments.push(extract_signature_suffix_from_query(&signed_query));

        Ok(append_query_segments(base_url, &segments))
    }

    pub(super) fn cookie_header(&self) -> &str {
        self.request_profile
            .headers
            .get("Cookie")
            .or_else(|| self.request_profile.headers.get("cookie"))
            .map(String::as_str)
            .unwrap_or_default()
    }

    pub(super) async fn send_text_request(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<String, AppError> {
        let (_, body) = self.send_text_request_with_headers(url, referer).await?;
        Ok(body)
    }

    pub(super) async fn send_bytes_request(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<Vec<u8>, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(self.request_profile.timeout_ms))
            .build()?;
        let headers = self.build_headers(referer)?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            match client.get(url).headers(headers.clone()).send().await {
                Ok(response) => {
                    let status = response.status();
                    let body = response.bytes().await.unwrap_or_default();

                    if !status.is_success() {
                        return Err(AppError::UpstreamResponse {
                            status: Some(status),
                            message: format!("request to {url} returned binary body"),
                        });
                    }

                    return Ok(body.to_vec());
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

    async fn send_post_json_request(
        &self,
        url: &str,
        body: &Value,
        referer: Option<&str>,
    ) -> Result<String, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(self.request_profile.timeout_ms))
            .build()?;
        let headers = self.build_headers(referer)?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            match client
                .post(url)
                .headers(headers.clone())
                .json(body)
                .send()
                .await
            {
                Ok(response) => {
                    let status = response.status();
                    let response_body = response.text().await.unwrap_or_default();

                    if !status.is_success() {
                        return Err(AppError::UpstreamResponse {
                            status: Some(status),
                            message: format!("request to {url} returned `{response_body}`"),
                        });
                    }

                    return Ok(response_body);
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

    pub(super) async fn send_text_request_with_headers(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<(HeaderMap, String), AppError> {
        let client = Client::builder()
            .timeout(Duration::from_millis(self.request_profile.timeout_ms))
            .build()?;
        let headers = self.build_headers(referer)?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            match client.get(url).headers(headers.clone()).send().await {
                Ok(response) => {
                    let status = response.status();
                    let response_headers = response.headers().clone();
                    let body = response.text().await.unwrap_or_default();

                    if !status.is_success() {
                        return Err(AppError::UpstreamResponse {
                            status: Some(status),
                            message: format!("request to {url} returned `{body}`"),
                        });
                    }

                    return Ok((response_headers, body));
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

    fn build_headers(&self, referer: Option<&str>) -> Result<HeaderMap, AppError> {
        let mut headers = self.request_profile.headers.clone();

        if let Some(referer) = referer {
            headers.insert("Referer".into(), referer.to_owned());
        }

        header_map_from_headers(&headers)
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
