use tracing::info;

use crate::APP_NAME;
use crate::config::XiaohongshuRunTask;
use crate::output::Printer;
use amagi_adapters::xiaohongshu::{
    XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions, XiaohongshuNoteDetailOptions,
};
use amagi_client::AmagiClient;
use amagi_core::AppError;

pub(super) async fn run_task(
    printer: &Printer,
    client: &AmagiClient,
    task: &XiaohongshuRunTask,
) -> Result<(), AppError> {
    match task {
        XiaohongshuRunTask::HomeFeed {
            cursor_score,
            num,
            refresh_type,
            note_index,
            category,
            search_key,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .fetch_home_feed(&XiaohongshuHomeFeedOptions {
                    cursor_score: cursor_score.clone(),
                    num: *num,
                    refresh_type: *refresh_type,
                    note_index: *note_index,
                    category: category.clone(),
                    search_key: search_key.clone(),
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "homeFeed",
                "cli fetch completed"
            );
        }
        XiaohongshuRunTask::NoteDetail {
            note_id,
            xsec_token,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .fetch_note_detail(&XiaohongshuNoteDetailOptions {
                    note_id: note_id.clone(),
                    xsec_token: xsec_token.clone(),
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "noteDetail",
                note_id = note_id.as_str(),
                "cli fetch completed"
            );
        }
        XiaohongshuRunTask::NoteComments {
            note_id,
            xsec_token,
            cursor,
        } => {
            let result = client
                .xiaohongshu_fetcher()
                .fetch_note_comments(&XiaohongshuCommentsOptions {
                    note_id: note_id.clone(),
                    cursor: cursor.clone(),
                    xsec_token: xsec_token.clone(),
                })
                .await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "noteComments",
                note_id = note_id.as_str(),
                "cli fetch completed"
            );
        }
        XiaohongshuRunTask::EmojiList => {
            let result = client.xiaohongshu_fetcher().fetch_emoji_list().await?;
            printer.print_payload(&result)?;
            info!(
                app = APP_NAME,
                mode = "cli",
                platform = "xiaohongshu",
                method = "emojiList",
                "cli fetch completed"
            );
        }
        _ => unreachable!("unsupported xiaohongshu content task"),
    }

    Ok(())
}
