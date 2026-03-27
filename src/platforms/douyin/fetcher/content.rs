use serde_json::{Value, json};

use crate::error::AppError;

use super::super::types::{
    DouyinCommentReplies, DouyinDanmakuList, DouyinDynamicEmojiList, DouyinEmojiList,
    DouyinImageAlbumWork, DouyinLiveRoomInfo, DouyinLoginQrcode, DouyinMusicInfo, DouyinParsedWork,
    DouyinSlidesWork, DouyinTextWork, DouyinUserFavoriteList, DouyinUserProfile,
    DouyinUserRecommendList, DouyinUserVideoList, DouyinVideoWork, DouyinWorkComments,
};
use super::{
    DANMAKU_SEGMENT_MS, DouyinFetcher, DouyinSignType,
    payload::{
        decode_douyin_payload, extract_array_field, has_more_value, set_array_field, set_field,
    },
};

impl DouyinFetcher {
    /// Fetch a Douyin work and let downstream callers infer its type.
    #[doc(alias = "parseWork")]
    pub async fn parse_work(&self, aweme_id: &str) -> Result<DouyinParsedWork, AppError> {
        self.fetch_work_payload(aweme_id)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin video work.
    #[doc(alias = "fetchVideoWork")]
    pub async fn fetch_video_work(&self, aweme_id: &str) -> Result<DouyinVideoWork, AppError> {
        self.fetch_work_payload(aweme_id)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin image album work.
    #[doc(alias = "fetchImageAlbumWork")]
    pub async fn fetch_image_album_work(
        &self,
        aweme_id: &str,
    ) -> Result<DouyinImageAlbumWork, AppError> {
        self.fetch_work_payload(aweme_id)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin slides work.
    #[doc(alias = "fetchSlidesWork")]
    pub async fn fetch_slides_work(&self, aweme_id: &str) -> Result<DouyinSlidesWork, AppError> {
        self.fetch_work_payload(aweme_id)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin text work.
    #[doc(alias = "fetchTextWork")]
    pub async fn fetch_text_work(&self, aweme_id: &str) -> Result<DouyinTextWork, AppError> {
        self.fetch_work_payload(aweme_id)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch comments for one Douyin work.
    #[doc(alias = "fetchWorkComments")]
    pub async fn fetch_work_comments(
        &self,
        aweme_id: &str,
        number: Option<u32>,
        cursor: Option<u64>,
    ) -> Result<DouyinWorkComments, AppError> {
        let target = number.unwrap_or(50);
        let mut next_cursor = cursor.unwrap_or(0);
        let mut comments = Vec::new();
        let mut last_response = json!({
            "comments": [],
            "cursor": next_cursor,
            "has_more": 0
        });

        while comments.len() < target as usize {
            let request_count = (target as usize - comments.len()).min(50) as u32;
            let url = self
                .request_builder()
                .comments(aweme_id, next_cursor, request_count)?;
            let response = self
                .fetch_json(&url, DouyinSignType::ABogus, None::<String>)
                .await?;
            comments.extend(extract_array_field(&response, "comments"));
            next_cursor = response
                .get("cursor")
                .and_then(Value::as_u64)
                .unwrap_or(next_cursor);
            let has_more = has_more_value(response.get("has_more"));
            last_response = response;
            if !has_more {
                break;
            }
        }

        set_array_field(
            &mut last_response,
            "comments",
            comments,
            Some(target as usize),
        );
        set_field(
            &mut last_response,
            "cursor",
            Value::Number(serde_json::Number::from(next_cursor)),
        );
        decode_douyin_payload(last_response)
    }

    /// Fetch replies for a Douyin comment.
    #[doc(alias = "fetchCommentReplies")]
    pub async fn fetch_comment_replies(
        &self,
        aweme_id: &str,
        comment_id: &str,
        number: Option<u32>,
        cursor: Option<u64>,
    ) -> Result<DouyinCommentReplies, AppError> {
        let target = number.unwrap_or(3);
        let mut next_cursor = cursor.unwrap_or(0);
        let mut comments = Vec::new();
        let mut last_response = json!({
            "comments": [],
            "cursor": next_cursor,
            "has_more": 0
        });

        while comments.len() < target as usize {
            let request_count = (target as usize - comments.len()).min(3) as u32;
            let url = self.request_builder().comment_replies(
                aweme_id,
                comment_id,
                next_cursor,
                request_count,
            )?;
            let response = self
                .fetch_json(&url, DouyinSignType::XBogus, None::<String>)
                .await?;
            comments.extend(extract_array_field(&response, "comments"));
            next_cursor = response
                .get("cursor")
                .and_then(Value::as_u64)
                .unwrap_or(next_cursor);
            let has_more = has_more_value(response.get("has_more"));
            last_response = response;
            if !has_more {
                break;
            }
        }

        set_array_field(
            &mut last_response,
            "comments",
            comments,
            Some(target as usize),
        );
        set_field(
            &mut last_response,
            "cursor",
            Value::Number(serde_json::Number::from(next_cursor)),
        );
        decode_douyin_payload(last_response)
    }

    /// Fetch a Douyin user profile.
    #[doc(alias = "fetchUserProfile")]
    pub async fn fetch_user_profile(&self, sec_uid: &str) -> Result<DouyinUserProfile, AppError> {
        let url = self.request_builder().user_profile(sec_uid)?;
        self.fetch_json(
            &url,
            DouyinSignType::ABogus,
            Some(format!("https://www.douyin.com/user/{sec_uid}")),
        )
        .await
        .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin user's published videos.
    #[doc(alias = "fetchUserVideoList")]
    pub async fn fetch_user_video_list(
        &self,
        sec_uid: &str,
        number: Option<u32>,
        max_cursor: Option<&str>,
    ) -> Result<DouyinUserVideoList, AppError> {
        self.fetch_user_list(
            sec_uid,
            number,
            max_cursor,
            |builder, sec_uid, cursor, count| builder.user_video_list(sec_uid, cursor, count),
        )
        .await
        .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin user's favorite list.
    #[doc(alias = "fetchUserFavoriteList")]
    pub async fn fetch_user_favorite_list(
        &self,
        sec_uid: &str,
        number: Option<u32>,
        max_cursor: Option<&str>,
    ) -> Result<DouyinUserFavoriteList, AppError> {
        self.fetch_user_list(
            sec_uid,
            number,
            max_cursor,
            |builder, sec_uid, cursor, count| builder.user_favorite_list(sec_uid, cursor, count),
        )
        .await
        .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin user's recommendation feed.
    #[doc(alias = "fetchUserRecommendList")]
    pub async fn fetch_user_recommend_list(
        &self,
        sec_uid: &str,
        number: Option<u32>,
        max_cursor: Option<&str>,
    ) -> Result<DouyinUserRecommendList, AppError> {
        self.fetch_user_list(
            sec_uid,
            number,
            max_cursor,
            |builder, sec_uid, cursor, count| builder.user_recommend_list(sec_uid, cursor, count),
        )
        .await
        .and_then(decode_douyin_payload)
    }

    /// Fetch Douyin music metadata.
    #[doc(alias = "fetchMusicInfo")]
    pub async fn fetch_music_info(&self, music_id: &str) -> Result<DouyinMusicInfo, AppError> {
        let url = self.request_builder().music_info(music_id)?;
        self.fetch_json(&url, DouyinSignType::ABogus, None::<String>)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch one Douyin live room.
    #[doc(alias = "fetchLiveRoomInfo")]
    pub async fn fetch_live_room_info(
        &self,
        room_id: &str,
        web_rid: &str,
    ) -> Result<DouyinLiveRoomInfo, AppError> {
        let url = self.request_builder().live_room_info(room_id, web_rid)?;
        self.fetch_json(
            &url,
            DouyinSignType::ABogus,
            Some(format!("https://live.douyin.com/{web_rid}")),
        )
        .await
        .and_then(decode_douyin_payload)
    }

    /// Request a Douyin login QR code.
    #[doc(alias = "requestLoginQrcode")]
    pub async fn request_login_qrcode(
        &self,
        verify_fp: Option<&str>,
    ) -> Result<DouyinLoginQrcode, AppError> {
        let verify_fp = verify_fp.unwrap_or(&self.verify_fp);
        let url = self.request_builder().login_qrcode(verify_fp)?;
        self.fetch_json(&url, DouyinSignType::ABogus, None::<String>)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch the Douyin emoji catalog.
    #[doc(alias = "fetchEmojiList")]
    pub async fn fetch_emoji_list(&self) -> Result<DouyinEmojiList, AppError> {
        let url = self.request_builder().emoji_list();
        self.fetch_json(&url, DouyinSignType::None, None::<String>)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch the Douyin animated emoji configuration.
    #[doc(alias = "fetchDynamicEmojiList")]
    pub async fn fetch_dynamic_emoji_list(&self) -> Result<DouyinDynamicEmojiList, AppError> {
        let url = self.request_builder().dynamic_emoji_list()?;
        self.fetch_json(&url, DouyinSignType::ABogus, None::<String>)
            .await
            .and_then(decode_douyin_payload)
    }

    /// Fetch a Douyin danmaku range, segmenting long requests when needed.
    #[doc(alias = "fetchDanmakuList")]
    pub async fn fetch_danmaku_list(
        &self,
        aweme_id: &str,
        duration: u64,
        start_time: Option<u64>,
        end_time: Option<u64>,
    ) -> Result<DouyinDanmakuList, AppError> {
        let start_time = start_time.unwrap_or(0);
        let end_time = end_time.unwrap_or(duration);
        let total_duration = end_time.saturating_sub(start_time);

        if total_duration <= DANMAKU_SEGMENT_MS {
            let url = self
                .request_builder()
                .danmaku_list(aweme_id, start_time, end_time, duration)?;
            return self
                .fetch_json(&url, DouyinSignType::ABogus, None::<String>)
                .await
                .and_then(decode_douyin_payload);
        }

        let mut current_start = start_time;
        let mut merged = Vec::new();
        let mut extra = Value::Null;
        let mut log_pb = Value::Null;
        let mut status_code = 0i64;

        while current_start < end_time {
            let current_end = (current_start + DANMAKU_SEGMENT_MS).min(end_time);
            let url = self.request_builder().danmaku_list(
                aweme_id,
                current_start,
                current_end,
                duration,
            )?;
            let response = self
                .fetch_json(&url, DouyinSignType::ABogus, None::<String>)
                .await?;
            let current_items = extract_array_field(&response, "danmaku_list");
            merged.extend(current_items);

            if extra.is_null() {
                extra = response.get("extra").cloned().unwrap_or(Value::Null);
                log_pb = response.get("log_pb").cloned().unwrap_or(Value::Null);
                status_code = response
                    .get("status_code")
                    .and_then(Value::as_i64)
                    .unwrap_or_default();
            }

            current_start = current_end;
        }

        merged.sort_by_key(|value| {
            value
                .get("offset_time")
                .and_then(Value::as_u64)
                .unwrap_or_default()
        });

        let total = merged.len();

        decode_douyin_payload(json!({
            "danmaku_list": merged,
            "start_time": start_time,
            "end_time": end_time,
            "total": total,
            "status_code": status_code,
            "extra": extra,
            "log_pb": log_pb
        }))
    }
}
