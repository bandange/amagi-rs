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
        DouyinRunTask::ParseWork { aweme_id } => {
            let result = client.douyin_fetcher().parse_work(aweme_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "parseWork",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::VideoWork { aweme_id } => {
            let result = client.douyin_fetcher().fetch_video_work(aweme_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "videoWork",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::ImageAlbumWork { aweme_id } => {
            let result = client
                .douyin_fetcher()
                .fetch_image_album_work(aweme_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "imageAlbumWork",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::SlidesWork { aweme_id } => {
            let result = client.douyin_fetcher().fetch_slides_work(aweme_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "slidesWork",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::TextWork { aweme_id } => {
            let result = client.douyin_fetcher().fetch_text_work(aweme_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "textWork",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::MusicInfo { music_id } => {
            let result = client.douyin_fetcher().fetch_music_info(music_id).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "musicInfo",
                music_id = music_id.as_str(),
                "cli fetch completed"
            );
        }
        DouyinRunTask::DanmakuList {
            aweme_id,
            duration,
            start_time,
            end_time,
        } => {
            let result = client
                .douyin_fetcher()
                .fetch_danmaku_list(aweme_id, *duration, *start_time, *end_time)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "douyin",
                method = "danmakuList",
                aweme_id = aweme_id.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported douyin content task"),
    }

    Ok(())
}
