use serde_json::{Map, Value, json};

use amagi_core::AppError;

use super::TwitterFetcher;
use super::transport::{
    bool_at_path, normalize_optional_string, normalize_upstream_payload, string_at_path,
    u64_at_path, value_at_path,
};
use crate::twitter::{TwitterLiveBroadcastDetail, TwitterLiveRoomStream};

impl TwitterFetcher {
    /// Fetch a Twitter/X live-room playback stream by broadcast id.
    #[doc(alias = "fetchLiveRoomStream")]
    pub async fn fetch_live_room_stream(
        &self,
        broadcast_id: &str,
    ) -> Result<TwitterLiveRoomStream, AppError> {
        let broadcast_id = normalize_broadcast_id(broadcast_id)?;
        let broadcast = self.fetch_live_broadcast_detail(broadcast_id).await?;
        let media_key = broadcast
            .media_key
            .clone()
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter broadcast `{broadcast_id}` did not include `media_key`"),
            })?;

        self.fetch_live_room_stream_inner(
            &media_key,
            Some(broadcast),
            None,
            Some(&format!(
                "{}/i/broadcasts/{broadcast_id}",
                self.web_base_url.trim_end_matches('/')
            )),
        )
        .await
    }

    /// Fetch a Twitter/X live-room playback stream directly by broadcast media key.
    #[doc(alias = "fetchLiveRoomStreamByMediaKey")]
    pub async fn fetch_live_room_stream_by_media_key(
        &self,
        media_key: &str,
    ) -> Result<TwitterLiveRoomStream, AppError> {
        let media_key = normalize_media_key(media_key)?;
        self.fetch_live_room_stream_inner(media_key, None, None, None)
            .await
    }

    /// Fetch a Twitter/X live-room playback stream by tweet id.
    #[doc(alias = "fetchLiveRoomStreamByTweetId")]
    pub async fn fetch_live_room_stream_by_tweet_id(
        &self,
        tweet_id: &str,
    ) -> Result<TwitterLiveRoomStream, AppError> {
        let tweet_id = normalize_tweet_id(tweet_id)?;
        let tweet = self.fetch_tweet_detail(tweet_id).await?;
        let tweet_ref =
            find_tweet_broadcast_reference(&tweet.upstream_payload).ok_or_else(|| {
                AppError::UpstreamResponse {
                    status: None,
                    message: format!(
                        "twitter tweet `{tweet_id}` does not contain a resolvable broadcast stream"
                    ),
                }
            })?;

        if let Some(broadcast_id) = tweet_ref.broadcast_id.as_deref() {
            let mut stream = self.fetch_live_room_stream(broadcast_id).await?;
            stream.tweet_id = stream.tweet_id.or_else(|| Some(tweet_id.to_owned()));
            return Ok(stream);
        }

        let media_key =
            tweet_ref
                .media_key
                .as_deref()
                .ok_or_else(|| AppError::UpstreamResponse {
                    status: None,
                    message: format!(
                        "twitter tweet `{tweet_id}` does not contain a broadcast media key"
                    ),
                })?;
        let mut stream = self.fetch_live_room_stream_by_media_key(media_key).await?;
        stream.tweet_id = Some(tweet_id.to_owned());
        Ok(stream)
    }

    async fn fetch_live_broadcast_detail(
        &self,
        broadcast_id: &str,
    ) -> Result<TwitterLiveBroadcastDetail, AppError> {
        let value = self
            .fetch_web_api_value(
                &self.api_urls.broadcast_show(broadcast_id)?,
                Some(&format!(
                    "{}/i/broadcasts/{broadcast_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;
        parse_live_broadcast_detail(&value, broadcast_id)
    }

    async fn fetch_live_room_stream_inner(
        &self,
        media_key: &str,
        broadcast: Option<TwitterLiveBroadcastDetail>,
        tweet_id: Option<String>,
        referer: Option<&str>,
    ) -> Result<TwitterLiveRoomStream, AppError> {
        let media_key = normalize_media_key(media_key)?;
        let value = self
            .fetch_web_api_value(&self.api_urls.live_video_stream_status(media_key)?, referer)
            .await?;
        parse_live_room_stream(&value, media_key, broadcast, tweet_id)
    }
}

fn parse_live_room_stream(
    value: &Value,
    media_key: &str,
    broadcast: Option<TwitterLiveBroadcastDetail>,
    tweet_id: Option<String>,
) -> Result<TwitterLiveRoomStream, AppError> {
    let hls_url = string_at_path(value, &["source", "location"])
        .or_else(|| string_at_path(value, &["source", "noRedirectPlaybackUrl"]))
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("twitter stream status for `{media_key}` did not include an HLS URL"),
        })?;
    let no_redirect_playback_url = string_at_path(value, &["source", "noRedirectPlaybackUrl"]);
    let source_status = string_at_path(value, &["source", "status"]);
    let stream_type = string_at_path(value, &["source", "streamType"]);
    let share_url = string_at_path(value, &["shareUrl"]);
    let broadcast_id = broadcast
        .as_ref()
        .map(|broadcast| broadcast.broadcast_id.clone())
        .or_else(|| share_url.as_deref().and_then(broadcast_id_from_share_url));
    let tweet_id = tweet_id.or_else(|| {
        broadcast
            .as_ref()
            .and_then(|broadcast| broadcast.tweet_id.clone())
    });
    let is_replay = is_replay_stream(
        &hls_url,
        broadcast.as_ref().and_then(|value| value.state.as_deref()),
    );

    Ok(TwitterLiveRoomStream {
        broadcast_id,
        media_key: media_key.to_owned(),
        tweet_id,
        hls_url,
        no_redirect_playback_url,
        source_status,
        stream_type,
        is_replay,
        session_id: string_at_path(value, &["sessionId"]),
        share_url,
        upstream_payload: json!({
            "stream_status": redact_sensitive_payload(&normalize_upstream_payload(value)),
            "broadcast": broadcast.as_ref().map(|value| value.upstream_payload.clone()),
        }),
        broadcast,
    })
}

