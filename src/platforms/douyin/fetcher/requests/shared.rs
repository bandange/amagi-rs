use std::fmt::Write;

use crate::error::AppError;

const DEFAULT_BROWSER_VERSION: &str = "125.0.0.0";

pub(super) fn extract_browser_version(user_agent: Option<&str>) -> String {
    let Some(user_agent) = user_agent else {
        return DEFAULT_BROWSER_VERSION.into();
    };

    for marker in ["Chrome/", "Edg/"] {
        if let Some(version) = user_agent
            .split(marker)
            .nth(1)
            .and_then(|tail| tail.split_whitespace().next())
            .filter(|value| !value.is_empty())
        {
            return version.to_owned();
        }
    }

    DEFAULT_BROWSER_VERSION.into()
}

pub(super) fn join_base(base: &str, path: &str) -> String {
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        path.trim_start_matches('/')
    )
}

pub(super) fn build_url(
    base: &str,
    params: Vec<(&'static str, String)>,
) -> Result<String, AppError> {
    reqwest::Url::parse(base).map_err(|error| {
        AppError::InvalidRequestConfig(format!("invalid Douyin endpoint `{base}`: {error}"))
    })?;

    if params.is_empty() {
        return Ok(base.to_owned());
    }

    Ok(format!("{base}?{}", build_query_string(&params)))
}

fn build_query_string(params: &[(&str, String)]) -> String {
    params
        .iter()
        .map(|(key, value)| {
            format!(
                "{}={}",
                encode_uri_component(key),
                encode_uri_component(value)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

pub(super) fn encode_uri_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());

    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => encoded.push(*byte as char),
            _ => {
                let _ = write!(&mut encoded, "%{:02X}", byte);
            }
        }
    }

    encoded
}
