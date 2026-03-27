use crate::{error::AppError, platforms::douyin::generate_ms_token};

use super::{
    builder::DouyinRequestBuilder,
    shared::{build_url, join_base},
};

impl DouyinRequestBuilder {
    pub(crate) fn emoji_list(&self) -> String {
        join_base(&self.endpoints.web_base, "/aweme/v1/web/emoji/list")
    }

    pub(crate) fn dynamic_emoji_list(&self) -> Result<String, AppError> {
        let params = vec![
            ("device_platform", "webapp".into()),
            ("aid", "1128".into()),
            ("channel", "channel_pc_web".into()),
            ("publish_video_strategy_type", "2".into()),
            ("app_id", "1128".into()),
            ("scenes", "[\"interactive_resources\"]".into()),
            ("pc_client_type", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("cookie_enabled", "true".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Chrome".into()),
            ("browser_version", "126.0.0.0".into()),
            ("browser_online", "true".into()),
            ("engine_name", "Blink".into()),
            ("engine_version", "126.0.0.0".into()),
            ("os_name", "Windows".into()),
            ("os_version", "10".into()),
            ("cpu_core_num", "16".into()),
            ("device_memory", "8".into()),
            ("platform", "PC".into()),
            ("downlink", "1.5".into()),
            ("effective_type", "4g".into()),
            ("round_trip_time", "350".into()),
            ("webid", "7347329698282833447".into()),
            ("msToken", generate_ms_token(116)),
            ("verifyFp", self.verify_fp.clone()),
            ("fp", self.verify_fp.clone()),
        ];
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/im/strategy/config"),
            params,
        )
    }

    pub(crate) fn music_info(&self, music_id: &str) -> Result<String, AppError> {
        let params = vec![
            ("device_platform", "webapp".into()),
            ("aid", "6383".into()),
            ("channel", "channel_pc_web".into()),
            ("music_id", music_id.to_owned()),
            ("scene", "1".into()),
            ("pc_client_type", "1".into()),
            ("version_code", "170400".into()),
            ("version_name", "17.4.0".into()),
            ("cookie_enabled", "true".into()),
            ("screen_width", "2328".into()),
            ("screen_height", "1310".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Chrome".into()),
            ("browser_version", "126.0.0.0".into()),
            ("browser_online", "true".into()),
            ("engine_name", "Blink".into()),
            ("engine_version", "126.0.0.0".into()),
            ("os_name", "Windows".into()),
            ("os_version", "10".into()),
            ("cpu_core_num", "16".into()),
            ("device_memory", "8".into()),
            ("platform", "PC".into()),
            ("downlink", "1.5".into()),
            ("effective_type", "4g".into()),
            ("round_trip_time", "350".into()),
            ("webid", "7347329698282833447".into()),
            ("msToken", generate_ms_token(116)),
            ("verifyFp", self.verify_fp.clone()),
            ("fp", self.verify_fp.clone()),
        ];
        build_url(
            &join_base(&self.endpoints.web_base, "/aweme/v1/web/music/detail/"),
            params,
        )
    }
}
