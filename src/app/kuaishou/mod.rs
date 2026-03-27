mod content;
mod live;
mod user;

use crate::client::AmagiClient;
use crate::config::KuaishouRunTask;
use crate::error::AppError;
use crate::output::Printer;

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
