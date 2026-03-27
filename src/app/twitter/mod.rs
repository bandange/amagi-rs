mod content;
mod resolve;
mod space;
mod user;

use crate::client::AmagiClient;
use crate::config::TwitterRunTask;
use crate::error::AppError;
use crate::output::Printer;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &TwitterRunTask,
) -> Result<(), AppError> {
    match task {
        TwitterRunTask::UserProfile { .. }
        | TwitterRunTask::UserTimeline { .. }
        | TwitterRunTask::UserReplies { .. }
        | TwitterRunTask::UserMedia { .. }
        | TwitterRunTask::UserFollowers { .. }
        | TwitterRunTask::UserFollowing { .. }
        | TwitterRunTask::UserLikes { .. }
        | TwitterRunTask::UserBookmarks { .. }
        | TwitterRunTask::UserFollowed { .. }
        | TwitterRunTask::UserRecommended { .. }
        | TwitterRunTask::SearchUsers { .. } => user::run_task(printer, client, task).await,
        TwitterRunTask::SearchTweets { .. }
        | TwitterRunTask::TweetDetail { .. }
        | TwitterRunTask::TweetReplies { .. }
        | TwitterRunTask::TweetLikers { .. }
        | TwitterRunTask::TweetRetweeters { .. } => content::run_task(printer, client, task).await,
        TwitterRunTask::SpaceDetail { .. } => space::run_task(printer, client, task).await,
    }
}
