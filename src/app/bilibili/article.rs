use tracing::info;

use crate::APP_NAME;
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
        BilibiliRunTask::ArticleContent { article_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_article_content(article_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "articleContent",
                article_id = article_id.as_str(),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::ArticleCards { ids } => {
            let result = client
                .bilibili_fetcher()
                .fetch_article_cards(ids.iter())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "articleCards",
                "cli fetch completed"
            );
        }
        BilibiliRunTask::ArticleInfo { article_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_article_info(article_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "articleInfo",
                article_id = article_id.as_str(),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::ArticleListInfo { list_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_article_list_info(list_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "articleListInfo",
                list_id = list_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili article task"),
    }

    Ok(())
}
