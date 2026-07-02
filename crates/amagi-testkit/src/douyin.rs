//! Douyin test fixtures, fetcher constructors, and assertion helpers.

use amagi::douyin::{
    DEFAULT_USER_AGENT, DEFAULT_WINDOW_ENV, DouyinApiUrls, DouyinFetcher, DouyinParsedWork,
    DouyinSearchResult, DouyinSearchType, build_signed_url_with_a_bogus,
    build_signed_url_with_x_bogus, clean_user_agent_for_signing, create_douyin_api_urls,
    generate_a_bogus, generate_a_bogus_with_options, generate_ms_token,
    generate_ms_token_from_entropy, generate_verify_fp, generate_verify_fp_with_entropy,
    generate_x_bogus, generate_x_bogus_at,
};
use serde_json::Value;

use crate::env::{TestResult, private_env};

const TEST_AWEME_ID: &str = "test_aweme_id";
const TEST_COMMENT_ID: &str = "test_comment_id";
const TEST_SEC_UID: &str = "test_sec_uid";
const TEST_MUSIC_ID: &str = "test_music_id";
const TEST_ROOM_ID: &str = "test_room_id";
const TEST_WEB_RID: &str = "test_web_rid";
const TEST_QUERY: &str = "test query";
const TEST_URL: &str =
    "https://www.douyin.com/aweme/v1/web/aweme/detail/?aweme_id=test_aweme_id&aid=6383";

/// Assert that every public Douyin URL builder can produce a plausible URL.
///
/// # Errors
///
/// Returns an error when any URL builder rejects the fixed test inputs.
pub fn assert_public_url_builders_cover_all_endpoints() -> TestResult {
    let urls = DouyinApiUrls::with_verify_fp(Some(DEFAULT_USER_AGENT), "verify_test");
    assert_eq!(urls.verify_fp(), "verify_test");

    let default_urls = create_douyin_api_urls(Some(DEFAULT_USER_AGENT));
    assert!(default_urls.verify_fp().starts_with("verify_"));

    let cases = [
        ("work_detail", urls.work_detail(TEST_AWEME_ID)?),
        ("comments", urls.comments(TEST_AWEME_ID, Some(0), Some(2))?),
        (
            "comment_replies",
            urls.comment_replies(TEST_AWEME_ID, TEST_COMMENT_ID, Some(0), Some(2))?,
        ),
        ("slides_info", urls.slides_info(TEST_AWEME_ID)?),
        ("emoji_list", urls.emoji_list()),
        (
            "user_video_list",
            urls.user_video_list(TEST_SEC_UID, Some("0"), Some(2))?,
        ),
        (
            "user_favorite_list",
            urls.user_favorite_list(TEST_SEC_UID, Some("0"), Some(2))?,
        ),
        (
            "user_recommend_list",
            urls.user_recommend_list(TEST_SEC_UID, Some("0"), Some(2))?,
        ),
        ("user_profile", urls.user_profile(TEST_SEC_UID)?),
        ("suggest_words", urls.suggest_words(TEST_QUERY)?),
        (
            "search_general",
            urls.search(TEST_QUERY, DouyinSearchType::General, Some(2), None)?,
        ),
        (
            "search_user",
            urls.search(TEST_QUERY, DouyinSearchType::User, Some(2), Some("rid"))?,
        ),
        (
            "search_video",
            urls.search(TEST_QUERY, DouyinSearchType::Video, Some(2), Some("rid"))?,
        ),
        ("dynamic_emoji_list", urls.dynamic_emoji_list()?),
        ("music_info", urls.music_info(TEST_MUSIC_ID)?),
        (
            "live_room_info",
            urls.live_room_info(TEST_ROOM_ID, TEST_WEB_RID)?,
        ),
        ("login_qrcode", urls.login_qrcode(Some("verify_test"))?),
        (
            "danmaku_list",
            urls.danmaku_list(TEST_AWEME_ID, Some(0), Some(1_000), 1_000)?,
        ),
    ];

    for (name, url) in cases {
        assert!(
            url.starts_with("https://"),
            "{name} should build an absolute HTTPS URL: {url}"
        );
        assert!(
            url.contains("douyin.com") || url.contains("amemv.com"),
            "{name} should target a Douyin endpoint: {url}"
        );
    }

    Ok(())
}

