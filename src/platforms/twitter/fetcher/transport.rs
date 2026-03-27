use std::{collections::BTreeMap, time::Duration};

use reqwest::{
    Client, Method, Url,
    header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue},
};
use serde_json::{Map, Value};

use crate::error::AppError;
use crate::platforms::internal::random::now_unix_secs;
use crate::platforms::twitter::sign::build_client_transaction_state;

use super::{CLIENT_TRANSACTION_CACHE_TTL_SECS, CachedTransactionState, TwitterFetcher};
use crate::platforms::twitter::auth::{
    TWITTER_DEFAULT_LANGUAGE, TWITTER_WEB_BEARER_TOKEN, extract_csrf_token,
};

impl TwitterFetcher {
    pub(super) async fn fetch_graphql_value(
        &self,
        url: &str,
        referer: Option<&str>,
    ) -> Result<Value, AppError> {
        let guest_token = if self.has_authenticated_session() {
            None
        } else {
            Some(self.activate_guest_token().await?)
        };
        let headers = self
            .graphql_headers(guest_token.as_deref(), url, referer)
            .await?;
        let body = self.send_text_request(Method::GET, url, headers).await?;
        let value: Value = serde_json::from_str(&body)?;
        ensure_graphql_success(url, &value)?;
        Ok(value)
    }

