use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::error::AppError;

use super::super::{
    XiaohongshuMethod,
    api::{XiaohongshuUserNotesOptions, XiaohongshuUserProfileOptions},
    types::{XiaohongshuUserNoteList, XiaohongshuUserProfile},
};
use super::XiaohongshuFetcher;

impl XiaohongshuFetcher {
    async fn fetch_user_page_html(
        &self,
        options: &XiaohongshuUserProfileOptions,
    ) -> Result<(String, String), AppError> {
        let request = self.request_builder().user_profile(options)?;
        let body = self
            .fetch_signed_text(
                XiaohongshuMethod::Get,
                &request.api_path,
                &request.url,
                request.params.as_ref(),
                request.body.as_ref(),
            )
            .await?;

        Ok((body, request.url))
    }

    /// Fetch the Xiaohongshu user profile page and extract structured profile data.
    #[doc(alias = "fetchUserProfile")]
    pub async fn fetch_user_profile(
        &self,
        options: &XiaohongshuUserProfileOptions,
    ) -> Result<XiaohongshuUserProfile, AppError> {
        let (body, url) = self.fetch_user_page_html(options).await?;
        let profile_value = extract_user_profile_data(&body, &url)?;
        let envelope = json!({
            "code": 0,
            "msg": "success",
            "success": true,
            "data": profile_value.clone(),
            "upstream_payload": profile_value
        });

        Ok(serde_json::from_value(envelope)?)
    }

    /// Fetch one page of Xiaohongshu notes published by a user.
    #[doc(alias = "fetchUserNoteList")]
    pub async fn fetch_user_note_list(
        &self,
        options: &XiaohongshuUserNotesOptions,
    ) -> Result<XiaohongshuUserNoteList, AppError> {
        let request = self.request_builder().user_note_list(options)?;
        match self
            .fetch_signed_json(
                XiaohongshuMethod::Get,
                &request.api_path,
                &request.url,
                request.params.as_ref(),
                request.body.as_ref(),
            )
            .await
        {
            Ok(response) => Ok(response),
            Err(error) if should_fallback_to_user_page(options, &error) => {
                let (body, url) = self
                    .fetch_user_page_html(&XiaohongshuUserProfileOptions {
                        user_id: options.user_id.clone(),
                        xsec_token: options.xsec_token.clone(),
                        xsec_source: options.xsec_source.clone(),
                    })
                    .await?;
                let note_list_value = extract_user_note_list_data(&body, &url, options)?;
                let envelope = json!({
                    "code": 0,
                    "msg": "success",
                    "success": true,
                    "data": note_list_value.clone(),
                    "upstream_payload": note_list_value
                });

                Ok(serde_json::from_value(envelope)?)
            }
            Err(error) => Err(error),
        }
    }
}

fn should_fallback_to_user_page(options: &XiaohongshuUserNotesOptions, error: &AppError) -> bool {
    if options
        .cursor
        .as_deref()
        .is_some_and(|cursor| !cursor.trim().is_empty())
    {
        return false;
    }

    matches!(
        error,
        AppError::UpstreamResponse {
            status: Some(status),
            ..
        } if *status == StatusCode::NOT_ACCEPTABLE
    )
}

fn extract_user_note_list_data(
    body: &str,
    url: &str,
    options: &XiaohongshuUserNotesOptions,
) -> Result<Value, AppError> {
    let initial_state = extract_initial_state_json(body, url)?;
    let user_state = initial_state
        .get("user")
        .and_then(Value::as_object)
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("xiaohongshu page {url} is missing `user` state"),
        })?;
    let notes = user_state
        .get("notes")
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("xiaohongshu page {url} is missing `user.notes`"),
        })?;
    let items = flatten_user_note_items(notes, options.num);
    let note_queries = user_state
        .get("noteQueries")
        .cloned()
        .unwrap_or_else(|| Value::Array(Vec::new()));
    let cursor = extract_user_note_cursor(&note_queries)
        .or_else(|| items.last().and_then(extract_note_id))
        .unwrap_or_default();
    let has_more = note_queries
        .as_array()
        .map(|queries| {
            queries.iter().any(|query| {
                query
                    .get("hasMore")
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false);

    Ok(json!({
        "items": items,
        "cursor": cursor,
        "has_more": has_more,
        "user_id": options.user_id,
        "source": "user_page_ssr_fallback",
        "note_queries": note_queries
    }))
}

fn flatten_user_note_items(notes: &Value, num: Option<u32>) -> Vec<Value> {
    let mut items = match notes {
        Value::Array(entries) => entries
            .iter()
            .flat_map(|entry| match entry {
                Value::Array(column_items) => column_items.clone(),
                Value::Object(_) => vec![entry.clone()],
                _ => Vec::new(),
            })
            .collect::<Vec<_>>(),
        _ => Vec::new(),
    };

    if let Some(limit) = num {
        items.truncate(limit as usize);
    }

    items
}

fn extract_user_note_cursor(note_queries: &Value) -> Option<String> {
    note_queries.as_array().and_then(|queries| {
        queries
            .iter()
            .find_map(|query| non_empty_string(query.get("cursor")))
    })
}

fn extract_note_id(item: &Value) -> Option<String> {
    non_empty_string(item.get("id")).or_else(|| {
        item.get("noteCard")
            .and_then(|note_card| non_empty_string(note_card.get("noteId")))
    })
}

fn non_empty_string(value: Option<&Value>) -> Option<String> {
    value
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn extract_user_profile_data(body: &str, url: &str) -> Result<Value, AppError> {
    let initial_state = extract_initial_state_json(body, url)?;
    let value = initial_state
        .pointer("/user/userPageData")
        .cloned()
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("xiaohongshu page {url} is missing `user.userPageData`"),
        })?;
    let result_code = value
        .pointer("/result/code")
        .and_then(Value::as_i64)
        .unwrap_or(-1);

    if result_code != 0 {
        let message = value
            .pointer("/result/message")
            .and_then(Value::as_str)
            .unwrap_or("unknown error");
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!(
                "xiaohongshu request to {url} failed with result.code {result_code}: {message}"
            ),
        });
    }

    Ok(value)
}

fn extract_initial_state_json(body: &str, url: &str) -> Result<Value, AppError> {
    const STATE_MARKER: &str = "window.__INITIAL_STATE__";
    const END_MARKER: &str = "</script>";

    let marker_index = body
        .find(STATE_MARKER)
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!(
                "xiaohongshu page {url} is missing `window.__INITIAL_STATE__`; body preview: {}",
                preview_html(body)
            ),
        })?;
    let after_marker = &body[marker_index + STATE_MARKER.len()..];
    let equals_offset = after_marker
        .find('=')
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("xiaohongshu page {url} is missing `window.__INITIAL_STATE__=`"),
        })?;
    let whitespace_len = after_marker[equals_offset + 1..]
        .chars()
        .take_while(|character| character.is_whitespace())
        .map(char::len_utf8)
        .sum::<usize>();
    let start = marker_index + STATE_MARKER.len() + equals_offset + 1 + whitespace_len;
    let end = body[start..]
        .find(END_MARKER)
        .map(|offset| start + offset)
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!(
                "xiaohongshu page {url} is missing the closing initial-state script; body preview: {}",
                preview_html(body)
            ),
        })?;
    let raw = body[start..end].replace(":undefined", ":null");

    Ok(serde_json::from_str(&raw)?)
}

fn preview_html(body: &str) -> String {
    let collapsed = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let preview = collapsed.chars().take(240).collect::<String>();
    if collapsed.chars().count() > 240 {
        format!("{preview}...")
    } else {
        preview
    }
}