fn parse_live_broadcast_detail(
    value: &Value,
    broadcast_id: &str,
) -> Result<TwitterLiveBroadcastDetail, AppError> {
    let broadcast =
        find_broadcast_payload(value, broadcast_id).ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("twitter broadcast `{broadcast_id}` was not found"),
        })?;
    let parsed_id = string_at_path(broadcast, &["broadcast_id"])
        .or_else(|| string_at_path(broadcast, &["id"]))
        .unwrap_or_else(|| broadcast_id.to_owned());
    let media_key = string_at_path(broadcast, &["media_key"]);

    Ok(TwitterLiveBroadcastDetail {
        broadcast_id: parsed_id,
        media_id: string_at_path(broadcast, &["media_id"])
            .or_else(|| media_key.as_deref().and_then(media_id_from_media_key)),
        media_key,
        state: string_at_path(broadcast, &["state"]),
        title: string_at_path(broadcast, &["status"])
            .or_else(|| string_at_path(broadcast, &["title"])),
        tweet_id: string_at_path(broadcast, &["tweet_id"]),
        periscope_user_id: string_at_path(broadcast, &["user_id"]),
        twitter_user_id: string_at_path(broadcast, &["twitter_user_id"])
            .or_else(|| string_at_path(broadcast, &["twitter_id"])),
        twitter_username: string_at_path(broadcast, &["twitter_username"])
            .or_else(|| string_at_path(broadcast, &["username"]))
            .or_else(|| {
                string_at_path(
                    broadcast,
                    &["user_results", "result", "core", "screen_name"],
                )
            })
            .or_else(|| string_at_path(broadcast, &["periscope_user", "username"])),
        display_name: string_at_path(broadcast, &["user_display_name"])
            .or_else(|| string_at_path(broadcast, &["user_results", "result", "core", "name"]))
            .or_else(|| string_at_path(broadcast, &["periscope_user", "display_name"])),
        available_for_replay: bool_at_path(broadcast, &["available_for_replay"]),
        started_at: string_at_path(broadcast, &["start_ms"])
            .or_else(|| string_at_path(broadcast, &["start"]))
            .or_else(|| string_at_path(broadcast, &["created_at_ms"])),
        ended_at: string_at_path(broadcast, &["end_ms"])
            .or_else(|| string_at_path(broadcast, &["end"])),
        ping_at: string_at_path(broadcast, &["ping_ms"])
            .or_else(|| string_at_path(broadcast, &["ping"])),
        width: u64_at_path(broadcast, &["width"]),
        height: u64_at_path(broadcast, &["height"]),
        total_watching: u64_at_path(broadcast, &["total_watching"])
            .or_else(|| u64_at_path(broadcast, &["n_watching"])),
        total_watched: u64_at_path(broadcast, &["total_watched"])
            .or_else(|| u64_at_path(broadcast, &["n_total_watching"])),
        broadcast_source: string_at_path(broadcast, &["broadcast_source"]),
        is_high_latency: bool_at_path(broadcast, &["is_high_latency"]),
        upstream_payload: redact_sensitive_payload(&normalize_upstream_payload(broadcast)),
    })
}

fn find_broadcast_payload<'a>(value: &'a Value, broadcast_id: &str) -> Option<&'a Value> {
    value_at_path(value, &["broadcasts", broadcast_id])
        .or_else(|| value_at_path(value, &["data", "broadcast_by_rest_id"]))
        .filter(|broadcast| broadcast_matches_id(broadcast, broadcast_id))
        .or_else(|| find_matching_broadcast_payload(value, broadcast_id))
}

fn find_matching_broadcast_payload<'a>(value: &'a Value, broadcast_id: &str) -> Option<&'a Value> {
    match value {
        Value::Object(map) => {
            if broadcast_matches_id(value, broadcast_id) && value.get("media_key").is_some() {
                return Some(value);
            }

            map.values()
                .find_map(|value| find_matching_broadcast_payload(value, broadcast_id))
        }
        Value::Array(items) => items
            .iter()
            .find_map(|value| find_matching_broadcast_payload(value, broadcast_id)),
        _ => None,
    }
}

