use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::{Client, Method, Url};

use crate::error::AppError;
use crate::platforms::internal::base64::encode_base64;
use crate::platforms::internal::random::PseudoRandom;

use super::cubic::Cubic;
use super::hash::sha256_digest;
use super::html::{MigrationForm, TwitterHomePage, extract_key_byte_indices};
use super::interpolate::interpolate;
use super::rotation::convert_rotation_to_matrix;

const ADDITIONAL_RANDOM_NUMBER: u8 = 3;
const DEFAULT_KEYWORD: &str = "obfiowerehiring";
const TOTAL_TIME: f64 = 4096.0;
const TWITTER_TRANSACTION_EPOCH_SECONDS: u64 = 1_682_924_400;

#[derive(Debug, Clone)]
pub(crate) struct ClientTransactionState {
    key_bytes: Vec<u8>,
    animation_key: String,
}

impl ClientTransactionState {
    pub(crate) fn generate_transaction_id(&self, method: &str, path: &str) -> String {
        let relative_time = current_relative_time_seconds();
        let random_byte = (PseudoRandom::from_system().next_u32() & 0xff) as u8;
        self.generate_transaction_id_with(method, path, relative_time, random_byte)
    }

    fn generate_transaction_id_with(
        &self,
        method: &str,
        path: &str,
        relative_time: u32,
        random_byte: u8,
    ) -> String {
        let data = format!(
            "{method}!{path}!{relative_time}{DEFAULT_KEYWORD}{}",
            self.animation_key
        );
        let digest = sha256_digest(data.as_bytes());
        let time_bytes = relative_time.to_le_bytes();

        let payload_len = 1 + self.key_bytes.len() + time_bytes.len() + 16 + 1;
        let mut payload = Vec::with_capacity(payload_len);
        payload.push(random_byte);

        for byte in self
            .key_bytes
            .iter()
            .copied()
            .chain(time_bytes)
            .chain(digest[..16].iter().copied())
            .chain([ADDITIONAL_RANDOM_NUMBER])
        {
            payload.push(byte ^ random_byte);
        }

        let mut encoded = encode_base64(&payload);
        encoded.retain(|ch| ch != '=');
        encoded
    }
}

pub(crate) async fn build_client_transaction_state(
    client: &Client,
    request_headers: &BTreeMap<String, String>,
    web_base_url: &str,
) -> Result<ClientTransactionState, AppError> {
    let home_page = fetch_home_page(client, request_headers, web_base_url).await?;
    let key = home_page.site_verification_key()?;
    let key_bytes = decode_standard_base64(&key)?;
    let frame_paths = home_page.frame_paths()?;
    let ondemand_url = home_page.resolve_ondemand_file_url()?;
    let chunk = fetch_ondemand_chunk(client, request_headers, &ondemand_url, web_base_url).await?;
    let (row_index_key, key_byte_indices) = extract_key_byte_indices(&chunk)?;
    let animation_key =
        build_animation_key(&key_bytes, row_index_key, &key_byte_indices, &frame_paths)?;

    Ok(ClientTransactionState {
        key_bytes,
        animation_key,
    })
}

async fn fetch_home_page(
    client: &Client,
    request_headers: &BTreeMap<String, String>,
    web_base_url: &str,
) -> Result<TwitterHomePage, AppError> {
    let mut current_url =
        Url::parse(&format!("{}/", web_base_url.trim_end_matches('/'))).map_err(|error| {
            AppError::InvalidRequestConfig(format!("invalid twitter web base url: {error}"))
        })?;
    let mut page = TwitterHomePage::new(
        send_text_request(
            client,
            Method::GET,
            current_url.clone(),
            &build_document_headers(request_headers),
            None,
        )
        .await?,
    );

    if let Some(redirect_url) = page.migration_redirect_url() {
        current_url = resolve_url(&current_url, &redirect_url)?;
        page = TwitterHomePage::new(
            send_text_request(
                client,
                Method::GET,
                current_url.clone(),
                &build_document_headers(request_headers),
                None,
            )
            .await?,
        );
    }

    if let Some(form) = page.migration_form() {
        let action_url = resolve_url(&current_url, &form.action)?;
        let method = Method::from_bytes(form.method.as_bytes()).unwrap_or(Method::POST);
        page = TwitterHomePage::new(
            send_text_request(
                client,
                method,
                action_url,
                &build_form_headers(request_headers, &current_url),
                Some(&form),
            )
            .await?,
        );
    }

    Ok(page)
}

async fn fetch_ondemand_chunk(
    client: &Client,
    request_headers: &BTreeMap<String, String>,
    ondemand_url: &str,
    web_base_url: &str,
) -> Result<String, AppError> {
    let url = Url::parse(ondemand_url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("invalid twitter ondemand url: {error}"))
    })?;

    send_text_request(
        client,
        Method::GET,
        url,
        &build_asset_headers(request_headers, web_base_url),
        None,
    )
    .await
}