/// Assert that public Douyin signing helpers return non-empty deterministic values.
///
/// # Errors
///
/// Returns an error when a signing helper cannot derive a signature from the
/// fixed test input.
pub fn assert_sign_helpers_generate_nonempty_values() -> TestResult {
    let deterministic_ms_token = generate_ms_token_from_entropy(4, b"abcd");
    assert_eq!(deterministic_ms_token.len(), 4);
    assert!(
        deterministic_ms_token
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric())
    );
    assert_eq!(generate_ms_token(8).len(), 8);

    let verify_fp = generate_verify_fp_with_entropy(1_700_000_000_000, b"abcdef");
    assert!(verify_fp.starts_with("verify_"));
    assert!(generate_verify_fp().starts_with("verify_"));

    let edge_user_agent = format!("{DEFAULT_USER_AGENT} Edg/125.0.0.0");
    assert!(!clean_user_agent_for_signing(&edge_user_agent).contains("Edg/"));
    assert!(!DEFAULT_WINDOW_ENV.is_empty());

    let a_bogus = generate_a_bogus(TEST_URL, Some(DEFAULT_USER_AGENT))?;
    assert!(!a_bogus.is_empty());

    let deterministic_a_bogus = generate_a_bogus_with_options(
        TEST_URL,
        DEFAULT_USER_AGENT,
        1_700_000_000_000,
        1_700_000_000_010,
        [1, 2, 3],
        DEFAULT_WINDOW_ENV,
    )?;
    assert!(!deterministic_a_bogus.is_empty());

    let x_bogus = generate_x_bogus(TEST_URL, Some(DEFAULT_USER_AGENT))?;
    assert!(!x_bogus.is_empty());

    let deterministic_x_bogus = generate_x_bogus_at(TEST_URL, DEFAULT_USER_AGENT, 1_700_000_000)?;
    assert!(!deterministic_x_bogus.is_empty());

    assert!(
        build_signed_url_with_a_bogus(TEST_URL, Some(DEFAULT_USER_AGENT))?.contains("a_bogus=")
    );
    assert!(
        build_signed_url_with_x_bogus(TEST_URL, Some(DEFAULT_USER_AGENT))?.contains("X-Bogus=")
    );

    Ok(())
}

/// Build a Douyin fetcher only when `AMAGI_DOUYIN_COOKIE` is set.
///
/// Returns `Ok(None)` and prints a skip message when the cookie is unavailable.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env_if_cookie(
    manifest_dir: impl AsRef<std::path::Path>,
) -> TestResult<Option<DouyinFetcher>> {
    let client = crate::client::client_from_env(manifest_dir)?;
    let cookie = client
        .options()
        .cookies
        .douyin
        .as_deref()
        .unwrap_or_default();
    if cookie.trim().is_empty() {
        eprintln!("skipped: AMAGI_DOUYIN_COOKIE is not set");
        return Ok(None);
    }

    Ok(Some(client.douyin_fetcher()))
}

/// Minimal Douyin identifiers derived from private test data or search results.
#[derive(Debug, Clone)]
pub struct DouyinSeedSample {
    /// Aweme id used as the primary work-detail seed.
    pub aweme_id: String,
    /// Optional author `sec_uid` derived from the same sample.
    pub sec_uid: Option<String>,
    /// Optional music id derived from the same sample.
    pub music_id: Option<String>,
    /// Work duration in milliseconds.
    pub duration_ms: u64,
}

/// Derive a reusable Douyin sample from private env vars or live search data.
///
/// # Errors
///
/// Returns an error when upstream requests fail or no usable sample can be
/// derived.
pub async fn seed_sample(fetcher: &DouyinFetcher) -> TestResult<DouyinSeedSample> {
    if let Some(aweme_id) = private_env("AMAGI_PRIVATE_DOUYIN_AWEME_ID") {
        let work = fetcher.parse_work(&aweme_id).await?;
        if let Some(seed) = seed_from_work(&work, aweme_id) {
            return Ok(seed);
        }
    }

    let query = private_env("AMAGI_PRIVATE_DOUYIN_QUERY").unwrap_or_else(|| "test".to_owned());
    let search = fetcher
        .search_content(&query, Some(DouyinSearchType::Video), Some(10), None)
        .await?;
    if let Some(seed) = seed_from_search(&search) {
        if let Ok(work) = fetcher.parse_work(&seed.aweme_id).await {
            if let Some(enriched_seed) = seed_from_work(&work, seed.aweme_id.clone()) {
                return Ok(enriched_seed);
            }
        }
        return Ok(seed);
    }

    Err("could not derive a Douyin seed aweme from env or search results".into())
}

