use crate::error::AppError;
use crate::platforms::internal::random::{PseudoRandom, now_unix_ms};
use serde_json::{from_str as parse_json, to_string as to_json_string};

use super::super::config::Config;
use super::super::ordered_json::OrderedJson;
use super::super::session::{XiaohongshuSession, XiaohongshuSignState};
use super::super::utils::{
    build_url as build_signed_url, generate_b3_trace_id, generate_search_id, generate_xray_trace_id,
};
use super::codec::{crc32_js_int, decode_custom_base64, encode_custom_base64};
use super::fingerprint::build_b1_value;
use super::payload::{build_mnsv2_input, build_xs_common_payload, build_xs_envelope};
use super::types::{
    CookieJar, XiaohongshuBrowserState, XiaohongshuHeaders, XiaohongshuMethod,
    XiaohongshuMnsv2Input, XiaohongshuXsCommonPayload, XiaohongshuXsEnvelope,
};
use super::x3::{build_x3_payload, decode_x3_bytes, sign_x3_payload};

const DEFAULT_APP_ID: &str = "xhs-pc-web";

/// Xiaohongshu pure-protocol signer.
#[derive(Debug, Clone)]
pub struct XiaohongshuSigner {
    config: Config,
    random: PseudoRandom,
    now_ms_override: Option<u64>,
    browser_state: Option<XiaohongshuBrowserState>,
}

impl Default for XiaohongshuSigner {
    fn default() -> Self {
        Self::new()
    }
}

