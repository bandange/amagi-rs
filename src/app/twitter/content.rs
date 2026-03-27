use tracing::info;

use super::resolve::resolve_tweet_reference;
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
        TwitterRunTask::SearchTweets {
            query,
            search_type,
            count,
            cursor,
        } => {
            let result = fetcher
                .search_tweets(query, *search_type, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "searchTweets",
                query = query.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::TweetDetail { tweet_id } => {
            let tweet_id = resolve_tweet_reference(tweet_id)?;
            let result = fetcher.fetch_tweet_detail(&tweet_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "tweetDetail",
                tweet_id = tweet_id.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::TweetReplies {
            tweet_id,
            cursor,
            sort_by,
        } => {
            let tweet_id = resolve_tweet_reference(tweet_id)?;
            let result = fetcher
                .fetch_tweet_replies(&tweet_id, cursor.as_deref(), *sort_by)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "tweetReplies",
                tweet_id = tweet_id.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::TweetLikers {
            tweet_id,
            count,
            cursor,
        } => {
            let tweet_id = resolve_tweet_reference(tweet_id)?;
            let result = fetcher
                .fetch_tweet_likers(&tweet_id, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "tweetLikers",
                tweet_id = tweet_id.as_str(),
                "cli fetch completed"
            );
        }
        TwitterRunTask::TweetRetweeters {
            tweet_id,
            count,
            cursor,
        } => {
            let tweet_id = resolve_tweet_reference(tweet_id)?;
            let result = fetcher
                .fetch_tweet_retweeters(&tweet_id, *count, cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "tweetRetweeters",
                tweet_id = tweet_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported twitter content task"),
    }

    Ok(())
}