/// Build a Douyin seed sample from parsed work detail.
pub fn seed_from_work(
    work: &DouyinParsedWork,
    fallback_aweme_id: String,
) -> Option<DouyinSeedSample> {
    let detail = work.aweme_detail.as_ref()?;
    Some(DouyinSeedSample {
        aweme_id: detail
            .aweme_id
            .clone()
            .filter(|value| !value.is_empty())
            .unwrap_or(fallback_aweme_id),
        sec_uid: private_env("AMAGI_PRIVATE_DOUYIN_SEC_UID").or_else(|| {
            detail
                .author
                .as_ref()
                .and_then(|author| author.sec_uid.clone())
        }),
        music_id: private_env("AMAGI_PRIVATE_DOUYIN_MUSIC_ID").or_else(|| {
            detail.music.as_ref().and_then(|music| {
                music
                    .id_str
                    .clone()
                    .or_else(|| music.mid.clone())
                    .or_else(|| music.id.map(|id| id.to_string()))
            })
        }),
        duration_ms: detail.duration.unwrap_or_default().max(0) as u64,
    })
}

/// Build a Douyin seed sample from a search response.
pub fn seed_from_search(search: &DouyinSearchResult) -> Option<DouyinSeedSample> {
    search.data.iter().find_map(|item| {
        let aweme = item.aweme_info.as_ref()?;
        let aweme_id = string_at(aweme, &["aweme_id", "group_id"])?;
        Some(DouyinSeedSample {
            aweme_id,
            sec_uid: private_env("AMAGI_PRIVATE_DOUYIN_SEC_UID")
                .or_else(|| nested_string_at(aweme, "author", &["sec_uid"])),
            music_id: private_env("AMAGI_PRIVATE_DOUYIN_MUSIC_ID")
                .or_else(|| nested_string_at(aweme, "music", &["id_str", "mid", "id"])),
            duration_ms: number_at(aweme, &["duration"]).unwrap_or_default(),
        })
    })
}

/// Assert that parsed work detail returned the expected aweme id.
pub fn assert_work_detail(label: &str, work: &DouyinParsedWork, expected_aweme_id: &str) {
    assert_status_ok(label, work.meta.status_code);
    let detail = work
        .aweme_detail
        .as_ref()
        .unwrap_or_else(|| panic!("{label} should include aweme_detail"));
    assert_eq!(
        detail.aweme_id.as_deref(),
        Some(expected_aweme_id),
        "{label} should return the requested aweme"
    );
}

/// Assert that a Douyin search response is successful and contains useful data.
pub fn assert_search_result(label: &str, search: &DouyinSearchResult) {
    assert_status_ok(label, search.meta.status_code);
    assert!(
        !search.data.is_empty()
            || !search.user_list.is_empty()
            || !search.meta.upstream_payload.is_null(),
        "{label} should return result items or an upstream payload"
    );
}

/// Assert that an optional Douyin `status_code` is either absent or success.
pub fn assert_status_ok(label: &str, status_code: Option<i64>) {
    if let Some(status_code) = status_code {
        assert_eq!(status_code, 0, "{label} returned non-zero status_code");
    }
}

/// Assert that a raw-payload condition is true for the named test case.
pub fn assert_raw_payload(label: &str, condition: bool) {
    assert!(condition, "{label} should return a non-empty payload");
}

/// Return the first non-empty string or numeric value found at one of `keys`.
pub fn string_at(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        value.get(*key).and_then(|value| match value {
            Value::String(value) if !value.trim().is_empty() => Some(value.clone()),
            Value::Number(value) => Some(value.to_string()),
            _ => None,
        })
    })
}

/// Return the first non-empty string or numeric value under `parent`.
pub fn nested_string_at(value: &Value, parent: &str, keys: &[&str]) -> Option<String> {
    value.get(parent).and_then(|value| string_at(value, keys))
}

/// Return the first unsigned integer value found at one of `keys`.
pub fn number_at(value: &Value, keys: &[&str]) -> Option<u64> {
    keys.iter().find_map(|key| {
        value.get(*key).and_then(|value| {
            value
                .as_u64()
                .or_else(|| value.as_i64().and_then(|value| u64::try_from(value).ok()))
                .or_else(|| value.as_str().and_then(|value| value.parse::<u64>().ok()))
        })
    })
}
