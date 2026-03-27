use tracing::info;

use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::KuaishouRunTask;
use crate::error::AppError;
use crate::output::Printer;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &KuaishouRunTask,
) -> Result<(), AppError> {
    match task {
        KuaishouRunTask::UserProfile { principal_id } => {
            let result = client
                .kuaishou_fetcher()
                .fetch_user_profile(principal_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "userProfile",
                principal_id = principal_id.as_str(),
                "cli fetch completed"
            );
        }
        KuaishouRunTask::UserWorkList {
            principal_id,
            pcursor,
            count,
        } => {
            let result = client
                .kuaishou_fetcher()
                .fetch_user_work_list(principal_id, *count, pcursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "kuaishou",
                method = "userWorkList",
                principal_id = principal_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported kuaishou user task"),
    }

    Ok(())
}
