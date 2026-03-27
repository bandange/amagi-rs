use crate::error::AppError;

use super::{
    builder::DouyinRequestBuilder,
    shared::{build_url, join_base},
};

impl DouyinRequestBuilder {
    pub(crate) fn user_video_list(
        &self,
        sec_uid: &str,
        max_cursor: &str,
        count: u32,
    ) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("sec_user_id", sec_uid.to_owned()),
            ("max_cursor", max_cursor.to_owned()),
            ("locate_query", "false".into()),
            ("show_live_replay_strategy", "1".into()),
            ("need_time_list", "1".into()),
            ("time_list_query", "0".into()),
            ("whale_cut_token", String::new()),
            ("cut_version", "1".into()),
            ("count", count.to_string()),
            ("publish_video_strategy_type", "2".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "1552".into()),
            ("screen_height", "970".into()),
            ("round_trip_time", "50".into()),
            ("webid", "7338423850134226495".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/aweme/post/"),
            params,
        )
    }

    pub(crate) fn user_favorite_list(
        &self,
        sec_uid: &str,
        max_cursor: &str,
        count: u32,
    ) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("sec_user_id", sec_uid.to_owned()),
            ("max_cursor", max_cursor.to_owned()),
            ("min_cursor", "0".into()),
            ("whale_cut_token", String::new()),
            ("cut_version", "1".into()),
            ("count", count.to_string()),
            ("publish_video_strategy_type", "2".into()),
            ("update_version_code", "170400".into()),
            ("pc_libra_divert", "Windows".into()),
            ("support_h265", "1".into()),
            ("support_dash", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("round_trip_time", "0".into()),
            ("webid", "7487210762873685515".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.hj_base, "/aweme/v1/web/aweme/favorite/"),
            params,
        )
    }

    pub(crate) fn user_recommend_list(
        &self,
        sec_uid: &str,
        max_cursor: &str,
        count: u32,
    ) -> Result<String, AppError> {
        let params = vec![
            ("device_platform", String::new()),
            ("aid", "6383".into()),
            ("channel", "channel_pc_web".into()),
            ("sec_user_id", sec_uid.to_owned()),
            ("max_cursor", max_cursor.to_owned()),
            ("min_cursor", "0".into()),
            ("whale_cut_token", String::new()),
            ("count", count.to_string()),
            ("from", "1".into()),
            ("update_version_code", "170400".into()),
            ("pc_client_type", "1".into()),
            ("pc_libra_divert", "Windows".into()),
            ("support_h265", "1".into()),
            ("support_dash", "1".into()),
            ("cpu_core_num", "16".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("cookie_enabled", "true".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Edge".into()),
            ("browser_version", self.browser_version.clone()),
            ("browser_online", "true".into()),
            ("engine_name", "Blink".into()),
            ("engine_version", self.browser_version.clone()),
            ("os_name", "Windows".into()),
            ("os_version", "10".into()),
            ("device_memory", "8".into()),
            ("platform", "PC".into()),
            ("downlink", "10".into()),
            ("effective_type", "4g".into()),
            ("round_trip_time", "50".into()),
            ("webid", "7487210762873685515".into()),
            ("msToken", crate::platforms::douyin::generate_ms_token(184)),
            ("verifyFp", self.verify_fp.clone()),
            ("fp", self.verify_fp.clone()),
        ];
        build_url(
            &join_base(
                &self.endpoints.web_base,
                "/aweme/v1/web/familiar/recommend/feed/",
            ),
            params,
        )
    }

    pub(crate) fn user_profile(&self, sec_uid: &str) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("publish_video_strategy_type", "2".into()),
            ("source", "channel_pc_web".into()),
            ("sec_user_id", sec_uid.to_owned()),
            ("personal_center_strategy", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "1552".into()),
            ("screen_height", "970".into()),
            ("round_trip_time", "0".into()),
            ("webid", "7327957959955580467".into()),
        ]);
        build_url(
            &join_base(
                &self.endpoints.web_base,
                "/aweme/v1/web/user/profile/other/",
            ),
            params,
        )
    }
}