    async fn activate_guest_token(&self) -> Result<String, AppError> {
        let mut headers = self.request_profile.headers.clone();
        headers.remove("Cookie");
        headers.remove("cookie");
        headers.remove("x-csrf-token");
        headers.remove("x-twitter-auth-type");
        headers.insert(
            "Authorization".into(),
            format!("Bearer {TWITTER_WEB_BEARER_TOKEN}"),
        );
        headers.insert(
            "Referer".into(),
            format!("{}/", self.web_base_url.trim_end_matches('/')),
        );
        headers.insert("Origin".into(), self.web_base_url.to_string());

        let header_map = header_map_from_headers(&headers)?;
        let body = self
            .send_text_request(Method::POST, self.api_urls.guest_activate(), header_map)
            .await?;
        let value: Value = serde_json::from_str(&body)?;
        value
            .get("guest_token")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: "twitter guest activation did not return `guest_token`".into(),
            })
    }

    async fn graphql_headers(
        &self,
        guest_token: Option<&str>,
        url: &str,
        referer: Option<&str>,
    ) -> Result<HeaderMap, AppError> {
        let mut headers = self.request_profile.headers.clone();
        headers.insert(
            "Authorization".into(),
            format!("Bearer {TWITTER_WEB_BEARER_TOKEN}"),
        );
        headers.insert("x-twitter-active-user".into(), "yes".into());
        headers.insert(
            "x-twitter-client-language".into(),
            self.language().to_owned(),
        );
        headers.insert("Origin".into(), self.web_base_url.to_string());
        headers
            .entry("Referer".into())
            .or_insert_with(|| format!("{}/", self.web_base_url.trim_end_matches('/')));

        if let Some(referer) = referer {
            headers.insert("Referer".into(), referer.to_owned());
        }

        if let Some(guest_token) = guest_token {
            headers.insert("x-guest-token".into(), guest_token.to_owned());
        }

        if let Some(cookie) = self.cookie_header() {
            headers.insert("Cookie".into(), cookie.to_owned());

            if let Some(csrf_token) = extract_csrf_token(cookie) {
                headers.insert("x-csrf-token".into(), csrf_token);
                headers.insert("x-twitter-auth-type".into(), "OAuth2Session".into());
            }
        }

        headers.insert(
            "x-client-transaction-id".into(),
            self.client_transaction_id(url).await?,
        );

        header_map_from_headers(&headers)
    }

    fn language(&self) -> &str {
        self.request_profile
            .headers
            .get("x-twitter-client-language")
            .or_else(|| {
                self.request_profile
                    .headers
                    .get("X-Twitter-Client-Language")
            })
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
            .unwrap_or(TWITTER_DEFAULT_LANGUAGE)
    }

    pub(super) fn cookie_header(&self) -> Option<&str> {
        self.request_profile
            .headers
            .get("Cookie")
            .or_else(|| self.request_profile.headers.get("cookie"))
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
    }

    pub(super) fn ensure_authenticated_session(&self, capability: &str) -> Result<(), AppError> {
        if self.has_authenticated_session() {
            return Ok(());
        }

        Err(AppError::InvalidRequestConfig(format!(
            "twitter capability `{capability}` requires `AMAGI_TWITTER_COOKIE` with at least `auth_token` and `ct0`"
        )))
    }

    async fn client_transaction_id(&self, url: &str) -> Result<String, AppError> {
        let path = Url::parse(url)
            .map_err(|error| {
                AppError::InvalidRequestConfig(format!(
                    "invalid twitter graphql url `{url}`: {error}"
                ))
            })?
            .path()
            .to_owned();
        let state = self.client_transaction_state().await?;
        Ok(state.generate_transaction_id("GET", &path))
    }

    async fn client_transaction_state(
        &self,
    ) -> Result<crate::platforms::twitter::sign::ClientTransactionState, AppError> {
        let now = now_unix_secs();
        let cached = self
            .transaction_state
            .lock()
            .map_err(|_| {
                AppError::InvalidRequestConfig(
                    "twitter client transaction cache lock was poisoned".into(),
                )
            })?
            .clone();

        if let Some(cached) = cached
            .clone()
            .filter(|cached| cached.expires_at_unix_secs > now)
        {
            return Ok(cached.state);
        }

        let state = match build_client_transaction_state(
            &self.http_client()?,
            &self.request_profile.headers,
            self.web_base_url.as_ref(),
        )
        .await
        {
            Ok(state) => state,
            Err(error) => {
                if let Some(mut cached) = cached {
                    cached.expires_at_unix_secs =
                        now.saturating_add(CLIENT_TRANSACTION_CACHE_TTL_SECS);
                    *self.transaction_state.lock().map_err(|_| {
                        AppError::InvalidRequestConfig(
                            "twitter client transaction cache lock was poisoned".into(),
                        )
                    })? = Some(cached.clone());
                    return Ok(cached.state);
                }

                return Err(error);
            }
        };
        let cached = CachedTransactionState {
            state: state.clone(),
            expires_at_unix_secs: now.saturating_add(CLIENT_TRANSACTION_CACHE_TTL_SECS),
        };

        *self.transaction_state.lock().map_err(|_| {
            AppError::InvalidRequestConfig(
                "twitter client transaction cache lock was poisoned".into(),
            )
        })? = Some(cached);

        Ok(state)
    }

    async fn send_text_request(
        &self,
        method: Method,
        url: &str,
        headers: HeaderMap,
    ) -> Result<String, AppError> {
        let client = self.http_client()?;
        let attempt_count = self.request_profile.max_retries.saturating_add(1);
        let mut last_error = None;

        for attempt in 0..attempt_count {
            match client
                .request(method.clone(), url)
                .headers(headers.clone())
                .send()
                .await
            {
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

    fn http_client(&self) -> Result<Client, AppError> {
        Client::builder()
            .timeout(Duration::from_millis(self.request_profile.timeout_ms))
            .build()
            .map_err(AppError::from)
    }

    fn has_authenticated_session(&self) -> bool {
        let Some(cookie) = self.cookie_header() else {
            return false;
        };

        cookie_has_name(cookie, "auth_token") && extract_csrf_token(cookie).is_some()
    }
}

pub(super) fn value_at_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    Some(current)
}

pub(super) fn array_at_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a [Value]> {
    value_at_path(value, path)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
}

pub(super) fn string_at_path(value: &Value, path: &[&str]) -> Option<String> {
    value_at_path(value, path)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .and_then(normalize_optional_string)
}

pub(super) fn bool_at_path(value: &Value, path: &[&str]) -> Option<bool> {
    value_at_path(value, path).and_then(Value::as_bool)
}

pub(super) fn u64_at_path(value: &Value, path: &[&str]) -> Option<u64> {
    match value_at_path(value, path) {
        Some(Value::Number(number)) => number.as_u64(),
        Some(Value::String(text)) => text.parse::<u64>().ok(),
        _ => None,
    }
}

pub(super) fn normalize_optional_string(value: String) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

pub(super) fn unwrap_user_result<'a>(value: &'a Value) -> Option<&'a Value> {
    match value.get("__typename").and_then(Value::as_str) {
        Some("User") => Some(value),
        _ => value.get("result").and_then(unwrap_user_result),
    }
}

pub(super) fn unwrap_tweet_result<'a>(value: &'a Value) -> Option<&'a Value> {
    match value.get("__typename").and_then(Value::as_str) {
        Some("Tweet") => Some(value),
        Some("TweetWithVisibilityResults") => value.get("tweet"),
        _ => {
            if value.get("legacy").is_some() && value.get("rest_id").is_some() {
                Some(value)
            } else {
                value.get("result").and_then(unwrap_tweet_result)
            }
        }
    }
}

