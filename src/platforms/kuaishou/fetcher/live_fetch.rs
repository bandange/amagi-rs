use serde_json::Value;

use crate::error::AppError;

use super::{
    super::types::{KuaishouEmojiList, KuaishouEmojiListData, KuaishouLiveRoomInfo},
    KuaishouFetcher,
    graphql::EmojiListRequest,
    live::{KuaishouLiveRoomSources, build_kuaishou_live_room_info},
    requests,
};

impl KuaishouFetcher {
    /// Fetch the aggregated live-room info for a Kuaishou principal id.
    ///
    /// # Errors
    ///
    /// Returns an error when the primary live-detail request fails or the
    /// upstream responses cannot be decoded.
    #[doc(alias = "fetchLiveRoomInfo")]
    pub async fn fetch_live_room_info(
        &self,
        principal_id: &str,
    ) -> Result<KuaishouLiveRoomInfo, AppError> {
        let referer_path = format!("u/{principal_id}");
        let base_url = self.live_base_url.as_ref();
        let live_detail_request = requests::live_detail(base_url, principal_id)?;
        let user_info_request = requests::user_info_by_id(base_url, principal_id)?;
        let sensitive_request = requests::user_sensitive_info(base_url, principal_id)?;
        let emoji_request = EmojiListRequest::default();

        let (live_detail_payload, user_info_payload, sensitive_payload, emoji_payload) = tokio::join!(
            self.send_live_api_request(&live_detail_request, &referer_path, true),
            self.send_live_api_request(&user_info_request, &referer_path, true),
            self.send_live_api_request(&sensitive_request, &referer_path, true),
            self.send_graphql_data(&emoji_request),
        );

        let live_detail_payload = live_detail_payload?;
        let user_info_payload = user_info_payload.ok();
        let sensitive_payload = sensitive_payload.ok();
        let emoji_payload = emoji_payload.ok().and_then(|data| {
            serde_json::from_value::<KuaishouEmojiListData>(data.clone())
                .ok()
                .map(|parsed| KuaishouEmojiList {
                    data: parsed,
                    upstream_payload: data,
                })
        });

        let live_stream_id = live_detail_payload
            .get("data")
            .and_then(|value| value.get("liveStream"))
            .and_then(|value| value.get("id"))
            .and_then(Value::as_str)
            .map(str::to_owned)
            .or_else(|| {
                live_detail_payload
                    .get("data")
                    .and_then(|value| value.get("config"))
                    .and_then(|value| value.get("liveStreamId"))
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            });
        let current_game_id = live_detail_payload
            .get("data")
            .and_then(|value| value.get("gameInfo"))
            .and_then(|value| value.get("id").or_else(|| value.get("gameId")))
            .and_then(|value| match value {
                Value::String(value) => Some(value.clone()),
                Value::Number(value) => Some(value.to_string()),
                _ => None,
            });
        let detail_websocket_urls = live_detail_payload
            .get("data")
            .and_then(|value| value.get("websocketInfo"))
            .and_then(|value| value.get("websocketUrls"))
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or_default();
        let detail_websocket_token = live_detail_payload
            .get("data")
            .and_then(|value| value.get("websocketInfo"))
            .and_then(|value| value.get("token"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let detail_recommend_count = live_detail_payload
            .get("data")
            .and_then(|value| value.get("recommendList"))
            .and_then(Value::as_array)
            .map(Vec::len)
            .unwrap_or_default();

        let gift_request = live_stream_id
            .as_deref()
            .map(|id| requests::live_gift_list(base_url, id))
            .transpose()?;
        let websocket_request = (detail_websocket_urls == 0 || detail_websocket_token.is_empty())
            .then(|| {
                live_stream_id
                    .as_deref()
                    .map(|id| requests::live_websocket_info(base_url, id))
                    .transpose()
            })
            .transpose()?
            .flatten();
        let reco_request = (detail_recommend_count == 0)
            .then(|| requests::live_reco(base_url, current_game_id.as_deref()))
            .transpose()?;

        let (gift_payload, websocket_payload, reco_payload) = tokio::join!(
            async {
                match gift_request.as_ref() {
                    Some(request) => self
                        .send_live_api_request(request, &referer_path, true)
                        .await
                        .ok(),
                    None => None,
                }
            },
            async {
                match websocket_request.as_ref() {
                    Some(request) => self
                        .send_live_api_request(request, &referer_path, true)
                        .await
                        .ok(),
                    None => None,
                }
            },
            async {
                match reco_request.as_ref() {
                    Some(request) => self
                        .send_live_api_request(request, &referer_path, true)
                        .await
                        .ok(),
                    None => None,
                }
            },
        );

        Ok(build_kuaishou_live_room_info(
            principal_id,
            KuaishouLiveRoomSources {
                user_info_payload: user_info_payload.as_ref(),
                sensitive_payload: sensitive_payload.as_ref(),
                live_detail_payload: &live_detail_payload,
                emoji_payload: emoji_payload.as_ref(),
                gift_payload: gift_payload.as_ref(),
                websocket_payload: websocket_payload.as_ref(),
                reco_payload: reco_payload.as_ref(),
            },
        ))
    }
}
