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
        KuaishouRunTask::LiveRoomInfo { principal_id } => {
            let result = client
                .kuaishou_fetcher()
                .fetch_live_room_info(principal_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "liveRoomInfo",
                principal_id = principal_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported kuaishou live task"),
    }

    Ok(())
}
