//! Twitter/X test fixtures and live-room contract helpers.

use amagi::{
    AppError,
    twitter::{
        TwitterFetcher, TwitterLiveBroadcastDetail, TwitterLiveRoomInfo, TwitterLiveRoomStream,
        TwitterLiveVideoBroadcast,
    },
};
use serde_json::{Value, json};

use crate::env::TestResult;

const TEST_USER_ID: &str = "test_user_id";
const TEST_SCREEN_NAME: &str = "test_screen_name";
const TEST_BROADCAST_ID: &str = "test_broadcast_id";
const TEST_MEDIA_KEY: &str = "28_test_media_id";
const TEST_TWEET_ID: &str = "test_tweet_id";

/// Return a representative live-room info payload for public surface tests.
pub fn sample_live_room_info() -> TwitterLiveRoomInfo {
    TwitterLiveRoomInfo {
        user_id: TEST_USER_ID.to_owned(),
        screen_name: Some(TEST_SCREEN_NAME.to_owned()),
        is_live: true,
        refresh_delay_secs: Some(30),
        live_video: Some(TwitterLiveVideoBroadcast {
            broadcast_id: TEST_BROADCAST_ID.to_owned(),
            state: Some("RUNNING".to_owned()),
            media_key: Some(TEST_MEDIA_KEY.to_owned()),
            media_id: Some("test_media_id".to_owned()),
            tweet_id: Some(TEST_TWEET_ID.to_owned()),
            ..TwitterLiveVideoBroadcast::default()
        }),
        upstream_payload: json!({
            "users": {
                TEST_USER_ID: {
                    "spaces": {}
                }
            }
        }),
    }
}

/// Return a representative live-room stream payload for public surface tests.
pub fn sample_live_room_stream() -> TwitterLiveRoomStream {
    TwitterLiveRoomStream {
        broadcast_id: Some(TEST_BROADCAST_ID.to_owned()),
        media_key: TEST_MEDIA_KEY.to_owned(),
        tweet_id: Some(TEST_TWEET_ID.to_owned()),
        hls_url: "https://example.test/master.m3u8?type=replay".to_owned(),
        stream_type: Some("HLS".to_owned()),
        is_replay: true,
        broadcast: Some(TwitterLiveBroadcastDetail {
            broadcast_id: TEST_BROADCAST_ID.to_owned(),
            media_key: Some(TEST_MEDIA_KEY.to_owned()),
            state: Some("ENDED".to_owned()),
            ..TwitterLiveBroadcastDetail::default()
        }),
        ..TwitterLiveRoomStream::default()
    }
}

/// Assert that Twitter live-room info APIs are present in the public catalog.
pub fn assert_live_room_info_public_surface() {
    let _ = sample_live_room_info();

    let by_screen_name = amagi::find_operation(amagi::Platform::Twitter, "liveRoomInfo").unwrap();
    assert_eq!(by_screen_name.fetcher_name, "fetchLiveRoomInfo");
    assert_eq!(by_screen_name.route, "/user/{screen_name}/live-room-info");

    let by_user_id =
        amagi::find_operation(amagi::Platform::Twitter, "liveRoomInfoByUserId").unwrap();
    assert_eq!(by_user_id.fetcher_name, "fetchLiveRoomInfoByUserId");
    assert_eq!(by_user_id.route, "/user-id/{user_id}/live-room-info");
}

/// Assert that Twitter live-room stream APIs are present in the public catalog.
pub fn assert_live_room_stream_public_surface() {
    let _ = sample_live_room_stream();

    let by_broadcast = amagi::find_operation(amagi::Platform::Twitter, "liveRoomStream").unwrap();
    assert_eq!(by_broadcast.fetcher_name, "fetchLiveRoomStream");
    assert_eq!(by_broadcast.route, "/live-room/{broadcast_id}/stream");

    let by_media_key =
        amagi::find_operation(amagi::Platform::Twitter, "liveRoomStreamByMediaKey").unwrap();
    assert_eq!(by_media_key.fetcher_name, "fetchLiveRoomStreamByMediaKey");
    assert_eq!(by_media_key.route, "/live-media/{media_key}/stream");

    let by_tweet =
        amagi::find_operation(amagi::Platform::Twitter, "liveRoomStreamByTweetId").unwrap();
    assert_eq!(by_tweet.fetcher_name, "fetchLiveRoomStreamByTweetId");
    assert_eq!(by_tweet.route, "/tweet/{tweet_id}/live-room-stream");
}

