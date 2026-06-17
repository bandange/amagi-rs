use serde_json::Value;

use amagi_core::AppError;

use super::TwitterFetcher;
use super::transport::{
    bool_at_path, normalize_upstream_payload, string_at_path, u64_at_path, value_at_path,
};
use crate::twitter::{TwitterLiveRoomInfo, TwitterLiveVideoBroadcast};

impl TwitterFetcher {
    /// Fetch Twitter/X live-room information by screen name.
    #[doc(alias = "fetchLiveRoomInfo")]
    pub async fn fetch_live_room_info(
        &self,
        screen_name: &str,
    ) -> Result<TwitterLiveRoomInfo, AppError> {
        let profile = self.fetch_user_profile(screen_name).await?;
        let mut info = self.fetch_live_room_info_by_user_id(&profile.id).await?;
        info.screen_name = Some(profile.screen_name);
        Ok(info)
    }

    /// Fetch Twitter/X live-room information by numeric user id.
    #[doc(alias = "fetchLiveRoomInfoByUserId")]
    pub async fn fetch_live_room_info_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<TwitterLiveRoomInfo, AppError> {
        let user_id = normalize_user_id(user_id)?;
        self.ensure_authenticated_session("live-room-info")?;
        let value = self
            .fetch_web_api_value(
                &self.api_urls.user_live_room_info(user_id)?,
                Some(&format!(
                    "{}/i/user/{user_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;

        parse_live_room_info(&value, user_id)
    }
}

fn parse_live_room_info(value: &Value, user_id: &str) -> Result<TwitterLiveRoomInfo, AppError> {
    let Some(user_entry) = value.get("users").and_then(|users| users.get(user_id)) else {
        return Ok(TwitterLiveRoomInfo {
            user_id: user_id.to_owned(),
            screen_name: None,
            is_live: false,
            refresh_delay_secs: u64_at_path(value, &["refresh_delay_secs"]),
            live_video: None,
            upstream_payload: normalize_upstream_payload(value),
        });
    };
    let live_video = value_at_path(user_entry, &["spaces", "live_content", "livevideo"])
        .map(parse_live_video_broadcast)
        .transpose()?;
    let screen_name = live_video
        .as_ref()
        .and_then(|video| video.twitter_username.clone())
        .or_else(|| string_at_path(user_entry, &["screen_name"]))
        .or_else(|| string_at_path(user_entry, &["username"]));
    let is_live = live_video
        .as_ref()
        .and_then(|video| video.state.as_deref())
        .is_some_and(|state| state.eq_ignore_ascii_case("RUNNING"));

    Ok(TwitterLiveRoomInfo {
        user_id: user_id.to_owned(),
        screen_name,
        is_live,
        refresh_delay_secs: u64_at_path(value, &["refresh_delay_secs"]),
        live_video,
        upstream_payload: normalize_upstream_payload(value),
    })
}

fn normalize_user_id(value: &str) -> Result<&str, AppError> {
    let user_id = value.trim();
    if !user_id.is_empty() && user_id.bytes().all(|byte| byte.is_ascii_digit()) {
        return Ok(user_id);
    }

    Err(AppError::InvalidRequestConfig(format!(
        "twitter user id `{value}` must be numeric"
    )))
}

fn parse_live_video_broadcast(value: &Value) -> Result<TwitterLiveVideoBroadcast, AppError> {
    let broadcast_id = string_at_path(value, &["id"])
        .or_else(|| string_at_path(value, &["broadcast_id"]))
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: "twitter live-room broadcast is missing `id`".into(),
        })?;
    let media_key = string_at_path(value, &["media_key"]);

    Ok(TwitterLiveVideoBroadcast {
        broadcast_id,
        state: string_at_path(value, &["state"]),
        title: string_at_path(value, &["status"]).or_else(|| string_at_path(value, &["title"])),
        media_id: string_at_path(value, &["media_id"])
            .or_else(|| media_key.as_deref().and_then(media_id_from_media_key)),
        media_key,
        tweet_id: string_at_path(value, &["tweet_id"]),
        periscope_user_id: string_at_path(value, &["user_id"]),
        twitter_user_id: string_at_path(value, &["twitter_id"])
            .or_else(|| string_at_path(value, &["twitter_user_id"])),
        twitter_username: string_at_path(value, &["twitter_username"])
            .or_else(|| string_at_path(value, &["username"])),
        display_name: string_at_path(value, &["user_display_name"]),
        available_for_replay: bool_at_path(value, &["available_for_replay"]).unwrap_or(false),
        started_at: string_at_path(value, &["start"])
            .or_else(|| string_at_path(value, &["start_ms"])),
        ping_at: string_at_path(value, &["ping"]).or_else(|| string_at_path(value, &["ping_ms"])),
        width: u64_at_path(value, &["width"]),
        height: u64_at_path(value, &["height"]),
        total_watching: u64_at_path(value, &["total_watching"])
            .or_else(|| u64_at_path(value, &["n_watching"])),
        total_watched: u64_at_path(value, &["total_watched"])
            .or_else(|| u64_at_path(value, &["n_total_watching"])),
        broadcast_source: string_at_path(value, &["broadcast_source"]),
        is_high_latency: bool_at_path(value, &["is_high_latency"]),
        upstream_payload: normalize_upstream_payload(value),
    })
}

fn media_id_from_media_key(media_key: &str) -> Option<String> {
    media_key
        .strip_prefix("28_")
        .filter(|value| !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()))
        .map(str::to_owned)
}
