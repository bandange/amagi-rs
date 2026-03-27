use serde_json::{Value, json};

use crate::error::AppError;

use super::super::types::{DouyinSearchResult, DouyinSearchType, DouyinSuggestWords};
use super::{
    DouyinFetcher, DouyinSignType,
    payload::{
        decode_douyin_payload, encode_search_segment, extract_array_field, has_more_value,
        search_referer, set_array_field,
    },
};

impl DouyinFetcher {
    /// Search Douyin content.
    #[doc(alias = "searchContent")]
    pub async fn search_content(
        &self,
        query: &str,
        search_type: Option<DouyinSearchType>,
        number: Option<u32>,
        search_id: Option<&str>,
    ) -> Result<DouyinSearchResult, AppError> {
        let search_type = search_type.unwrap_or_default();
        let target = number.unwrap_or(10);
        let mut next_search_id = search_id.unwrap_or_default().to_owned();
        let mut items = Vec::new();
        let mut last_response = json!({
            "data": [],
            "has_more": 0
        });

        while items.len() < target as usize {
            let request_count = (target as usize - items.len()).min(15) as u32;
            let url = self.request_builder().search(
                query,
                search_type,
                request_count,
                Some(next_search_id.as_str()),
            )?;
            let referer = search_referer(query, search_type);
            let response = self.fetch_search_json(&url, &referer, search_type).await?;
            let list_field = if matches!(search_type, DouyinSearchType::User) {
                "user_list"
            } else {
                "data"
            };
            let current_items = extract_array_field(&response, list_field);
            items.extend(current_items.clone());
            next_search_id = match search_type {
                DouyinSearchType::User => response
                    .get("rid")
                    .and_then(Value::as_str)
                    .unwrap_or(next_search_id.as_str())
                    .to_owned(),
                DouyinSearchType::General | DouyinSearchType::Video => response
                    .get("log_pb")
                    .and_then(|value| value.get("impr_id"))
                    .and_then(Value::as_str)
                    .unwrap_or(next_search_id.as_str())
                    .to_owned(),
            };
            let has_more = has_more_value(response.get("has_more"));
            last_response = response;
            if !has_more || current_items.is_empty() {
                break;
            }
        }

        let list_field = if matches!(search_type, DouyinSearchType::User) {
            "user_list"
        } else {
            "data"
        };
        set_array_field(&mut last_response, list_field, items, Some(target as usize));
        decode_douyin_payload(last_response)
    }

    /// Fetch Douyin search suggestion keywords.
    #[doc(alias = "fetchSuggestWords")]
    pub async fn fetch_suggest_words(&self, query: &str) -> Result<DouyinSuggestWords, AppError> {
        let url = self.request_builder().suggest_words(query)?;
        self.fetch_json(
            &url,
            DouyinSignType::ABogus,
            Some(format!(
                "https://www.douyin.com/search/{}",
                encode_search_segment(query)
            )),
        )
        .await
        .and_then(decode_douyin_payload)
    }
}