async fn send_text_request(
    client: &Client,
    method: Method,
    url: Url,
    headers: &BTreeMap<String, String>,
    form: Option<&MigrationForm>,
) -> Result<String, AppError> {
    let url_text = url.to_string();
    let mut request = client.request(method, url);

    for (name, value) in headers {
        request = request.header(name, value);
    }

    if let Some(form) = form {
        request = request.form(&form.fields);
    }

    let response = request.send().await?;
    let status = response.status();
    let body = response.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(AppError::UpstreamResponse {
            status: Some(status),
            message: format!("request to {url_text} returned `{body}`"),
        });
    }

    Ok(body)
}

fn build_document_headers(request_headers: &BTreeMap<String, String>) -> BTreeMap<String, String> {
    let mut headers = request_headers.clone();
    strip_unwanted_headers(&mut headers);
    headers.insert(
        "Accept".into(),
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".into(),
    );
    headers.insert("Cache-Control".into(), "no-cache".into());
    headers.insert("Pragma".into(), "no-cache".into());
    headers.insert("Sec-Fetch-Dest".into(), "document".into());
    headers.insert("Sec-Fetch-Mode".into(), "navigate".into());
    headers.insert("Sec-Fetch-Site".into(), "none".into());
    headers.insert("Sec-Fetch-User".into(), "?1".into());
    headers.insert("Upgrade-Insecure-Requests".into(), "1".into());
    headers
}

fn build_form_headers(
    request_headers: &BTreeMap<String, String>,
    current_url: &Url,
) -> BTreeMap<String, String> {
    let mut headers = build_document_headers(request_headers);
    headers.insert("Referer".into(), current_url.to_string());
    if let Some(origin) = current_url.domain() {
        headers.insert(
            "Origin".into(),
            format!("{}://{}", current_url.scheme(), origin),
        );
    }
    headers
}

fn build_asset_headers(
    request_headers: &BTreeMap<String, String>,
    web_base_url: &str,
) -> BTreeMap<String, String> {
    let mut headers = request_headers.clone();
    strip_unwanted_headers(&mut headers);
    headers.insert("Accept".into(), "*/*".into());
    headers.insert("Sec-Fetch-Dest".into(), "script".into());
    headers.insert("Sec-Fetch-Mode".into(), "no-cors".into());
    headers.insert("Sec-Fetch-Site".into(), "cross-site".into());
    headers.insert(
        "Referer".into(),
        format!("{}/", web_base_url.trim_end_matches('/')),
    );
    headers
}

fn strip_unwanted_headers(headers: &mut BTreeMap<String, String>) {
    let keys = headers.keys().cloned().collect::<Vec<_>>();
    for key in keys {
        if [
            "authorization",
            "content-type",
            "origin",
            "x-client-transaction-id",
            "x-csrf-token",
            "x-guest-token",
            "x-twitter-auth-type",
        ]
        .iter()
        .any(|header| key.eq_ignore_ascii_case(header))
        {
            headers.remove(&key);
        }
    }
}

fn resolve_url(base_url: &Url, target: &str) -> Result<Url, AppError> {
    base_url.join(target).map_err(|error| {
        AppError::InvalidRequestConfig(format!("invalid twitter migration url `{target}`: {error}"))
    })
}

fn build_animation_key(
    key_bytes: &[u8],
    row_index_key: usize,
    key_byte_indices: &[usize],
    frame_paths: &[String],
) -> Result<String, AppError> {
    if frame_paths.len() < 4 {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!(
                "twitter homepage did not expose enough animation frames, got {}",
                frame_paths.len()
            ),
        });
    }

    let frame_index = key_bytes
        .get(5)
        .copied()
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: "twitter site verification key was shorter than expected".into(),
        })? as usize
        % 4;

    let row_index =
        key_bytes
            .get(row_index_key)
            .copied()
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter row index key `{row_index_key}` was out of bounds"),
            })? as usize
            % 16;

    let mut frame_time = 1u64;
    for index in key_byte_indices {
        let value = key_bytes
            .get(*index)
            .copied()
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter key byte index `{index}` was out of bounds"),
            })?;
        frame_time = frame_time.saturating_mul((value % 16) as u64);
    }

    frame_time = (js_round(frame_time as f64 / 10.0) * 10.0) as u64;
    let target_time = frame_time as f64 / TOTAL_TIME;
    let rows = parse_frame_rows(&frame_paths[frame_index]);
    let frame_row = rows
        .get(row_index)
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("twitter animation frame row `{row_index}` was missing"),
        })?;

    animate(frame_row, target_time)
}

