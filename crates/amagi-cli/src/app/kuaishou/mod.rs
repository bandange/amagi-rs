mod content;
mod live;
mod user;

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
        KuaishouRunTask::VideoWork { .. }
        | KuaishouRunTask::WorkComments { .. }
        | KuaishouRunTask::EmojiList => content::run_task(printer, client, task).await?,
        KuaishouRunTask::UserProfile { .. } | KuaishouRunTask::UserWorkList { .. } => {
            user::run_task(printer, client, task).await?
        }
        KuaishouRunTask::LiveRoomInfo { .. } => live::run_task(printer, client, task).await?,
    }

    Ok(())
}
