mod auth;
mod content;
mod live;
mod social;
mod user;

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
        DouyinRunTask::ParseWork { .. }
        | DouyinRunTask::VideoWork { .. }
        | DouyinRunTask::ImageAlbumWork { .. }
        | DouyinRunTask::SlidesWork { .. }
        | DouyinRunTask::TextWork { .. }
        | DouyinRunTask::MusicInfo { .. }
        | DouyinRunTask::DanmakuList { .. } => content::run_task(printer, client, task).await,
        DouyinRunTask::WorkComments { .. }
        | DouyinRunTask::CommentReplies { .. }
        | DouyinRunTask::Search { .. }
        | DouyinRunTask::SuggestWords { .. } => social::run_task(printer, client, task).await,
        DouyinRunTask::UserProfile { .. }
        | DouyinRunTask::UserVideoList { .. }
        | DouyinRunTask::UserFavoriteList { .. }
        | DouyinRunTask::UserRecommendList { .. } => user::run_task(printer, client, task).await,
        DouyinRunTask::LiveRoomInfo { .. } => live::run_task(printer, client, task).await,
        DouyinRunTask::LoginQrcode { .. }
        | DouyinRunTask::EmojiList
        | DouyinRunTask::DynamicEmojiList => auth::run_task(printer, client, task).await,
    }
}
