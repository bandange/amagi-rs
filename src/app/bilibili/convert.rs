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
        BilibiliRunTask::AvToBv { aid } => {
            let result = client.bilibili_fetcher().convert_av_to_bv(*aid);
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "avToBv",
                aid = *aid,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::BvToAv { bvid } => {
            let result = client.bilibili_fetcher().convert_bv_to_av(bvid)?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "bvToAv",
                bvid = bvid.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili convert task"),
    }

    Ok(())
}
