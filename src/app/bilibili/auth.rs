use tracing::info;

use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::BilibiliRunTask;
use crate::error::AppError;
use crate::output::Printer;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &BilibiliRunTask,
) -> Result<(), AppError> {
    match task {
        BilibiliRunTask::LoginStatus => {
            let result = client.bilibili_fetcher().fetch_login_status().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "loginStatus",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::LoginQrcode => {
            let result = client.bilibili_fetcher().request_login_qrcode().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "loginQrcode",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::QrcodeStatus { qrcode_key } => {
            let result = client
                .bilibili_fetcher()
                .check_qrcode_status(qrcode_key)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "qrcodeStatus",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::EmojiList => {
            let result = client.bilibili_fetcher().fetch_emoji_list().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "emojiList",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::CaptchaFromVoucher { v_voucher, csrf } => {
            let result = client
                .bilibili_fetcher()
                .request_captcha_from_voucher(v_voucher, csrf.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "captchaFromVoucher",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::ValidateCaptcha {
            challenge,
            token,
            validate,
            seccode,
            csrf,
        } => {
            let result = client
                .bilibili_fetcher()
                .validate_captcha_result(challenge, token, validate, seccode, csrf.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "validateCaptcha",
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili auth task"),
    }

    Ok(())
}
