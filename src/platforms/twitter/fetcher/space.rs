use serde_json::Value;

use crate::error::AppError;

use super::TwitterFetcher;
use super::transport::{
    bool_at_path, normalize_upstream_payload, string_at_path, twitter_datetime_to_rfc3339,
    u64_at_path, unwrap_user_result, value_at_path,
};
use super::user::parse_user_summary;
use crate::platforms::twitter::TwitterSpaceDetail;

impl TwitterFetcher {
    /// Fetch one Twitter/X Space by space id.
    #[doc(alias = "fetchSpaceDetail")]
    pub async fn fetch_space_detail(&self, space_id: &str) -> Result<TwitterSpaceDetail, AppError> {
        let value = self
            .fetch_graphql_value(
                &self.api_urls.space_detail(space_id)?,
                Some(&format!(
                    "{}/i/spaces/{space_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;
        parse_space_detail(&value, space_id)
    }
}

fn parse_space_detail(
    value: &Value,
    requested_space_id: &str,
) -> Result<TwitterSpaceDetail, AppError> {
    let audio_space = value_at_path(value, &["data", "audioSpace"]).ok_or_else(|| {
        AppError::UpstreamResponse {
            status: None,
            message: format!("twitter space `{requested_space_id}` was not found"),
        }
    })?;
    let metadata = audio_space
        .get("metadata")
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("twitter space `{requested_space_id}` is missing metadata"),
        })?;
    let id =
        string_at_path(metadata, &["rest_id"]).unwrap_or_else(|| requested_space_id.to_owned());
    let host = value_at_path(metadata, &["creator_results", "result"])
        .and_then(unwrap_user_result)
        .map(parse_user_summary)
        .transpose()?
        .or_else(|| {
            value_at_path(audio_space, &["participants", "admins"])
                .and_then(Value::as_array)
                .and_then(|admins| admins.first())
                .and_then(|admin| admin.get("user_results"))
                .map(parse_user_summary)
                .transpose()
                .ok()
                .flatten()
        });

    Ok(TwitterSpaceDetail {
        id,
        title: string_at_path(metadata, &["title"]),
        state: string_at_path(metadata, &["state"]),
        host,
        created_at: timestamp_or_string(metadata, "created_at"),
        started_at: timestamp_or_string(metadata, "started_at"),
        ended_at: timestamp_or_string(metadata, "ended_at"),
        scheduled_start: timestamp_or_string(metadata, "scheduled_start"),
        replay_enabled: bool_at_path(metadata, &["is_space_available_for_replay"]).unwrap_or(false),
        total_live_listeners: u64_at_path(metadata, &["total_live_listeners"]),
        participant_count: u64_at_path(audio_space, &["participants", "total"])
            .or_else(|| u64_at_path(metadata, &["participant_count"])),
        media_key: string_at_path(metadata, &["media_key"]),
        upstream_payload: normalize_upstream_payload(audio_space),
    })
}

fn timestamp_or_string(value: &Value, key: &str) -> Option<String> {
    let numeric = u64_at_path(value, &[key]);
    if let Some(numeric) = numeric {
        return Some(numeric.to_string());
    }

    twitter_datetime_to_rfc3339(string_at_path(value, &[key]).as_deref())
}
