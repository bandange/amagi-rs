#![allow(missing_docs)]

use clap::Subcommand;

/// Xiaohongshu tasks exposed through the CLI.
#[derive(Debug, Subcommand, Clone)]
pub enum XiaohongshuCommand {
    #[command(name = "home-feed")]
    HomeFeed {
        #[arg(long)]
        cursor_score: Option<String>,
        #[arg(long)]
        num: Option<u32>,
        #[arg(long)]
        refresh_type: Option<u32>,
        #[arg(long)]
        note_index: Option<u32>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        search_key: Option<String>,
    },
    #[command(name = "note-detail")]
    NoteDetail {
        note_id: String,
        #[arg(long)]
        xsec_token: String,
    },
    #[command(name = "note-comments")]
    NoteComments {
        note_id: String,
        #[arg(long)]
        xsec_token: String,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-profile")]
    UserProfile {
        user_id: String,
        #[arg(long)]
        xsec_token: String,
        #[arg(long)]
        xsec_source: Option<String>,
    },
    #[command(name = "user-note-list")]
    UserNoteList {
        user_id: String,
        #[arg(long)]
        xsec_token: String,
        #[arg(long)]
        xsec_source: Option<String>,
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long)]
        num: Option<u32>,
    },
    #[command(name = "emoji-list")]
    EmojiList,
    #[command(name = "search")]
    Search {
        keyword: String,
        #[arg(long)]
        page: Option<u32>,
        #[arg(long)]
        page_size: Option<u32>,
        #[arg(long, value_enum)]
        sort: Option<crate::platforms::xiaohongshu::XiaohongshuSearchSortType>,
        #[arg(long, value_enum)]
        note_type: Option<crate::platforms::xiaohongshu::XiaohongshuSearchNoteType>,
    },
}
