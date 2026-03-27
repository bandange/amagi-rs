#![allow(missing_docs)]

use clap::Subcommand;

/// Kuaishou tasks exposed through the CLI.
#[derive(Debug, Subcommand, Clone)]
pub enum KuaishouCommand {
    #[command(name = "video-work")]
    VideoWork { photo_id: String },
    #[command(name = "work-comments")]
    WorkComments { photo_id: String },
    #[command(name = "emoji-list")]
    EmojiList,
    #[command(name = "user-profile")]
    UserProfile { principal_id: String },
    #[command(name = "user-work-list")]
    UserWorkList {
        principal_id: String,
        #[arg(long)]
        pcursor: Option<String>,
        #[arg(long)]
        count: Option<u32>,
    },
    #[command(name = "live-room-info")]
    LiveRoomInfo { principal_id: String },
}
