use crate::error::AppError;
use crate::platforms::internal::md5::md5_hex;

use super::super::OrderedJson;
use super::super::utils::{build_url as build_signed_url, extract_api_path};
use super::types::{
    CookieJar, XiaohongshuBrowserState, XiaohongshuMnsv2Input, XiaohongshuXsCommonPayload,
    XiaohongshuXsEnvelope,
};

const DEFAULT_XS_VERSION: &str = "4.2.6";
const DEFAULT_PLATFORM: &str = "Windows";
const DEFAULT_APP_ID: &str = "xhs-pc-web";
const DEFAULT_APP_VERSION: &str = "4.86.0";

pub(super) fn build_mnsv2_input(
    uri: &str,
    params: Option<&OrderedJson>,
    payload: Option<&OrderedJson>,
) -> Result<XiaohongshuMnsv2Input, AppError> {
    let full_input = match payload {
        Some(payload) => format!("{uri}{}", payload.to_json_string()?),
        None => build_signed_url(uri, params)?,
    };
    let sign_path = extract_api_path(&full_input).to_owned();

    Ok(XiaohongshuMnsv2Input {
        md5_path: md5_hex(sign_path.as_bytes()),
        md5_full: md5_hex(full_input.as_bytes()),
        sign_path,
        full_input,
    })
}

pub(super) fn build_xs_envelope(
    x3: &str,
    _payload: Option<&OrderedJson>,
    _browser_state: Option<&XiaohongshuBrowserState>,
) -> XiaohongshuXsEnvelope {
    XiaohongshuXsEnvelope {
        x0: DEFAULT_XS_VERSION.to_owned(),
        x1: DEFAULT_APP_ID.to_owned(),
        x2: DEFAULT_PLATFORM.to_owned(),
        x3: x3.to_owned(),
        x4: String::new(),
    }
}

pub(super) fn build_xs_common_payload(
    cookies: &CookieJar,
    b1_value: &str,
    x9_value: i32,
) -> Result<XiaohongshuXsCommonPayload, AppError> {
    let a1 = cookies
        .get("a1")
        .ok_or_else(|| AppError::InvalidRequestConfig("missing xiaohongshu cookie `a1`".into()))?;

    Ok(XiaohongshuXsCommonPayload {
        s0: 5,
        s1: String::new(),
        x0: "1".to_owned(),
        x1: DEFAULT_XS_VERSION.to_owned(),
        x2: DEFAULT_PLATFORM.to_owned(),
        x3: DEFAULT_APP_ID.to_owned(),
        x4: DEFAULT_APP_VERSION.to_owned(),
        x5: a1.to_owned(),
        x6: String::new(),
        x7: String::new(),
        x8: b1_value.to_owned(),
        x9: x9_value,
        x10: 0,
        x11: "normal".to_owned(),
    })
}
