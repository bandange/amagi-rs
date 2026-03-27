use std::collections::BTreeMap;

use reqwest::{
    Client, Method, Response,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde_json::{Map, Value};

use crate::error::AppError;

use super::super::sign::{
    KuaishouLiveApiMethod, KuaishouLiveApiRequest, generate_kww, sign_live_api_request,
};
use super::{KuaishouFetcher, value::json_object_to_value};

impl KuaishouFetcher {
    pub(super) fn http_client(&self) -> Result<Client, AppError> {
        Ok(Client::builder()
            .timeout(std::time::Duration::from_millis(
                self.request_profile.timeout_ms,
            ))
            .build()?)
    }

    pub(super) fn cookie(&self) -> Option<&str> {
        self.request_profile
            .headers
            .get("Cookie")
            .map(String::as_str)
            .or_else(|| {
                self.request_profile
                    .headers
                    .get("cookie")
                    .map(String::as_str)
            })
    }

    fn build_live_api_headers(
        &self,
        request: &KuaishouLiveApiRequest,
        referer_path: &str,
        signed_headers: &BTreeMap<String, String>,
    ) -> Result<HeaderMap, AppError> {
        let mut headers = self.request_profile.headers.clone();

        headers.insert("Accept".into(), "application/json, text/plain, */*".into());
        headers.insert("Accept-Encoding".into(), "gzip, deflate, br, zstd".into());
        headers.insert(
            "Accept-Language".into(),
            "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".into(),
        );
        headers.insert("Cache-Control".into(), "no-cache".into());
        headers.insert("Pragma".into(), "no-cache".into());
        headers.insert("Priority".into(), "u=0, i".into());
        headers.insert(
            "Referer".into(),
            format!("https://live.kuaishou.com/{referer_path}"),
        );
        headers.insert(
            "Sec-Ch-Ua".into(),
            "\"Chromium\";v=\"146\", \"Not-A.Brand\";v=\"24\", \"Google Chrome\";v=\"146\"".into(),
        );
        headers.insert("Sec-Ch-Ua-Mobile".into(), "?0".into());
        headers.insert("Sec-Ch-Ua-Platform".into(), "\"Windows\"".into());
        headers.insert("Sec-Fetch-Dest".into(), "empty".into());
        headers.insert("Sec-Fetch-Mode".into(), "cors".into());
        headers.insert("Sec-Fetch-Site".into(), "same-origin".into());
        headers.insert(
            "User-Agent".into(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36"
                .into(),
        );

        if matches!(request.method, KuaishouLiveApiMethod::Post) {
            headers.insert("Content-Type".into(), "application/json".into());
        }

        for (name, value) in signed_headers {
            headers.insert(name.clone(), value.clone());
        }

        if !headers.contains_key("kww") {
            let kww = generate_kww(self.cookie());
            if !kww.is_empty() {
                headers.insert("kww".into(), kww);
            }
        }

        header_map_from_headers(&headers)
    }

    pub(super) async fn send_live_api_request(
        &self,
        request: &KuaishouLiveApiRequest,
        referer_path: &str,
        allow_result_two: bool,
    ) -> Result<Value, AppError> {
        let signed = if request.requires_sign {
            Some(sign_live_api_request(request, self.cookie())?)
        } else {
            None
        };
        let empty_signed_headers = BTreeMap::new();
        let url = signed
            .as_ref()
            .map(|value| value.url.as_str())
            .unwrap_or(&request.url);
        let signed_headers = signed
            .as_ref()
            .map(|value| &value.headers)
            .unwrap_or(&empty_signed_headers);
        let headers = self.build_live_api_headers(request, referer_path, signed_headers)?;
        let method = live_api_method_to_reqwest(request.method);
        let body = (!request.body.is_empty()).then(|| json_object_to_value(&request.body));
        let client = self.http_client()?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            let mut request_builder = client.request(method.clone(), url).headers(headers.clone());

            if matches!(request.method, KuaishouLiveApiMethod::Post) {
                request_builder =
                    request_builder.json(body.as_ref().unwrap_or(&Value::Object(Map::new())));
            }

            match request_builder.send().await {
                Ok(response) => {
                    return decode_live_api_response(url, response, allow_result_two).await;
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

fn live_api_method_to_reqwest(method: KuaishouLiveApiMethod) -> Method {
    match method {
        KuaishouLiveApiMethod::Get => Method::GET,
        KuaishouLiveApiMethod::Post => Method::POST,
    }
}

async fn decode_live_api_response(
    url: &str,
    response: Response,
    allow_result_two: bool,
) -> Result<Value, AppError> {
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::UpstreamResponse {
            status: Some(status),
            message: format!("request to {url} returned `{body}`"),
        });
    }

    let value = response.json::<Value>().await?;

    if !allow_result_two
        && value
            .get("result")
            .and_then(Value::as_i64)
            .is_some_and(|result| result == 2)
    {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!("request to {url} returned result=2"),
        });
    }

    Ok(value)
}

fn is_retryable(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}
