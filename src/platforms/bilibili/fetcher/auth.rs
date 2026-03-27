use serde_json::Value;

use crate::error::AppError;

use super::super::types::{
    BilibiliLoginQrcode, BilibiliLoginStatus, BilibiliQrcodeStatus, BilibiliQrcodeStatusData,
};
use super::{
    BilibiliFetcher,
    helper::{ensure_bilibili_success, flatten_headers},
    requests,
};

impl BilibiliFetcher {
    /// Fetch the current Bilibili login status payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchLoginStatus")]
    pub async fn fetch_login_status(&self) -> Result<BilibiliLoginStatus, AppError> {
        self.fetch_json(&requests::login_status(self.api_base_url.as_ref())?)
            .await
    }

    /// Request a new Bilibili login QR code.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "requestLoginQrcode")]
    #[doc(alias = "fetchLoginQrcode")]
    pub async fn request_login_qrcode(&self) -> Result<BilibiliLoginQrcode, AppError> {
        self.fetch_json(&requests::login_qrcode(self.passport_base_url.as_ref())?)
            .await
    }

    /// Poll the status of one Bilibili login QR code.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "checkQrcodeStatus")]
    #[doc(alias = "fetchQrcodeStatus")]
    pub async fn check_qrcode_status(
        &self,
        qrcode_key: &str,
    ) -> Result<BilibiliQrcodeStatus, AppError> {
        let url = requests::qrcode_status(self.passport_base_url.as_ref(), qrcode_key)?;
        let (headers, body) = self.send_text_request_with_headers(&url, None).await?;
        let value: Value = serde_json::from_str(&body)?;
        ensure_bilibili_success(&url, &value)?;

        Ok(BilibiliQrcodeStatus {
            code: 0,
            message: value
                .get("message")
                .or_else(|| value.get("msg"))
                .and_then(Value::as_str)
                .unwrap_or("0")
                .to_owned(),
            data: BilibiliQrcodeStatusData {
                data: value.get("data").cloned().unwrap_or(Value::Null),
                headers: flatten_headers(&headers),
            },
            upstream_payload: value.get("data").cloned().unwrap_or(Value::Null),
        })
    }
}