/// Build an authenticated Twitter/X fetcher from local test environment files.
///
/// # Errors
///
/// Returns an error when the shared test client cannot be constructed.
#[cfg(feature = "client")]
pub fn fetcher_from_env(manifest_dir: impl AsRef<std::path::Path>) -> TestResult<TwitterFetcher> {
    Ok(crate::client::client_from_env(manifest_dir)?.twitter_fetcher())
}

/// Build an unauthenticated Twitter/X fetcher with default client options.
#[cfg(feature = "client")]
pub fn unauthenticated_fetcher() -> TwitterFetcher {
    crate::client::unauthenticated_client().twitter_fetcher()
}

/// Assert that live-room info rejects non-numeric user ids before upstream I/O.
///
/// # Errors
///
/// Returns an error only when the assertion body itself fails to complete.
pub async fn assert_live_room_info_rejects_non_numeric_user_id(
    fetcher: &TwitterFetcher,
    value: &str,
) -> TestResult {
    let error = fetcher
        .fetch_live_room_info_by_user_id(value)
        .await
        .expect_err("non-numeric user ids must be rejected before any upstream request");

    assert!(matches!(error, AppError::InvalidRequestConfig(_)));
    Ok(())
}

/// Assert that stream lookup rejects invalid live media keys before upstream I/O.
///
/// # Errors
///
/// Returns an error only when the assertion body itself fails to complete.
pub async fn assert_live_room_stream_rejects_invalid_media_key(
    fetcher: &TwitterFetcher,
    value: &str,
) -> TestResult {
    let error = fetcher
        .fetch_live_room_stream_by_media_key(value)
        .await
        .expect_err("non-broadcast media keys must be rejected before upstream requests");

    assert!(matches!(error, AppError::InvalidRequestConfig(_)));
    Ok(())
}

/// Assert the public live-room info contract for one expected user id.
pub fn assert_live_room_info_contract(info: &TwitterLiveRoomInfo, expected_user_id: &str) {
    assert_eq!(info.user_id, expected_user_id);

    if info.is_live {
        let live_video = info
            .live_video
            .as_ref()
            .expect("live room info should include broadcast metadata when is_live is true");
        assert!(!live_video.broadcast_id.is_empty());
        assert!(
            live_video
                .media_key
                .as_deref()
                .is_some_and(|value| !value.is_empty()),
            "live room info should include media_key for running broadcasts"
        );
        assert!(
            live_video
                .tweet_id
                .as_deref()
                .is_some_and(|value| !value.is_empty()),
            "live room info should include tweet_id for running broadcasts"
        );
    }
}

/// Assert the public live-room stream contract for a media key and replay state.
pub fn assert_live_room_stream_contract(
    stream: &TwitterLiveRoomStream,
    expected_media_key: &str,
    expected_replay: bool,
) {
    assert_eq!(stream.media_key, expected_media_key);
    assert_eq!(stream.stream_type.as_deref(), Some("HLS"));
    assert_eq!(stream.is_replay, expected_replay);
    assert!(stream.hls_url.starts_with("https://"));
    assert!(stream.hls_url.contains(".m3u8"));
}

/// Return whether a string contains a marker that looks like a secret value.
pub fn contains_secret_marker(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    [
        "auth_token",
        "ct0=",
        "authorization",
        "x-csrf-token",
        "cookie",
    ]
    .into_iter()
    .any(|marker| lower.contains(marker))
}

/// Return JSON key paths whose key names contain any of the provided needles.
pub fn matching_key_paths(value: &Value, needles: &[&str]) -> Vec<String> {
    let mut paths = Vec::new();
    collect_matching_key_paths(value, "$", needles, &mut paths);
    paths
}

fn collect_matching_key_paths(
    value: &Value,
    path: &str,
    needles: &[&str],
    paths: &mut Vec<String>,
) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                let next_path = format!("{path}.{key}");
                let lower_key = key.to_ascii_lowercase();
                if needles.iter().any(|needle| lower_key.contains(needle)) {
                    paths.push(next_path.clone());
                }
                collect_matching_key_paths(value, &next_path, needles, paths);
            }
        }
        Value::Array(items) => {
            for (index, value) in items.iter().enumerate() {
                collect_matching_key_paths(value, &format!("{path}[{index}]"), needles, paths);
            }
        }
        _ => {}
    }
}
