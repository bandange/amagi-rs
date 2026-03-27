/// Twitter/X tasks exposed by the CLI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TwitterRunTask {
    /// Search tweets by raw query.
    SearchTweets {
        /// Search keyword or advanced query string.
        query: String,
        /// Optional search result type.
        search_type: Option<crate::platforms::twitter::TwitterTweetSearchMode>,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one user profile.
    UserProfile {
        /// Target screen name.
        screen_name: String,
    },
    /// Fetch one page of a user timeline.
    UserTimeline {
        /// Target screen name.
        screen_name: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of a user's replies timeline.
    UserReplies {
        /// Target screen name.
        screen_name: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of a user's media timeline.
    UserMedia {
        /// Target screen name.
        screen_name: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of a user's followers.
    UserFollowers {
        /// Target screen name.
        screen_name: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of a user's following list.
    UserFollowing {
        /// Target screen name.
        screen_name: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of the authenticated user's liked tweets.
    UserLikes {
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of the authenticated user's bookmarks.
    UserBookmarks {
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of the authenticated user's followed feed.
    UserFollowed {
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of the authenticated user's recommended feed.
    UserRecommended {
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Search users by query text.
    SearchUsers {
        /// Search keyword or screen name fragment.
        query: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one tweet detail.
    TweetDetail {
        /// Target tweet id.
        tweet_id: String,
    },
    /// Fetch one page of replies to a tweet.
    TweetReplies {
        /// Target tweet id.
        tweet_id: String,
        /// Optional pagination cursor.
        cursor: Option<String>,
        /// Optional reply sorting mode.
        sort_by: Option<crate::platforms::twitter::TwitterTweetRepliesSortMode>,
    },
    /// Fetch one page of users who liked a tweet.
    TweetLikers {
        /// Target tweet id.
        tweet_id: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one page of users who retweeted a tweet.
    TweetRetweeters {
        /// Target tweet id.
        tweet_id: String,
        /// Optional page size.
        count: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<String>,
    },
    /// Fetch one Space detail.
    SpaceDetail {
        /// Target space id.
        space_id: String,
    },
}
