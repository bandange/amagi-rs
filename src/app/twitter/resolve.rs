use reqwest::Url;

use crate::error::AppError;

const TWITTER_HOSTS: &[&str] = &[
    "x.com",
    "www.x.com",
    "twitter.com",
    "www.twitter.com",
    "mobile.twitter.com",
];

const RESERVED_USER_PATHS: &[&str] = &[
    "home",
    "explore",
    "search",
    "i",
    "notifications",
    "messages",
    "compose",
    "settings",
    "tos",
    "privacy",
    "intent",
    "share",
    "hashtag",
];

pub(super) fn resolve_user_reference(input: &str) -> Result<String, AppError> {
    parse_user_reference(input)
}

pub(super) fn resolve_tweet_reference(input: &str) -> Result<String, AppError> {
    let candidate = normalized_reference_input(input)?;
    if is_numeric_identifier(candidate) {
        return Ok(candidate.to_owned());
    }

    let url = parse_twitter_url(candidate)?;
    let segments = twitter_path_segments(&url)?;

    if let Some(status_index) = segments
        .iter()
        .position(|segment| matches!(*segment, "status" | "statuses"))
    {
        let tweet_id = segments
            .get(status_index + 1)
            .copied()
            .filter(|value| is_numeric_identifier(value))
            .ok_or_else(|| {
                AppError::InvalidRequestConfig(format!(
                    "twitter tweet reference `{input}` does not contain a valid tweet id"
                ))
            })?;
        return Ok(tweet_id.to_owned());
    }

    Err(AppError::InvalidRequestConfig(format!(
        "unsupported twitter tweet reference `{input}`"
    )))
}

pub(super) fn resolve_space_reference(input: &str) -> Result<String, AppError> {
    let candidate = normalized_reference_input(input)?;
    if !looks_like_url(candidate) {
        return Ok(candidate.to_owned());
    }

    let url = parse_twitter_url(candidate)?;
    let segments = twitter_path_segments(&url)?;
    if segments.len() >= 3 && segments[0] == "i" && segments[1] == "spaces" {
        return Ok(segments[2].to_owned());
    }

    Err(AppError::InvalidRequestConfig(format!(
        "unsupported twitter space reference `{input}`"
    )))
}

pub(super) fn parse_user_reference(input: &str) -> Result<String, AppError> {
    let candidate = normalized_reference_input(input)?;
    if let Some(stripped) = candidate.strip_prefix('@') {
        return normalize_screen_name(stripped);
    }

    if !looks_like_url(candidate) {
        if is_numeric_identifier(candidate) {
            return Err(AppError::InvalidRequestConfig(format!(
                "twitter user reference `{input}` must be a screen name or profile url, numeric user ids are not supported"
            )));
        }

        return normalize_screen_name(candidate);
    }

    let url = parse_twitter_url(candidate)?;
    let segments = twitter_path_segments(&url)?;

    if segments.len() >= 3 && segments[0] == "i" && segments[1] == "user" {
        return Err(AppError::InvalidRequestConfig(format!(
            "twitter user reference `{input}` must contain a screen name, numeric user ids are not supported"
        )));
    }

    let first_segment = segments.first().copied().ok_or_else(|| {
        AppError::InvalidRequestConfig(format!(
            "twitter user reference `{input}` does not contain a user path"
        ))
    })?;
    if RESERVED_USER_PATHS.contains(&first_segment) {
        return Err(AppError::InvalidRequestConfig(format!(
            "unsupported twitter user reference `{input}`"
        )));
    }

    if is_numeric_identifier(first_segment) {
        return Err(AppError::InvalidRequestConfig(format!(
            "twitter user reference `{input}` must contain a screen name, numeric user ids are not supported"
        )));
    }

    normalize_screen_name(first_segment)
}

fn normalized_reference_input(input: &str) -> Result<&str, AppError> {
    let candidate = input.trim();
    if candidate.is_empty() {
        return Err(AppError::InvalidRequestConfig(
            "twitter reference input cannot be empty".into(),
        ));
    }

    Ok(candidate)
}

fn parse_twitter_url(input: &str) -> Result<Url, AppError> {
    let candidate = if input.contains("://") {
        input.to_owned()
    } else {
        format!("https://{input}")
    };

    let url = Url::parse(&candidate).map_err(|error| {
        AppError::InvalidRequestConfig(format!("invalid twitter reference url `{input}`: {error}"))
    })?;
    let host = url.host_str().unwrap_or_default().to_ascii_lowercase();
    if !TWITTER_HOSTS.contains(&host.as_str()) {
        return Err(AppError::InvalidRequestConfig(format!(
            "unsupported twitter host in `{input}`"
        )));
    }

    Ok(url)
}

fn twitter_path_segments(url: &Url) -> Result<Vec<&str>, AppError> {
    let segments = url
        .path_segments()
        .map(|items| {
            items
                .filter(|segment| !segment.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if segments.is_empty() {
        return Err(AppError::InvalidRequestConfig(format!(
            "twitter url `{url}` does not contain path segments"
        )));
    }

    Ok(segments)
}

fn looks_like_url(value: &str) -> bool {
    value.contains("://")
        || value.starts_with("x.com/")
        || value.starts_with("www.x.com/")
        || value.starts_with("twitter.com/")
        || value.starts_with("www.twitter.com/")
        || value.starts_with("mobile.twitter.com/")
}

fn is_numeric_identifier(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn normalize_screen_name(value: &str) -> Result<String, AppError> {
    let normalized = value.trim().trim_start_matches('@').trim_end_matches('/');
    if normalized.is_empty() {
        return Err(AppError::InvalidRequestConfig(
            "twitter screen name cannot be empty".into(),
        ));
    }

    if normalized.contains('/') {
        return Err(AppError::InvalidRequestConfig(format!(
            "invalid twitter screen name `{value}`"
        )));
    }

    Ok(normalized.to_owned())
}