fn broadcast_matches_id(value: &Value, broadcast_id: &str) -> bool {
    string_at_path(value, &["broadcast_id"])
        .or_else(|| string_at_path(value, &["id"]))
        .is_some_and(|value| value == broadcast_id)
}

#[derive(Debug, Clone, Default)]
struct TweetBroadcastReference {
    broadcast_id: Option<String>,
    media_key: Option<String>,
}

fn find_tweet_broadcast_reference(value: &Value) -> Option<TweetBroadcastReference> {
    let mut reference = TweetBroadcastReference::default();
    collect_tweet_broadcast_reference(value, &mut reference);

    if reference.broadcast_id.is_some() || reference.media_key.is_some() {
        Some(reference)
    } else {
        None
    }
}

fn collect_tweet_broadcast_reference(value: &Value, reference: &mut TweetBroadcastReference) {
    match value {
        Value::Object(map) => {
            collect_card_binding_value(map, reference);

            for (key, value) in map {
                let normalized_key = key.to_ascii_lowercase();
                if reference.broadcast_id.is_none()
                    && matches!(normalized_key.as_str(), "broadcast_id" | "broadcastid")
                {
                    reference.broadcast_id = string_value(value);
                }
                if reference.media_key.is_none()
                    && matches!(normalized_key.as_str(), "broadcast_media_key" | "media_key")
                {
                    reference.media_key =
                        string_value(value).filter(|value| is_live_media_key(value));
                }
                collect_tweet_broadcast_reference(value, reference);
            }
        }
        Value::Array(items) => {
            for value in items {
                collect_tweet_broadcast_reference(value, reference);
            }
        }
        _ => {}
    }
}

fn collect_card_binding_value(map: &Map<String, Value>, reference: &mut TweetBroadcastReference) {
    let Some(key) = map.get("key").and_then(Value::as_str) else {
        return;
    };
    let Some(value) = map.get("value").and_then(binding_string_value) else {
        return;
    };

    match key {
        "broadcast_id" if reference.broadcast_id.is_none() => {
            reference.broadcast_id = Some(value);
        }
        "broadcast_media_key" | "media_key"
            if reference.media_key.is_none() && is_live_media_key(&value) =>
        {
            reference.media_key = Some(value);
        }
        _ => {}
    }
}

fn binding_string_value(value: &Value) -> Option<String> {
    string_at_path(value, &["string_value"])
        .or_else(|| string_at_path(value, &["scribe_key"]))
        .or_else(|| string_value(value))
}

fn string_value(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => normalize_optional_string(value.clone()),
        Value::Object(_) => string_at_path(value, &["string_value"]),
        _ => None,
    }
}

fn normalize_broadcast_id(value: &str) -> Result<&str, AppError> {
    let trimmed = value.trim();
    if !trimmed.is_empty()
        && trimmed
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Ok(trimmed);
    }

    Err(AppError::InvalidRequestConfig(format!(
        "twitter broadcast id `{value}` is invalid"
    )))
}

fn normalize_media_key(value: &str) -> Result<&str, AppError> {
    let trimmed = value.trim();
    if is_live_media_key(trimmed) {
        return Ok(trimmed);
    }

    Err(AppError::InvalidRequestConfig(format!(
        "twitter live media key `{value}` must use the `28_<media_id>` form"
    )))
}

fn normalize_tweet_id(value: &str) -> Result<&str, AppError> {
    let trimmed = value.trim();
    if !trimmed.is_empty() && trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(trimmed);
    }

    Err(AppError::InvalidRequestConfig(format!(
        "twitter tweet id `{value}` must be numeric"
    )))
}

fn is_live_media_key(value: &str) -> bool {
    value
        .strip_prefix("28_")
        .is_some_and(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
}

fn media_id_from_media_key(media_key: &str) -> Option<String> {
    media_key
        .strip_prefix("28_")
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .map(str::to_owned)
}

fn broadcast_id_from_share_url(value: &str) -> Option<String> {
    value
        .trim_end_matches('/')
        .rsplit('/')
        .next()
        .and_then(|value| normalize_broadcast_id(value).ok())
        .map(str::to_owned)
}

fn is_replay_stream(hls_url: &str, broadcast_state: Option<&str>) -> bool {
    if let Some(state) = broadcast_state {
        let normalized = state.trim().to_ascii_uppercase();
        if matches!(normalized.as_str(), "ENDED" | "TIMED_OUT" | "REPLAY") {
            return true;
        }
        if normalized == "RUNNING" {
            return false;
        }
    }

    hls_url.contains("type=replay")
}

fn redact_sensitive_payload(value: &Value) -> Value {
    match value {
        Value::Array(items) => Value::Array(items.iter().map(redact_sensitive_payload).collect()),
        Value::Object(map) => Value::Object(
            map.iter()
                .map(|(key, value)| {
                    if is_sensitive_key(key) {
                        (key.clone(), Value::String("<redacted>".to_owned()))
                    } else {
                        (key.clone(), redact_sensitive_payload(value))
                    }
                })
                .collect::<Map<String, Value>>(),
        ),
        _ => value.clone(),
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let key = key.to_ascii_lowercase();
    key.contains("authorization")
        || key.contains("cookie")
        || key.contains("csrf")
        || key.contains("token")
}
