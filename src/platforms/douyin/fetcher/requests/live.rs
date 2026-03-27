use crate::{error::AppError, platforms::douyin::generate_ms_token};

use super::{
    builder::DouyinRequestBuilder,
    shared::{build_url, join_base},
};

impl DouyinRequestBuilder {
    pub(crate) fn live_room_info(&self, room_id: &str, web_rid: &str) -> Result<String, AppError> {
        let params = vec![
            ("aid", "6383".into()),
            ("app_name", "douyin_web".into()),
            ("live_id", "1".into()),
            ("device_platform", "web".into()),
            ("language", "zh-CN".into()),
            ("enter_from", "web_share_link".into()),
            ("cookie_enabled", "true".into()),
            ("screen_width", "2048".into()),
            ("screen_height", "1152".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Chrome".into()),
            ("browser_version", "125.0.0.0".into()),
            ("web_rid", web_rid.to_owned()),
            ("room_id_str", room_id.to_owned()),
            ("enter_source", String::new()),
            ("is_need_double_stream", "false".into()),
            ("insert_task_id", String::new()),
            ("live_reason", String::new()),
            ("msToken", generate_ms_token(116)),
            ("verifyFp", self.verify_fp.clone()),
            ("fp", self.verify_fp.clone()),
        ];
        build_url(
            &join_base(&self.endpoints.live_base, "/webcast/room/web/enter/"),
            params,
        )
    }

    pub(crate) fn login_qrcode(&self, verify_fp: &str) -> Result<String, AppError> {
        build_url(
            &join_base(&self.endpoints.sso_base, "/get_qrcode/"),
            vec![
                ("verifyFp", verify_fp.to_owned()),
                ("fp", verify_fp.to_owned()),
            ],
        )
    }
}
