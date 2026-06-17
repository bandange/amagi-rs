use tracing::info;

use super::resolve::resolve_tweet_reference;
use crate::APP_NAME;
use crate::config::TwitterRunTask;
use crate::output::Printer;
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &TwitterRunTask,
) -> Result<(), AppError> {
    let fetcher = client.twitter_fetcher();

    match task {
        TwitterRunTask::LiveRoomStream {
            broadcast_id,
            media_key,
            tweet_id,
        } => {
            let provided = [
                broadcast_id.as_deref(),
                media_key.as_deref(),
                tweet_id.as_deref(),
            ]
            .into_iter()
            .flatten()
            .count();

            if provided != 1 {
                return Err(AppError::InvalidRequestConfig(
                    "twitter live-room-stream requires exactly one of `broadcast_id`, `--media-key`, or `--tweet-id`"
                        .into(),
                ));
            }

            let result = if let Some(broadcast_id) = broadcast_id {
                fetcher.fetch_live_room_stream(broadcast_id).await?
            } else if let Some(media_key) = media_key {
                fetcher
                    .fetch_live_room_stream_by_media_key(media_key)
                    .await?
            } else {
                let tweet_id = resolve_tweet_reference(
                    tweet_id
                        .as_deref()
                        .expect("validated live-room-stream tweet id presence"),
                )?;
                fetcher
                    .fetch_live_room_stream_by_tweet_id(&tweet_id)
                    .await?
            };

            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "twitter",
                method = "liveRoomStream",
                broadcast_id = result.broadcast_id.as_deref().unwrap_or_default(),
                media_key = result.media_key.as_str(),
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported twitter stream task"),
    }

    Ok(())
}