pub(super) fn normalize_upstream_payload(value: &Value) -> Value {
    if let Some(tweet) = unwrap_tweet_result(value) {
        if !std::ptr::eq(tweet, value) {
            return normalize_upstream_payload(tweet);
        }
    }

    if let Some(user) = unwrap_user_result(value) {
        if !std::ptr::eq(user, value) {
            return normalize_upstream_payload(user);
        }
    }

    match value {
        Value::Array(items) => Value::Array(items.iter().map(normalize_upstream_payload).collect()),
        Value::Object(map) => {
            if let Some(result) = map.get("result").filter(|_| map.len() == 1) {
                return normalize_upstream_payload(result);
            }

            Value::Object(
                map.iter()
                    .map(|(key, value)| (key.clone(), normalize_upstream_payload(value)))
                    .collect::<Map<String, Value>>(),
            )
        }
        _ => value.clone(),
    }
}

pub(super) fn twitter_datetime_to_rfc3339(value: Option<&str>) -> Option<String> {
    let raw = value?.trim();
    if raw.is_empty() {
        return None;
    }

    if looks_like_iso_datetime(raw) {
        return Some(normalize_rfc3339_offset(raw));
    }

    let parts = raw.split_whitespace().collect::<Vec<_>>();
    if parts.len() != 6 {
        return Some(raw.to_owned());
    }

    let month = match parts[1] {
        "Jan" => "01",
        "Feb" => "02",
        "Mar" => "03",
        "Apr" => "04",
        "May" => "05",
        "Jun" => "06",
        "Jul" => "07",
        "Aug" => "08",
        "Sep" => "09",
        "Oct" => "10",
        "Nov" => "11",
        "Dec" => "12",
        _ => return Some(raw.to_owned()),
    };
    let offset = match normalize_compact_offset(parts[4]) {
        Some(offset) => offset,
        None => return Some(raw.to_owned()),
    };

    let day = format!("{:0>2}", parts[2]);
    Some(format!("{}-{month}-{day}T{}{}", parts[5], parts[3], offset))
}

fn normalize_rfc3339_offset(raw: &str) -> String {
    if raw.ends_with('Z') {
        return raw.to_owned();
    }

    let Some(compact_offset) = raw.get(raw.len().saturating_sub(5)..) else {
        return raw.to_owned();
    };
    let Some(offset) = normalize_compact_offset(compact_offset) else {
        return raw.to_owned();
    };

    format!("{}{}", &raw[..raw.len() - 5], offset)
}

fn looks_like_iso_datetime(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    bytes.len() >= 10
        && bytes[..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
}

fn normalize_compact_offset(offset: &str) -> Option<String> {
    let bytes = offset.as_bytes();
    if bytes.len() != 5 {
        return None;
    }

    let sign = bytes[0];
    if sign != b'+' && sign != b'-' {
        return None;
    }
    if !bytes[1..].iter().all(u8::is_ascii_digit) {
        return None;
    }

    Some(format!("{}:{}", &offset[..3], &offset[3..5]))
}

fn ensure_graphql_success(url: &str, value: &Value) -> Result<(), AppError> {
    if let Some(message) = first_non_ignorable_graphql_error_message(value) {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!("twitter request to {url} failed: {message}"),
        });
    }

    if value.get("data").is_none() {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!("twitter request to {url} did not return `data`"),
        });
    }

    Ok(())
}

fn first_non_ignorable_graphql_error_message(value: &Value) -> Option<&str> {
    graphql_errors(value)?
        .iter()
        .filter(|error| !is_ignorable_graphql_error(error))
        .find_map(graphql_error_message)
}

fn graphql_errors(value: &Value) -> Option<&[Value]> {
    value
        .get("errors")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
}

fn graphql_error_message(error: &Value) -> Option<&str> {
    error.get("message").and_then(Value::as_str)
}

fn is_ignorable_graphql_error(error: &Value) -> bool {
    graphql_error_message(error).is_some_and(|message| message.contains("LoadShed"))
        && error
            .get("path")
            .and_then(Value::as_array)
            .and_then(|path| path.last())
            .and_then(Value::as_str)
            == Some("quick_promote_eligibility")
}

fn header_map_from_headers(headers: &BTreeMap<String, String>) -> Result<HeaderMap, AppError> {
    let mut header_map = HeaderMap::new();

    for (name, value) in headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| AppError::InvalidRequestConfig(format!("invalid header name `{name}`")))?;
        let header_value = if header_name == AUTHORIZATION {
            HeaderValue::from_str(value).map_err(|_| {
                AppError::InvalidRequestConfig(format!("invalid header value for `{name}`"))
            })?
        } else {
            HeaderValue::from_str(value).map_err(|_| {
                AppError::InvalidRequestConfig(format!("invalid header value for `{name}`"))
            })?
        };
        header_map.insert(header_name, header_value);
    }

    Ok(header_map)
}

fn is_retryable(error: &reqwest::Error) -> bool {
    error.is_connect() || error.is_timeout()
}

fn cookie_has_name(cookie: &str, name: &str) -> bool {
    cookie
        .split(';')
        .filter_map(|part| part.trim().split_once('='))
        .any(|(key, _)| key.trim() == name)
}