fn parse_frame_rows(d_attr: &str) -> Vec<Vec<u8>> {
    let value = d_attr.get(9..).unwrap_or_default();
    value
        .split('C')
        .map(|segment| {
            let mut cleaned = String::with_capacity(segment.len());
            for ch in segment.chars() {
                if ch.is_ascii_digit() {
                    cleaned.push(ch);
                } else {
                    cleaned.push(' ');
                }
            }

            cleaned
                .split_whitespace()
                .filter_map(|part| part.parse::<u16>().ok())
                .map(|part| part.min(u8::MAX as u16) as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn animate(frame_row: &[u8], target_time: f64) -> Result<String, AppError> {
    if frame_row.len() < 11 {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!(
                "twitter animation frame row was too short to build a signature: {}",
                frame_row.len()
            ),
        });
    }

    let from_color = [
        frame_row[0] as f64,
        frame_row[1] as f64,
        frame_row[2] as f64,
        1.0,
    ];
    let to_color = [
        frame_row[3] as f64,
        frame_row[4] as f64,
        frame_row[5] as f64,
        1.0,
    ];
    let to_rotation = [solve(frame_row[6], 60.0, 360.0, true)];

    let curves = frame_row[7..]
        .iter()
        .enumerate()
        .map(|(index, value)| solve(*value, odd_offset(index), 1.0, false))
        .collect::<Vec<_>>();
    let cubic = Cubic::new(curves);
    let interpolation = cubic.get_value(target_time);
    let color = interpolate(&from_color, &to_color, interpolation)?
        .into_iter()
        .map(|value| if value > 0.0 { value } else { 0.0 })
        .collect::<Vec<_>>();
    let rotation = interpolate(&[0.0], &to_rotation, interpolation)?;
    let matrix = convert_rotation_to_matrix(rotation[0]);

    let mut parts = color[..3]
        .iter()
        .map(|value| format!("{:x}", js_round(*value) as i64))
        .collect::<Vec<_>>();

    for value in matrix {
        let mut rounded = js_round(value * 100.0) / 100.0;
        if rounded < 0.0 {
            rounded = -rounded;
        }

        let hex_value = float_to_hex(rounded).to_lowercase();
        if hex_value.is_empty() {
            parts.push("0".into());
        } else if hex_value.starts_with('.') {
            parts.push(format!("0{hex_value}"));
        } else {
            parts.push(hex_value);
        }
    }

    parts.push("0".into());
    parts.push("0".into());

    Ok(parts.join("").replace(['.', '-'], ""))
}

fn solve(value: u8, min: f64, max: f64, round_whole: bool) -> f64 {
    let result = (value as f64 * (max - min)) / 255.0 + min;
    if round_whole {
        js_round(result)
    } else {
        js_round(result * 100.0) / 100.0
    }
}

fn odd_offset(index: usize) -> f64 {
    if index % 2 == 1 { -1.0 } else { 0.0 }
}

fn js_round(value: f64) -> f64 {
    (value + 0.5).floor()
}

fn float_to_hex(mut value: f64) -> String {
    let mut result = Vec::new();
    let mut quotient = value.floor();
    let mut fraction = value - quotient;

    while quotient > 0.0 {
        quotient = (value / 16.0).floor();
        let remainder = (value - quotient * 16.0).floor() as u8;
        result.insert(0, hex_digit(remainder));
        value = quotient;
    }

    if fraction == 0.0 {
        return result.into_iter().collect();
    }

    result.push('.');
    while fraction > 0.0 {
        fraction *= 16.0;
        let integer = fraction.floor() as u8;
        fraction -= integer as f64;
        result.push(hex_digit(integer));
    }

    result.into_iter().collect()
}

fn hex_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        _ => (b'A' + (value - 10)) as char,
    }
}

fn decode_standard_base64(value: &str) -> Result<Vec<u8>, AppError> {
    let mut sextets = Vec::new();
    for ch in value.chars() {
        match ch {
            'A'..='Z' => sextets.push(ch as u8 - b'A'),
            'a'..='z' => sextets.push(ch as u8 - b'a' + 26),
            '0'..='9' => sextets.push(ch as u8 - b'0' + 52),
            '+' => sextets.push(62),
            '/' => sextets.push(63),
            '=' => sextets.push(64),
            _ if ch.is_whitespace() => {}
            _ => {
                return Err(AppError::InvalidRequestConfig(format!(
                    "invalid twitter base64 input: {value}"
                )));
            }
        }
    }

    if sextets.len() % 4 != 0 {
        return Err(AppError::InvalidRequestConfig(format!(
            "invalid twitter base64 input length: {value}"
        )));
    }

    let mut output = Vec::new();
    for chunk in sextets.chunks_exact(4) {
        let a = chunk[0];
        let b = chunk[1];
        let c = chunk[2];
        let d = chunk[3];

        if a == 64 || b == 64 {
            return Err(AppError::InvalidRequestConfig(format!(
                "invalid twitter base64 padding: {value}"
            )));
        }

        let block = ((a as u32) << 18)
            | ((b as u32) << 12)
            | (((if c == 64 { 0 } else { c }) as u32) << 6)
            | ((if d == 64 { 0 } else { d }) as u32);

        output.push(((block >> 16) & 0xff) as u8);
        if c != 64 {
            output.push(((block >> 8) & 0xff) as u8);
        }
        if d != 64 {
            output.push((block & 0xff) as u8);
        }
    }

    Ok(output)
}

fn current_relative_time_seconds() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    now.saturating_sub(TWITTER_TRANSACTION_EPOCH_SECONDS) as u32
}
