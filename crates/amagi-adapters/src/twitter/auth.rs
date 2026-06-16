pub(super) const TWITTER_WEB_BEARER_TOKEN: &str = "AAAAAAAAAAAAAAAAAAAAANRILgAAAAAAnNwIzUejRCOuH5E6I8xnZz4puTs%3D1Zv7ttfk8LF81IUq16cHjhLTvJu4FA33AGWWjCpTnA";
pub(super) const TWITTER_DEFAULT_LANGUAGE: &str = "en";

pub(super) fn extract_cookie_value<'a>(cookie: &'a str, key: &str) -> Option<&'a str> {
    cookie.split(';').find_map(|segment| {
        let mut parts = segment.trim().splitn(2, '=');
        let name = parts.next()?.trim();
        let value = parts.next()?.trim();
        (name == key && !value.is_empty()).then_some(value)
    })
}

pub(super) fn extract_csrf_token(cookie: &str) -> Option<String> {
    extract_cookie_value(cookie, "ct0").map(str::to_owned)
}

pub(super) fn extract_twid_user_id(cookie: &str) -> Option<String> {
    let raw = extract_cookie_value(cookie, "twid")?
        .trim()
        .trim_matches('"');
    let normalized = raw.replace("%3D", "=").replace("%3d", "=");
    let user_id = normalized.strip_prefix("u=")?;
    (!user_id.is_empty() && user_id.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| user_id.to_owned())
}
