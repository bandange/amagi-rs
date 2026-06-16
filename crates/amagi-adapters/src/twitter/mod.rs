//! Rust-native Twitter/X fetchers built on the public web protocol.

mod api;
mod auth;
mod bound;
mod fetcher;
mod sign;
mod types;

pub use api::{TwitterApiUrls, create_twitter_api_urls};
pub use bound::{BoundTwitterFetcher, create_bound_twitter_fetcher};
pub use fetcher::TwitterFetcher;
pub use types::{
    TwitterHashtagEntity, TwitterMediaEntity, TwitterSpaceDetail, TwitterSymbolEntity,
    TwitterTimestampEntity, TwitterTweet, TwitterTweetEntities, TwitterTweetPage,
    TwitterTweetRepliesSortMode, TwitterTweetSearchMode, TwitterTweetSearchPage, TwitterUrlEntity,
    TwitterUserListPage, TwitterUserMentionEntity, TwitterUserPage, TwitterUserProfile,
    TwitterUserSummary, TwitterUserTimeline,
};
