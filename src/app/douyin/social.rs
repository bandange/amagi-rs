use tracing::info;

use crate::APP_NAME;
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
        DouyinRunTask::WorkComments {
            aweme_id,
            number,
            cursor,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_work_comments(aweme_id, *number, *cursor)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "comments",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::CommentReplies {
            aweme_id,
            comment_id,
            number,
            cursor,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_comment_replies(aweme_id, comment_id, *number, *cursor)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "commentReplies",
                aweme_id = aweme_id.as_str(),
                comment_id = comment_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::Search {
            query,
            search_type,
            number,
            search_id,
        } => {
            let result = client
                .douyin_fetcher()
                .search_content(query, *search_type, *number, search_id.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "search",
                query = query.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::SuggestWords { query } => {
            let result = client.douyin_fetcher().fetch_suggest_words(query).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "suggestWords",
                query = query.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported douyin social task"),
    }

    Ok(())
}
