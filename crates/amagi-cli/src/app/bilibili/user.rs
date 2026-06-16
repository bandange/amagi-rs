use tracing::info;

use crate::APP_NAME;
use crate::config::BilibiliRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &BilibiliRunTask,
) -> Result<(), AppError> {
    match task {
        BilibiliRunTask::UserCard { host_mid } => {
            let result = client.bilibili_fetcher().fetch_user_card(*host_mid).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "userCard",
                host_mid = *host_mid,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::UserDynamicList { host_mid } => {
            let result = client
                .bilibili_fetcher()
                .fetch_user_dynamic_list(*host_mid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "userDynamicList",
                host_mid = *host_mid,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::UserSpaceInfo { host_mid } => {
            let result = client
                .bilibili_fetcher()
                .fetch_user_space_info(*host_mid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "userSpaceInfo",
                host_mid = *host_mid,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::UploaderTotalViews { host_mid } => {
            let result = client
                .bilibili_fetcher()
                .fetch_uploader_total_views(*host_mid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "uploaderTotalViews",
                host_mid = *host_mid,
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili user task"),
    }

    Ok(())
}
