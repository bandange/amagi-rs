use tracing::info;

use crate::APP_NAME;
use crate::config::XiaohongshuRunTask;
use crate::output::Printer;
use amagi_adapters::xiaohongshu::XiaohongshuSearchNotesOptions;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &XiaohongshuRunTask,
) -> Result<(), AppError> {
    match task {
        XiaohongshuRunTask::Search {
            keyword,
            page,
            page_size,
            sort,
            note_type,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .search_notes(&XiaohongshuSearchNotesOptions {
                    keyword: keyword.clone(),
                    page: *page,
                    page_size: *page_size,
                    sort: *sort,
                    note_type: *note_type,
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "searchNotes",
                keyword = keyword.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported xiaohongshu search task"),
    }

    Ok(())
}
