use tracing::info;

use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::DouyinRunTask;
use crate::error::AppError;
use crate::output::Printer;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &DouyinRunTask,
) -> Result<(), AppError> {
    match task {
        DouyinRunTask::LoginQrcode { verify_fp } => {
            let result = client
                .douyin_fetcher()
                .request_login_qrcode(verify_fp.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "loginQrcode",
                "cli fetch completed"
            );
        }
        DouyinRunTask::EmojiList => {
            let result = client.douyin_fetcher().fetch_emoji_list().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "emojiList",
                "cli fetch completed"
            );
        }
        DouyinRunTask::DynamicEmojiList => {
            let result = client.douyin_fetcher().fetch_dynamic_emoji_list().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "dynamicEmojiList",
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported douyin auth task"),
    }

    Ok(())
}
