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
        BilibiliRunTask::VideoInfo { bvid } => {
            let result = client.bilibili_fetcher().fetch_video_info(bvid).await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "videoInfo",
                bvid = bvid.as_str(),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::VideoStream { aid, cid } => {
            let result = client
                .bilibili_fetcher()
                .fetch_video_stream(*aid, *cid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "videoStream",
                aid = *aid,
                cid = *cid,
                "cli fetch completed"
            );
        }
        BilibiliRunTask::VideoDanmaku { cid, segment_index } => {
            let result = client
                .bilibili_fetcher()
                .fetch_video_danmaku(*cid, *segment_index)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "videoDanmaku",
                cid = *cid,
                segment_index = segment_index.unwrap_or(1),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::BangumiInfo { bangumi_id } => {
            let result = client
                .bilibili_fetcher()
                .fetch_bangumi_info(bangumi_id)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "bangumiInfo",
                bangumi_id = bangumi_id.as_str(),
                "cli fetch completed"
            );
        }
        BilibiliRunTask::BangumiStream { ep_id, cid } => {
            let result = client
                .bilibili_fetcher()
                .fetch_bangumi_stream(ep_id, *cid)
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "bilibili",
                method = "bangumiStream",
                ep_id = ep_id.as_str(),
                cid = *cid,
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported bilibili content task"),
    }

    Ok(())
}
