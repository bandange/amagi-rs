use tracing::info;

use crate::APP_NAME;
use crate::config::KuaishouRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &KuaishouRunTask,
) -> Result<(), AppError> {
    match task {
        KuaishouRunTask::VideoWork { photo_id } => {
            let result = client.kuaishou_fetcher().fetch_video_work(photo_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "videoWork",
                photo_id = photo_id.as_str(),
                "cli fetch completed"
            );
        }
        KuaishouRunTask::WorkComments { photo_id } => {
            let result = client
                .kuaishou_fetcher()
                .fetch_work_comments(photo_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "comments",
                photo_id = photo_id.as_str(),
                "cli fetch completed"
            );
        }
        KuaishouRunTask::EmojiList => {
            let result = client.kuaishou_fetcher().fetch_emoji_list().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "emojiList",
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported kuaishou content task"),
    }

    Ok(())
}
