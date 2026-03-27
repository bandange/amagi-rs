use std::time::Duration;

use reqwest::{Client, header::HeaderMap};
use serde_json::{Value, json};

use crate::error::AppError;

use super::super::{
    sign::{build_signed_url_with_a_bogus, build_signed_url_with_x_bogus},
    types::DouyinSearchType,
};
use super::{
    DouyinFetcher, DouyinSignType,
    payload::{
        extract_array_field, filter_search_responses, has_more_value, header_map_from_headers,
        parse_douyin_multi_json, parse_json_payload, set_array_field, set_field, stringify_cursor,
        validate_douyin_response,
    },
    requests::DouyinRequestBuilder,
};

impl DouyinFetcher {
    pub(super) async fn fetch_work_payload(&self, aweme_id: &str) -> Result<Value, AppError> {
        let url = self.request_builder().work_detail(aweme_id)?;
        self.fetch_json(&url, DouyinSignType::ABogus, None::<String>)
            .await
    }

    pub(super) async fn fetch_user_list(
        &self,
        sec_uid: &str,
        number: Option<u32>,
        max_cursor: Option<&str>,
        build_url: impl Fn(&DouyinRequestBuilder, &str, &str, u32) -> Result<String, AppError>,
    ) -> Result<Value, AppError> {
        let target = number.unwrap_or(18);
        let referer = format!("https://www.douyin.com/user/{sec_uid}");
        let mut next_cursor = max_cursor.unwrap_or("0").to_owned();
        let mut items = Vec::new();
        let mut last_response = json!({
            "aweme_list": [],
            "max_cursor": next_cursor,
            "has_more": 0
        });

        while items.len() < target as usize {
            let request_count = (target as usize - items.len()).min(18) as u32;
            let url = build_url(
                &self.request_builder(),
                sec_uid,
                &next_cursor,
                request_count,
            )?;
            let response = self
                .fetch_json(&url, DouyinSignType::ABogus, Some(referer.clone()))
                .await?;
            let current_items = extract_array_field(&response, "aweme_list");
            items.extend(current_items.clone());
            next_cursor = response
                .get("max_cursor")
                .and_then(stringify_cursor)
                .unwrap_or(next_cursor);
            let has_more = has_more_value(response.get("has_more"));
            last_response = response;
            if !has_more || current_items.is_empty() {
                break;
            }
        }

        set_array_field(
            &mut last_response,
            "aweme_list",
            items,
            Some(target as usize),
        );
        set_field(&mut last_response, "max_cursor", Value::String(next_cursor));
        Ok(last_response)
    }

    pub(super) async fn fetch_search_json(
        &self,
        url: &str,
        referer: &str,
        search_type: DouyinSearchType,
    ) -> Result<Value, AppError> {
        let body = self.send_text(url, Some(referer)).await?;

        match search_type {
            DouyinSearchType::General => {
                if let Ok(value) = serde_json::from_str::<Value>(&body) {
                    validate_douyin_response(url, &value)?;
                    return Ok(value);
                }

                let responses = filter_search_responses(parse_douyin_multi_json(&body));
                if responses.is_empty() {
                    return Err(AppError::UpstreamResponse {
                        status: None,
                        message: format!(
                            "douyin general search returned no valid JSON chunks for {url}"
                        ),
                    });
                }

                let mut merged_data = Vec::new();
                let mut last_valid = responses
                    .last()
                    .cloned()
                    .unwrap_or_else(|| json!({ "data": [] }));

                for response in responses {
                    merged_data.extend(extract_array_field(&response, "data"));
                }

                set_array_field(&mut last_valid, "data", merged_data, None);
                Ok(last_valid)
            }
            DouyinSearchType::User | DouyinSearchType::Video => parse_json_payload(url, &body),
        }
    }

    pub(super) async fn fetch_json(
        &self,
        url: &str,
        sign_type: DouyinSignType,
        referer: Option<impl AsRef<str>>,
    ) -> Result<Value, AppError> {
        let signed_url = self.sign_url(url, sign_type)?;
        let body = self
            .send_text(&signed_url, referer.as_ref().map(AsRef::as_ref))
            .await?;
        parse_json_payload(&signed_url, &body)
    }

    async fn send_text(&self, url: &str, referer: Option<&str>) -> Result<String, AppError> {
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

    fn build_headers(&self, referer: Option<&str>) -> Result<HeaderMap, AppError> {
        let mut headers = self.request_profile.headers.clone();

        if let Some(referer) = referer {
            headers.insert("Referer".into(), referer.to_owned());
        }

        header_map_from_headers(&headers)
    }

    pub(super) fn request_builder(&self) -> DouyinRequestBuilder {
        DouyinRequestBuilder::new(
            self.user_agent(),
            self.verify_fp.clone(),
            self.endpoints.clone(),
        )
    }

    fn user_agent(&self) -> Option<&str> {
        self.request_profile
            .headers
            .get("User-Agent")
            .map(String::as_str)
            .or_else(|| {
                self.request_profile
                    .headers
                    .get("user-agent")
                    .map(String::as_str)
            })
    }

    fn sign_url(&self, url: &str, sign_type: DouyinSignType) -> Result<String, AppError> {
        match sign_type {
            DouyinSignType::None => Ok(url.to_owned()),
            DouyinSignType::ABogus => build_signed_url_with_a_bogus(url, self.user_agent()),
            DouyinSignType::XBogus => build_signed_url_with_x_bogus(url, self.user_agent()),
        }
    }
}

fn is_retryable(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}
