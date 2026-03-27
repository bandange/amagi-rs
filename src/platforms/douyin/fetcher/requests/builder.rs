use crate::platforms::douyin::generate_ms_token;

use super::shared::extract_browser_version;

/// Host set used by the Douyin platform request builders.
#[derive(Debug, Clone)]
pub(crate) struct DouyinApiEndpoints {
    pub(crate) web_base: String,
    pub(crate) hj_base: String,
    pub(crate) live_base: String,
    pub(crate) sso_base: String,
}

impl Default for DouyinApiEndpoints {
    fn default() -> Self {
        Self {
            web_base: "https://www.douyin.com".into(),
            hj_base: "https://www-hj.douyin.com".into(),
            live_base: "https://live.douyin.com".into(),
            sso_base: "https://sso.douyin.com".into(),
        }
    }
}

/// URL builder mirroring the original TypeScript Douyin API layer.
#[derive(Debug, Clone)]
pub(crate) struct DouyinRequestBuilder {
    pub(super) browser_version: String,
    pub(super) verify_fp: String,
    pub(super) endpoints: DouyinApiEndpoints,
}

impl DouyinRequestBuilder {
    /// Create a request builder using the effective browser user-agent.
    pub(crate) fn new(
        user_agent: Option<&str>,
        verify_fp: impl Into<String>,
        endpoints: DouyinApiEndpoints,
    ) -> Self {
        Self {
            browser_version: extract_browser_version(user_agent),
            verify_fp: verify_fp.into(),
            endpoints,
        }
    }

    pub(super) fn base_params(
        &self,
        ms_token_len: usize,
        include_fp: bool,
    ) -> Vec<(&'static str, String)> {
        let mut params = vec![
            ("device_platform", "webapp".into()),
            ("aid", "6383".into()),
            ("channel", "channel_pc_web".into()),
            ("pc_client_type", "1".into()),
            ("cookie_enabled", "true".into()),
            ("browser_language", "zh-CN".into()),
            ("browser_platform", "Win32".into()),
            ("browser_name", "Chrome".into()),
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
            ("msToken", generate_ms_token(ms_token_len)),
        ];

        if include_fp {
            params.push(("verifyFp", self.verify_fp.clone()));
            params.push(("fp", self.verify_fp.clone()));
        }

        params
    }
}
