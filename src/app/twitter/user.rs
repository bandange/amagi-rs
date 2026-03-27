use tracing::info;

use super::resolve::resolve_user_reference;
use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::TwitterRunTask;
use crate::error::AppError;
use crate::output::Printer;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &TwitterRunTask,
) -> Result<(), AppError> {
    let fetcher = client.twitter_fetcher();

    match task {
        TwitterRunTask::UserProfile { screen_name } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher.fetch_user_profile(&screen_name).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userProfile",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserTimeline {
            screen_name,
            count,
            cursor,
        } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher
                .fetch_user_timeline(&screen_name, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userTimeline",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserReplies {
            screen_name,
            count,
            cursor,
        } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher
                .fetch_user_replies(&screen_name, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userReplies",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserMedia {
            screen_name,
            count,
            cursor,
        } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher
                .fetch_user_media(&screen_name, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userMedia",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserFollowers {
            screen_name,
            count,
            cursor,
        } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher
                .fetch_user_followers(&screen_name, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userFollowers",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserFollowing {
            screen_name,
            count,
            cursor,
        } => {
            let screen_name = resolve_user_reference(screen_name)?;
            let result = fetcher
                .fetch_user_following(&screen_name, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userFollowing",
                screen_name = screen_name.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserLikes { count, cursor } => {
            let result = fetcher.fetch_user_likes(*count, cursor.as_deref()).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userLikes",
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserBookmarks { count, cursor } => {
            let result = fetcher
                .fetch_user_bookmarks(*count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userBookmarks",
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserFollowed { count, cursor } => {
            let result = fetcher
                .fetch_user_followed(*count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userFollowed",
                "cli fetch completed"
            );
        }
        TwitterRunTask::UserRecommended { count, cursor } => {
            let result = fetcher
                .fetch_user_recommended(*count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "userRecommended",
                "cli fetch completed"
            );
        }
        TwitterRunTask::SearchUsers {
            query,
            count,
            cursor,
        } => {
            let result = fetcher
                .search_users(query, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "searchUsers",
                query = query.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported twitter user task"),
    }

    Ok(())
}
