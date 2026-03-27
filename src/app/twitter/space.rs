use tracing::info;

use super::resolve::resolve_space_reference;
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
        TwitterRunTask::SpaceDetail { space_id } => {
            let space_id = resolve_space_reference(space_id)?;
            let result = fetcher.fetch_space_detail(&space_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "spaceDetail",
                space_id = space_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported twitter space task"),
    }

    Ok(())
}
