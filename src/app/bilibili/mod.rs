mod article;
mod auth;
mod content;
mod convert;
mod live;
mod social;
mod user;

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
        BilibiliRunTask::VideoInfo { .. }
        | BilibiliRunTask::VideoStream { .. }
        | BilibiliRunTask::VideoDanmaku { .. }
        | BilibiliRunTask::BangumiInfo { .. }
        | BilibiliRunTask::BangumiStream { .. } => content::run_task(printer, client, task).await,
        BilibiliRunTask::Comments { .. }
        | BilibiliRunTask::CommentReplies { .. }
        | BilibiliRunTask::DynamicDetail { .. }
        | BilibiliRunTask::DynamicCard { .. } => social::run_task(printer, client, task).await,
        BilibiliRunTask::UserCard { .. }
        | BilibiliRunTask::UserDynamicList { .. }
        | BilibiliRunTask::UserSpaceInfo { .. }
        | BilibiliRunTask::UploaderTotalViews { .. } => user::run_task(printer, client, task).await,
        BilibiliRunTask::LiveRoomInfo { .. } | BilibiliRunTask::LiveRoomInit { .. } => {
            live::run_task(printer, client, task).await
        }
        BilibiliRunTask::LoginStatus
        | BilibiliRunTask::LoginQrcode
        | BilibiliRunTask::QrcodeStatus { .. }
        | BilibiliRunTask::EmojiList
        | BilibiliRunTask::CaptchaFromVoucher { .. }
        | BilibiliRunTask::ValidateCaptcha { .. } => auth::run_task(printer, client, task).await,
        BilibiliRunTask::ArticleContent { .. }
        | BilibiliRunTask::ArticleCards { .. }
        | BilibiliRunTask::ArticleInfo { .. }
        | BilibiliRunTask::ArticleListInfo { .. } => article::run_task(printer, client, task).await,
        BilibiliRunTask::AvToBv { .. } | BilibiliRunTask::BvToAv { .. } => {
            convert::run_task(printer, client, task).await
        }
    }
}
