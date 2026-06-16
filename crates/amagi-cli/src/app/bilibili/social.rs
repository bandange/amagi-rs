use tracing::info;

use crate::APP_NAME;
use crate::config::BilibiliRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &BilibiliRunTask,
) -> Result<(), AppError> {
    match task {
        BilibiliRunTask::Comments {
            oid,
            comment_type,
            number,
            mode,
        } => {
            let result = client
                .bilibili_fetcher()
                .fetch_comments(*oid, *comment_type, *number, *mode)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "comments",
                oid = *oid,
                comment_type = *comment_type,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::CommentReplies {
            oid,
            comment_type,
            root,
            number,
        } => {
            let result = client
                .bilibili_fetcher()
                .fetch_comment_replies(*oid, *comment_type, *root, *number)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "commentReplies",
                oid = *oid,
                root = *root,
                comment_type = *comment_type,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::DynamicDetail { dynamic_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_dynamic_detail(dynamic_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "dynamicDetail",
                dynamic_id = dynamic_id.as_str(),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::DynamicCard { dynamic_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_dynamic_card(dynamic_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "dynamicCard",
                dynamic_id = dynamic_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili social task"),
    }

    Ok(())
}
