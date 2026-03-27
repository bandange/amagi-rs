#![allow(missing_docs)]

use clap::Subcommand;

use crate::platforms::twitter::{TwitterTweetRepliesSortMode, TwitterTweetSearchMode};

/// Twitter/X tasks exposed through the CLI.
#[derive(Debug, Subcommand, Clone)]
pub enum TwitterCommand {
    #[command(name = "search-tweets")]
    SearchTweets {
        query: String,
        #[arg(long, value_enum)]
        search_type: Option<TwitterTweetSearchMode>,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-profile")]
    UserProfile { screen_name: String },
    #[command(name = "user-timeline")]
    UserTimeline {
        screen_name: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-replies")]
    UserReplies {
        screen_name: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-media")]
    UserMedia {
        screen_name: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-followers")]
    UserFollowers {
        screen_name: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-following")]
    UserFollowing {
        screen_name: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-likes")]
    UserLikes {
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-bookmarks")]
    UserBookmarks {
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-followed")]
    UserFollowed {
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "user-recommended")]
    UserRecommended {
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "search-users")]
    SearchUsers {
        query: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "tweet-detail")]
    TweetDetail { tweet_id: String },
    #[command(name = "tweet-replies")]
    TweetReplies {
        tweet_id: String,
        #[arg(long)]
        cursor: Option<String>,
        #[arg(long, value_enum)]
        sort_by: Option<TwitterTweetRepliesSortMode>,
    },
    #[command(name = "tweet-likers")]
    TweetLikers {
        tweet_id: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "tweet-retweeters")]
    TweetRetweeters {
        tweet_id: String,
        #[arg(long)]
        count: Option<u32>,
        #[arg(long)]
        cursor: Option<String>,
    },
    #[command(name = "space-detail")]
    SpaceDetail { space_id: String },
}
