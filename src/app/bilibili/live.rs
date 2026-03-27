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
        BilibiliRunTask::LiveRoomInfo { room_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_live_room_info(*room_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "liveRoomInfo",
                room_id = *room_id,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::LiveRoomInit { room_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_live_room_init(*room_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "liveRoomInit",
                room_id = *room_id,
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili live task"),
    }

    Ok(())
}
