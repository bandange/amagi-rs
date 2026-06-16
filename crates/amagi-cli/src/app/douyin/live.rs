use tracing::info;

use crate::APP_NAME;
use crate::config::DouyinRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &DouyinRunTask,
) -> Result<(), AppError> {
    match task {
        DouyinRunTask::LiveRoomInfo { room_id, web_rid } => {
            let result = client
                .douyin_fetcher()
                .fetch_live_room_info(room_id, web_rid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "liveRoomInfo",
                room_id = room_id.as_str(),
                web_rid = web_rid.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported douyin live task"),
    }

    Ok(())
}
