use crate::{error::AppError, platforms::douyin::generate_ms_token};

use super::{
    builder::DouyinRequestBuilder,
    shared::{build_url, join_base},
};

impl DouyinRequestBuilder {
    pub(crate) fn work_detail(&self, aweme_id: &str) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("aweme_id", aweme_id.to_owned()),
            ("update_version_code", "170400".into()),
            ("version_code", "190500".into()),
            ("version_name", "19.5.0".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("round_trip_time", "150".into()),
            ("webid", "7351848354471872041".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/aweme/detail/"),
            params,
        )
    }

    pub(crate) fn comments(
        &self,
        aweme_id: &str,
        cursor: u64,
        count: u32,
    ) -> Result<String, AppError> {
        let mut params = self.base_params(184, true);
        params.extend([
            ("aweme_id", aweme_id.to_owned()),
            ("cursor", cursor.to_string()),
            ("count", count.to_string()),
            ("item_type", "0".into()),
            ("insert_ids", String::new()),
            ("whale_cut_token", String::new()),
            ("cut_version", "1".into()),
            ("rcFT", String::new()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "1552".into()),
            ("screen_height", "970".into()),
            ("round_trip_time", "50".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/comment/list/"),
            params,
        )
    }

    pub(crate) fn comment_replies(
        &self,
        aweme_id: &str,
        comment_id: &str,
        cursor: u64,
        count: u32,
    ) -> Result<String, AppError> {
        let params = vec![
            ("device_platform", "webapp".into()),
            ("aid", "6383".into()),
            ("channel", "channel_pc_web".into()),
            ("item_id", aweme_id.to_owned()),
            ("comment_id", comment_id.to_owned()),
            ("cut_version", "1".into()),
            ("cursor", cursor.to_string()),
            ("count", count.to_string()),
            ("item_type", "0".into()),
            ("update_version_code", "170400".into()),
            ("pc_client_type", "1".into()),
            ("pc_libra_divert", "Windows".into()),
            ("support_h265", "1".into()),
            ("support_dash", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("cookie_enabled", "true".into()),
            ("screen_width", "1552".into()),
            ("screen_height", "970".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Edge".into()),
            ("browser_version", self.browser_version.clone()),
            ("browser_online", "true".into()),
            ("engine_name", "Blink".into()),
            ("engine_version", self.browser_version.clone()),
            ("os_name", "Windows".into()),
            ("os_version", "10".into()),
            ("cpu_core_num", "16".into()),
            ("device_memory", "8".into()),
            ("platform", "PC".into()),
            ("downlink", "10".into()),
            ("effective_type", "4g".into()),
            ("round_trip_time", "50".into()),
            ("webid", "7487210762873685515".into()),
            ("verifyFp", self.verify_fp.clone()),
            ("fp", self.verify_fp.clone()),
        ];
        build_url(
            &join_base(&self.endpoints.hj_base, "/aweme/v1/web/comment/list/reply/"),
            params,
        )
    }

    pub(crate) fn slides_info(&self, aweme_id: &str) -> Result<String, AppError> {
        build_url(
            "https://www.iesdouyin.com/web/api/v2/aweme/slidesinfo/",
            vec![
                ("reflow_source", "reflow_page".into()),
                ("web_id", "7326472315356857893".into()),
                ("device_id", "7326472315356857893".into()),
                ("aweme_ids", format!("[{aweme_id}]")),
                ("request_source", "200".into()),
                ("msToken", generate_ms_token(116)),
                ("verifyFp", self.verify_fp.clone()),
                ("fp", self.verify_fp.clone()),
            ],
        )
    }

    pub(crate) fn danmaku_list(
        &self,
        aweme_id: &str,
        start_time: u64,
        end_time: u64,
        duration: u64,
    ) -> Result<String, AppError> {
        let mut params = self.base_params(116, true);
        params.extend([
            ("app_name", "aweme".into()),
            ("format", "json".into()),
            ("group_id", aweme_id.to_owned()),
            ("item_id", aweme_id.to_owned()),
            ("start_time", start_time.to_string()),
            ("end_time", end_time.to_string()),
            ("duration", duration.to_string()),
            ("update_version_code", "170400".into()),
            ("pc_libra_divert", "Windows".into()),
            ("support_h265", "1".into()),
            ("support_dash", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("browser_name", "Edge".into()),
            ("browser_version", "140.0.0.0".into()),
            ("engine_name", "Blink".into()),
            ("engine_version", "140.0.0.0".into()),
            ("downlink", "1.55".into()),
            ("round_trip_time", "200".into()),
            ("webid", "7487210762873685515".into()),
        ]);
        build_url(
            &join_base(&self.endpoints.hj_base, "/aweme/v1/web/danmaku/get_v2/"),
            params,
        )
    }
}