impl XiaohongshuSigner {
    /// Create a signer with runtime randomness.
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            random: PseudoRandom::from_system(),
            now_ms_override: None,
            browser_state: None,
        }
    }

    /// Create a deterministic signer for tests and reproducible output.
    pub fn with_seed(seed: u64) -> Self {
        Self {
            config: Config::default(),
            random: PseudoRandom::new(seed),
            now_ms_override: None,
            browser_state: None,
        }
    }

    /// Create a deterministic signer pinned to a fixed current time in milliseconds.
    pub fn with_seed_and_time(seed: u64, now_ms: u64) -> Self {
        Self {
            config: Config::default(),
            random: PseudoRandom::new(seed),
            now_ms_override: Some(now_ms),
            browser_state: None,
        }
    }

    /// Create a signer with a legacy browser state placeholder bound in advance.
    pub fn with_browser_state(browser_state: XiaohongshuBrowserState) -> Self {
        let mut signer = Self::new();
        signer.browser_state = Some(browser_state);
        signer
    }

    /// Replace the currently bound legacy browser state placeholder.
    pub fn set_browser_state(&mut self, browser_state: XiaohongshuBrowserState) {
        self.browser_state = Some(browser_state);
    }

    /// Return the currently bound legacy browser state placeholder, if present.
    pub fn browser_state(&self) -> Option<&XiaohongshuBrowserState> {
        self.browser_state.as_ref()
    }

    /// Sign a Xiaohongshu request and return the `x-s` header value.
    pub fn sign_xs(
        &mut self,
        method: XiaohongshuMethod,
        uri: &str,
        a1_value: &str,
        xsec_appid: Option<&str>,
        payload: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<String, AppError> {
        let (params, body) = split_signing_input(method, payload);
        let sign_input = self.build_mnsv2_input(uri, params, body)?;
        let timestamp_ms = self.get_x_t(timestamp_secs);
        let sign_state = self.resolve_sign_state(session, uri, &sign_input, timestamp_ms);
        let x3 =
            self.sign_x3_from_input(&sign_input, a1_value, xsec_appid, sign_state, timestamp_ms)?;

        self.sign_xs_from_x3(&x3, body, self.browser_state.as_ref())
    }

    /// Sign a Xiaohongshu `GET` request.
    pub fn sign_xs_get(
        &mut self,
        uri: &str,
        a1_value: &str,
        xsec_appid: Option<&str>,
        params: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<String, AppError> {
        self.sign_xs(
            XiaohongshuMethod::Get,
            uri,
            a1_value,
            xsec_appid,
            params,
            timestamp_secs,
            session,
        )
    }

    /// Sign a Xiaohongshu `POST` request.
    pub fn sign_xs_post(
        &mut self,
        uri: &str,
        a1_value: &str,
        xsec_appid: Option<&str>,
        payload: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<String, AppError> {
        self.sign_xs(
            XiaohongshuMethod::Post,
            uri,
            a1_value,
            xsec_appid,
            payload,
            timestamp_secs,
            session,
        )
    }

    /// Generate the `x-s-common` header value.
    pub fn sign_xs_common(&mut self, cookies: &CookieJar) -> Result<String, AppError> {
        let payload = self.build_xs_common_payload(cookies)?;
        let json = to_json_string(&payload)?;
        Ok(encode_custom_base64(json.as_bytes()))
    }

    /// Build deterministic signing input material.
    pub fn build_mnsv2_input(
        &self,
        uri: &str,
        params: Option<&OrderedJson>,
        payload: Option<&OrderedJson>,
    ) -> Result<XiaohongshuMnsv2Input, AppError> {
        build_mnsv2_input(uri, params, payload)
    }

    /// Encode an `x-s` value from a known `x3` payload.
    pub fn sign_xs_from_x3(
        &self,
        x3: &str,
        payload: Option<&OrderedJson>,
        browser_state: Option<&XiaohongshuBrowserState>,
    ) -> Result<String, AppError> {
        let envelope = build_xs_envelope(x3, payload, browser_state);
        let json = to_json_string(&envelope)?;
        Ok(format!("XYS_{}", encode_custom_base64(json.as_bytes())))
    }

    /// Build the inner `x3` signature using the pure protocol flow.
    pub fn sign_x3_with_state(
        &mut self,
        sign_input: &XiaohongshuMnsv2Input,
        a1_value: &str,
        xsec_appid: Option<&str>,
        _browser_state: &XiaohongshuBrowserState,
        timestamp_secs: Option<f64>,
    ) -> Result<String, AppError> {
        let timestamp_ms = self.get_x_t(timestamp_secs);
        let sign_state = self.resolve_random_sign_state(sign_input, timestamp_ms);
        self.sign_x3_from_input(sign_input, a1_value, xsec_appid, sign_state, timestamp_ms)
    }

    /// Build the structured payload later encoded into `x-s-common`.
    pub fn build_xs_common_payload(
        &mut self,
        cookies: &CookieJar,
    ) -> Result<XiaohongshuXsCommonPayload, AppError> {
        let now_ms = self.current_now_ms();
        let b1 = build_b1_value(&mut self.random, now_ms)?;
        let x9 = crc32_js_int(&b1);
        build_xs_common_payload(cookies, &b1, x9)
    }

    /// Encode `x-s-common` from a legacy browser-derived state placeholder.
    pub fn sign_xs_common_with_state(
        &mut self,
        cookies: &CookieJar,
        _browser_state: &XiaohongshuBrowserState,
    ) -> Result<String, AppError> {
        self.sign_xs_common(cookies)
    }

    /// Generate fully-signed request headers using an explicit legacy state placeholder.
    pub fn sign_headers_with_state(
        &mut self,
        method: XiaohongshuMethod,
        uri: &str,
        cookies: &CookieJar,
        xsec_appid: Option<&str>,
        params: Option<&OrderedJson>,
        payload: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        _browser_state: &XiaohongshuBrowserState,
    ) -> Result<XiaohongshuHeaders, AppError> {
        self.sign_headers(
            method,
            uri,
            cookies,
            xsec_appid,
            params,
            payload,
            timestamp_secs,
            None,
        )
    }

    /// Generate fully-signed request headers.
    pub fn sign_headers(
        &mut self,
        method: XiaohongshuMethod,
        uri: &str,
        cookies: &CookieJar,
        xsec_appid: Option<&str>,
        params: Option<&OrderedJson>,
        payload: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<XiaohongshuHeaders, AppError> {
        let a1_value = cookies.get("a1").ok_or_else(|| {
            AppError::InvalidRequestConfig("missing xiaohongshu cookie `a1`".into())
        })?;
        let body = match method {
            XiaohongshuMethod::Get => None,
            XiaohongshuMethod::Post => payload,
        };
        let sign_input = self.build_mnsv2_input(uri, params, body)?;
        let timestamp_ms = self.get_x_t(timestamp_secs);
        let sign_state = self.resolve_sign_state(session, uri, &sign_input, timestamp_ms);
        let x3 =
            self.sign_x3_from_input(&sign_input, a1_value, xsec_appid, sign_state, timestamp_ms)?;

        Ok(XiaohongshuHeaders {
            x_s: self.sign_xs_from_x3(&x3, body, self.browser_state.as_ref())?,
            x_s_common: self.sign_xs_common(cookies)?,
            x_t: timestamp_ms.to_string(),
            x_b3_traceid: self.get_b3_trace_id(),
            x_xray_traceid: self.get_xray_trace_id(Some(timestamp_ms), None),
        })
    }

    /// Generate signed headers for a `GET` request.
    pub fn sign_headers_get(
        &mut self,
        uri: &str,
        cookies: &CookieJar,
        xsec_appid: Option<&str>,
        params: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<XiaohongshuHeaders, AppError> {
        self.sign_headers(
            XiaohongshuMethod::Get,
            uri,
            cookies,
            xsec_appid,
            params,
            None,
            timestamp_secs,
            session,
        )
    }

    /// Generate signed headers for a `POST` request.
    pub fn sign_headers_post(
        &mut self,
        uri: &str,
        cookies: &CookieJar,
        xsec_appid: Option<&str>,
        payload: Option<&OrderedJson>,
        timestamp_secs: Option<f64>,
        session: Option<&mut XiaohongshuSession>,
    ) -> Result<XiaohongshuHeaders, AppError> {
        self.sign_headers(
            XiaohongshuMethod::Post,
            uri,
            cookies,
            xsec_appid,
            None,
            payload,
            timestamp_secs,
            session,
        )
    }

    /// Decode a full `x-s` envelope.
    pub fn decode_xs(&self, xs_signature: &str) -> Result<XiaohongshuXsEnvelope, AppError> {
        let payload = xs_signature.strip_prefix("XYS_").ok_or_else(|| {
            AppError::InvalidRequestConfig("invalid xiaohongshu x-s prefix".into())
        })?;
        let decoded = decode_custom_base64(payload)?;
        let json = String::from_utf8(decoded).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid xiaohongshu x-s utf-8 payload: {error}"
            ))
        })?;
        Ok(parse_json(&json)?)
    }

    /// Decode the inner `x3` payload back into raw bytes.
    pub fn decode_x3(&self, x3_signature: &str) -> Result<Vec<u8>, AppError> {
        decode_x3_bytes(x3_signature)
    }

    /// Build a URL using Xiaohongshu-specific query escaping.
    pub fn build_url(
        &self,
        base_url: &str,
        params: Option<&OrderedJson>,
    ) -> Result<String, AppError> {
        build_signed_url(base_url, params)
    }

    /// Serialize an ordered payload into a JSON body string.
    pub fn build_json_body(&self, payload: &OrderedJson) -> Result<String, AppError> {
        payload.to_json_string()
    }

    /// Generate `x-b3-traceid`.
    pub fn get_b3_trace_id(&mut self) -> String {
        generate_b3_trace_id(&mut self.random, &self.config)
    }

    /// Generate `x-xray-traceid`.
    pub fn get_xray_trace_id(&mut self, timestamp_ms: Option<u64>, seq: Option<u32>) -> String {
        generate_xray_trace_id(&mut self.random, &self.config, timestamp_ms, seq)
    }

    /// Generate `x-t` in milliseconds.
    pub fn get_x_t(&self, timestamp_secs: Option<f64>) -> u64 {
        match timestamp_secs {
            Some(value) => (value * 1000.0).floor() as u64,
            None => self.current_now_ms(),
        }
    }

    /// Generate a Xiaohongshu search id using the source implementation layout.
    pub fn generate_search_id(&mut self) -> String {
        generate_search_id(self.current_now_ms(), self.random.next_u32())
    }

    fn current_now_ms(&self) -> u64 {
        self.now_ms_override.unwrap_or_else(now_unix_ms)
    }

    fn sign_x3_from_input(
        &mut self,
        sign_input: &XiaohongshuMnsv2Input,
        a1_value: &str,
        xsec_appid: Option<&str>,
        sign_state: XiaohongshuSignState,
        timestamp_ms: u64,
    ) -> Result<String, AppError> {
        let payload = build_x3_payload(
            sign_input,
            a1_value,
            xsec_appid
                .filter(|value| !value.is_empty())
                .unwrap_or(DEFAULT_APP_ID),
            sign_state,
            timestamp_ms,
            self.random.next_u32(),
        )?;

        sign_x3_payload(&payload)
    }

    fn resolve_sign_state(
        &mut self,
        session: Option<&mut XiaohongshuSession>,
        uri: &str,
        sign_input: &XiaohongshuMnsv2Input,
        timestamp_ms: u64,
    ) -> XiaohongshuSignState {
        match session {
            Some(session) => session.current_state(uri),
            None => self.resolve_random_sign_state(sign_input, timestamp_ms),
        }
    }

    fn resolve_random_sign_state(
        &mut self,
        sign_input: &XiaohongshuMnsv2Input,
        timestamp_ms: u64,
    ) -> XiaohongshuSignState {
        let time_offset_secs = random_range(&mut self.random, 10, 50) as u64;
        XiaohongshuSignState {
            page_load_timestamp: timestamp_ms.saturating_sub(time_offset_secs * 1000),
            sequence_value: random_range(&mut self.random, 15, 50),
            window_props_length: random_range(&mut self.random, 1000, 1200),
            uri_length: sign_input.full_input.len() as u32,
        }
    }
}

fn random_range(random: &mut PseudoRandom, min: u32, max: u32) -> u32 {
    if max <= min {
        min
    } else {
        min + random.next_mod(max - min + 1)
    }
}

fn split_signing_input(
    method: XiaohongshuMethod,
    payload: Option<&OrderedJson>,
) -> (Option<&OrderedJson>, Option<&OrderedJson>) {
    match method {
        XiaohongshuMethod::Get => (payload, None),
        XiaohongshuMethod::Post => (None, payload),
    }
}
