mod content;
mod search;
mod user;

use crate::config::XiaohongshuRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &XiaohongshuRunTask,
) -> Result<(), AppError> {
    match task {
        XiaohongshuRunTask::HomeFeed { .. }
        | XiaohongshuRunTask::NoteDetail { .. }
        | XiaohongshuRunTask::NoteComments { .. }
        | XiaohongshuRunTask::EmojiList => content::run_task(printer, client, task).await?,
        XiaohongshuRunTask::UserProfile { .. } | XiaohongshuRunTask::UserNoteList { .. } => {
            user::run_task(printer, client, task).await?
        }
        XiaohongshuRunTask::Search { .. } => search::run_task(printer, client, task).await?,
    }

    Ok(())
}
