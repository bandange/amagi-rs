use tracing::info;

use crate::APP_NAME;
use crate::client::AmagiClient;
use crate::config::XiaohongshuRunTask;
use crate::error::AppError;
use crate::output::Printer;
use crate::platforms::xiaohongshu::{XiaohongshuUserNotesOptions, XiaohongshuUserProfileOptions};

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &XiaohongshuRunTask,
) -> Result<(), AppError> {
    match task {
        XiaohongshuRunTask::UserProfile {
            user_id,
            xsec_token,
            xsec_source,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .fetch_user_profile(&XiaohongshuUserProfileOptions {
                    user_id: user_id.clone(),
                    xsec_token: xsec_token.clone(),
                    xsec_source: xsec_source.clone(),
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "userProfile",
                user_id = user_id.as_str(),
                "cli fetch completed"
            );
        }
        XiaohongshuRunTask::UserNoteList {
            user_id,
            xsec_token,
            xsec_source,
            cursor,
            num,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .fetch_user_note_list(&XiaohongshuUserNotesOptions {
                    user_id: user_id.clone(),
                    xsec_token: xsec_token.clone(),
                    xsec_source: xsec_source.clone(),
                    cursor: cursor.clone(),
                    num: *num,
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "userNoteList",
                user_id = user_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported xiaohongshu user task"),
    }

    Ok(())
}
