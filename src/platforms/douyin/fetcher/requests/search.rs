use crate::{error::AppError, platforms::douyin::DouyinSearchType};

use super::{
    builder::DouyinRequestBuilder,
    shared::{build_url, join_base},
};

impl DouyinRequestBuilder {
    pub(crate) fn suggest_words(&self, query: &str) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("query", query.to_owned()),
            ("business_id", "30088".into()),
            ("from_group_id", "7129543174929812767".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "1552".into()),
            ("screen_height", "970".into()),
            ("round_trip_time", "50".into()),
            ("webid", "7327957959955580467".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/api/suggest_words/"),
            params,
        )
    }

    pub(crate) fn search(
        &self,
        query: &str,
        search_type: DouyinSearchType,
        count: u32,
        search_id: Option<&str>,
    ) -> Result<String, AppError> {
        let mut params = self.base_params(184, false);

        match search_type {
            DouyinSearchType::User => {
                params.extend([
                    ("count", count.to_string()),
                    ("disable_rs", "0".into()),
                    ("from_group_id", String::new()),
                    ("is_filter_search", "0".into()),
                    ("keyword", query.to_owned()),
                    ("list_type", "single".into()),
                    ("need_filter_settings", "1".into()),
                    ("offset", "0".into()),
                    ("pc_libra_divert", "Windows".into()),
                    (
                        "pc_search_top_1_params",
                        "{\"enable_ai_search_top_1\":1}".into(),
                    ),
                    ("query_correct_type", "1".into()),
                    ("round_trip_time", "250".into()),
                    ("screen_height", "1310".into()),
                    ("screen_width", "2328".into()),
                    ("search_channel", "aweme_user_web".into()),
                    ("search_source", "switch_tab".into()),
                    ("support_dash", "1".into()),
                    ("support_h265", "1".into()),
                    ("version_code", "170400".into()),
                    ("version_name", "17.4.0".into()),
                    ("webid", "7521399115230610959".into()),
                ]);

                if let Some(search_id) = search_id.filter(|value| !value.is_empty()) {
                    params.push(("search_id", search_id.to_owned()));
                }

                build_url(
                    &join_base(&self.endpoints.web_base, "/aweme/v1/web/discover/search/"),
                    params,
                )
            }
            DouyinSearchType::Video => {
                params.extend([
                    ("count", count.to_string()),
                    ("disable_rs", "0".into()),
                    ("enable_history", "1".into()),
                    ("from_group_id", String::new()),
                    ("is_filter_search", "0".into()),
                    ("keyword", query.to_owned()),
                    ("list_type", "single".into()),
                    ("need_filter_settings", "1".into()),
                    ("offset", "0".into()),
                    ("pc_libra_divert", "Windows".into()),
                    (
                        "pc_search_top_1_params",
                        "{\"enable_ai_search_top_1\":1}".into(),
                    ),
                    ("query_correct_type", "1".into()),
                    ("round_trip_time", "50".into()),
                    ("screen_height", "1310".into()),
                    ("screen_width", "2328".into()),
                    ("search_channel", "aweme_video_web".into()),
                    ("search_source", "switch_tab".into()),
                    ("support_dash", "1".into()),
                    ("support_h265", "1".into()),
                    ("version_code", "170400".into()),
                    ("version_name", "17.4.0".into()),
                    ("webid", "7521399115230610959".into()),
                ]);

                if let Some(search_id) = search_id.filter(|value| !value.is_empty()) {
                    params.push(("search_id", search_id.to_owned()));
                }

                build_url(
                    &join_base(&self.endpoints.web_base, "/aweme/v1/web/search/item/"),
                    params,
                )
            }
            DouyinSearchType::General => {
                params.extend([
                    ("count", count.to_string()),
                    ("disable_rs", "0".into()),
                    ("enable_history", "1".into()),
                    ("is_filter_search", "0".into()),
                    ("keyword", query.to_owned()),
                    ("list_type", String::new()),
                    ("need_filter_settings", "1".into()),
                    ("offset", "0".into()),
                    ("pc_libra_divert", "Windows".into()),
                    (
                        "pc_search_top_1_params",
                        "{\"enable_ai_search_top_1\":1}".into(),
                    ),
                    ("query_correct_type", "1".into()),
                    ("round_trip_time", "0".into()),
                    ("screen_height", "1310".into()),
                    ("screen_width", "2328".into()),
                    ("search_channel", "aweme_general".into()),
                    ("search_source", "normal_search".into()),
                    ("support_dash", "1".into()),
                    ("support_h265", "1".into()),
                    ("version_code", "190600".into()),
                    ("version_name", "19.6.0".into()),
                    ("webid", "7521399115230610959".into()),
                ]);
                build_url(
                    &join_base(
                        &self.endpoints.web_base,
                        "/aweme/v1/web/general/search/stream/",
                    ),
                    params,
                )
            }
        }
    }
}
