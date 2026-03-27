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
        DouyinRunTask::UserProfile { sec_uid } => {
            let result = client.douyin_fetcher().fetch_user_profile(sec_uid).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "userProfile",
                sec_uid = sec_uid.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::UserVideoList {
            sec_uid,
            number,
            max_cursor,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_user_video_list(sec_uid, *number, max_cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "userVideoList",
                sec_uid = sec_uid.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::UserFavoriteList {
            sec_uid,
            number,
            max_cursor,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_user_favorite_list(sec_uid, *number, max_cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "userFavoriteList",
                sec_uid = sec_uid.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::UserRecommendList {
            sec_uid,
            number,
            max_cursor,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_user_recommend_list(sec_uid, *number, max_cursor.as_deref())
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "userRecommendList",
                sec_uid = sec_uid.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported douyin user task"),
    }

    Ok(())
}
