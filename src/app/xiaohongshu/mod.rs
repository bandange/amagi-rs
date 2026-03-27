mod content;
mod search;
mod user;

use crate::client::AmagiClient;
use crate::config::XiaohongshuRunTask;
use crate::error::AppError;
use crate::output::Printer;

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
