use std::collections::BTreeSet;

use reqwest::Url;
use serde_json::Value;

use crate::error::AppError;
use crate::platforms::twitter::auth::extract_twid_user_id;

use super::TwitterFetcher;
use super::transport::{
    array_at_path, bool_at_path, normalize_optional_string, normalize_upstream_payload,
    string_at_path, twitter_datetime_to_rfc3339, u64_at_path, unwrap_user_result, value_at_path,
};
use crate::platforms::twitter::{
    TwitterTweetPage, TwitterUserListPage, TwitterUserPage, TwitterUserProfile, TwitterUserSummary,
    TwitterUserTimeline,
};

impl TwitterFetcher {
    /// Fetch a Twitter/X user profile by screen name.
    #[doc(alias = "fetchUserProfile")]
    pub async fn fetch_user_profile(
        &self,
        screen_name: &str,
    ) -> Result<TwitterUserProfile, AppError> {
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_profile(screen_name)?,
                Some(&format!(
                    "{}/{}",
                    self.web_base_url.trim_end_matches('/'),
                    screen_name
                )),
            )
            .await?;
        let user = value_at_path(&value, &["data", "user", "result"])
            .and_then(unwrap_user_result)
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter user `{screen_name}` was not found"),
            })?;

        parse_user_profile(user)
    }

    /// Fetch a Twitter/X user profile by numeric user rest id.
    #[doc(alias = "fetchUserProfileById")]
    pub async fn fetch_user_profile_by_id(
        &self,
        user_id: &str,
    ) -> Result<TwitterUserProfile, AppError> {
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_profile_by_id(user_id)?,
                Some(&format!(
                    "{}/i/user/{user_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;
        let user = value_at_path(&value, &["data", "user", "result"])
            .and_then(unwrap_user_result)
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter user id `{user_id}` was not found"),
            })?;

        parse_user_profile(user)
    }

    /// Fetch a Twitter/X user's public timeline by screen name.
    #[doc(alias = "fetchUserTimeline")]
    pub async fn fetch_user_timeline(
        &self,
        screen_name: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserTimeline, AppError> {
        let profile = self.fetch_user_profile(screen_name).await?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_timeline(&profile.id, count, cursor)?,
                Some(&format!(
                    "{}/{}",
                    self.web_base_url.trim_end_matches('/'),
                    screen_name
                )),
            )
            .await?;
        super::content::parse_user_timeline_response(
            &value,
            profile,
            self.web_base_url.as_ref(),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch a Twitter/X user's public replies timeline by screen name.
    #[doc(alias = "fetchUserReplies")]
    pub async fn fetch_user_replies(
        &self,
        screen_name: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserTimeline, AppError> {
        self.ensure_authenticated_session("user-replies")?;
        let profile = self.fetch_user_profile(screen_name).await?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_replies(&profile.id, count, cursor)?,
                Some(&format!(
                    "{}/with_replies",
                    user_referer(self.web_base_url.as_ref(), screen_name)
                )),
            )
            .await?;
        super::content::parse_user_timeline_response(
            &value,
            profile,
            self.web_base_url.as_ref(),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch a Twitter/X user's media timeline by screen name.
    #[doc(alias = "fetchUserMedia")]
    pub async fn fetch_user_media(
        &self,
        screen_name: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserTimeline, AppError> {
        let requested_limit = count.unwrap_or(20) as usize;
        let profile = self.fetch_user_profile(screen_name).await?;
        if requested_limit == 0 {
            return Ok(TwitterUserTimeline {
                user: profile,
                tweets: Vec::new(),
                previous_cursor: None,
                next_cursor: None,
                upstream_payload: Value::Null,
            });
        }

        let referer = format!(
            "{}/media",
            user_referer(self.web_base_url.as_ref(), screen_name)
        );
        let context = format!("twitter media timeline for `{screen_name}`");
        let mut tweets = Vec::new();
        let mut seen = BTreeSet::new();
        let mut previous_cursor = None;
        let mut request_cursor = cursor.map(str::to_owned);
        let mut upstream_pages = Vec::new();

        let next_cursor = loop {
            let value = self
                .fetch_graphql_value(
                    &self
                        .api_urls
                        .user_media(&profile.id, count, request_cursor.as_deref())?,
                    Some(&referer),
                )
                .await?;
            let page = super::content::parse_tweet_page_response(
                &value,
                &[
                    "data",
                    "user",
                    "result",
                    "timeline",
                    "timeline",
                    "instructions",
                ],
                &context,
                self.web_base_url.as_ref(),
                usize::MAX,
            )?;
            upstream_pages.push(page.upstream_payload.clone());

            if previous_cursor.is_none() {
                previous_cursor = page.previous_cursor.clone();
            }
            let page_next_cursor = page.next_cursor.clone();
            extend_unique_tweets(&mut tweets, &mut seen, page.tweets, requested_limit);

            if tweets.len() >= requested_limit {
                break page_next_cursor;
            }

            let Some(cursor) = page_next_cursor.clone() else {
                break None;
            };
            if request_cursor.as_deref() == Some(cursor.as_str()) {
                break page_next_cursor;
            }
            request_cursor = Some(cursor);
        };

        Ok(TwitterUserTimeline {
            user: profile,
            tweets,
            previous_cursor,
            next_cursor,
            upstream_payload: collapse_upstream_pages(upstream_pages),
        })
    }

    /// Fetch a Twitter/X user's followers by screen name.
    #[doc(alias = "fetchUserFollowers")]
    pub async fn fetch_user_followers(
        &self,
        screen_name: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserListPage, AppError> {
        self.ensure_authenticated_session("user-followers")?;
        let profile = self.fetch_user_profile(screen_name).await?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_followers(&profile.id, count, cursor)?,
                Some(&format!(
                    "{}/followers",
                    user_referer(self.web_base_url.as_ref(), screen_name)
                )),
            )
            .await?;
        parse_user_list_page_response(&value, profile, count.unwrap_or(20) as usize)
    }

    /// Fetch a Twitter/X user's followings by screen name.
    #[doc(alias = "fetchUserFollowing")]
    pub async fn fetch_user_following(
        &self,
        screen_name: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserListPage, AppError> {
        self.ensure_authenticated_session("user-following")?;
        let profile = self.fetch_user_profile(screen_name).await?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_following(&profile.id, count, cursor)?,
                Some(&format!(
                    "{}/following",
                    user_referer(self.web_base_url.as_ref(), screen_name)
                )),
            )
            .await?;
        parse_user_list_page_response(&value, profile, count.unwrap_or(20) as usize)
    }

    /// Fetch one page of liked tweets for the authenticated Twitter/X account.
    #[doc(alias = "fetchUserLikes")]
    pub async fn fetch_user_likes(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterTweetPage, AppError> {
        self.ensure_authenticated_session("user-likes")?;
        let user_id = self.authenticated_user_id("user-likes")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_likes(&user_id, count, cursor)?,
                Some(&format!("{}/home", self.web_base_url.trim_end_matches('/'))),
            )
            .await?;

        parse_user_likes_page_response(
            &value,
            "authenticated account",
            self.web_base_url.as_ref(),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch one page of authenticated user bookmarks.
    #[doc(alias = "fetchUserBookmarks")]
    pub async fn fetch_user_bookmarks(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterTweetPage, AppError> {
        self.ensure_authenticated_session("user-bookmarks")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_bookmarks(count, cursor)?,
                Some(&format!(
                    "{}/i/bookmarks",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;

        super::content::parse_tweet_page_response(
            &value,
            &["data", "bookmark_timeline_v2", "timeline", "instructions"],
            "twitter bookmarks timeline",
            self.web_base_url.as_ref(),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch one page of authenticated followed feed items.
    #[doc(alias = "fetchUserFollowed")]
    pub async fn fetch_user_followed(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterTweetPage, AppError> {
        self.ensure_authenticated_session("user-followed")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_followed(count, cursor)?,
                Some(&format!("{}/home", self.web_base_url.trim_end_matches('/'))),
            )
            .await?;

        super::content::parse_tweet_page_response(
            &value,
            &["data", "home", "home_timeline_urt", "instructions"],
            "twitter followed feed",
            self.web_base_url.as_ref(),
            count.unwrap_or(35) as usize,
        )
    }

    /// Fetch one page of authenticated recommended feed items.
    #[doc(alias = "fetchUserRecommended")]
    pub async fn fetch_user_recommended(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterTweetPage, AppError> {
        self.ensure_authenticated_session("user-recommended")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.user_recommended(count, cursor)?,
                Some(&format!("{}/home", self.web_base_url.trim_end_matches('/'))),
            )
            .await?;

        super::content::parse_tweet_page_response(
            &value,
            &["data", "home", "home_timeline_urt", "instructions"],
            "twitter recommended feed",
            self.web_base_url.as_ref(),
            count.unwrap_or(35) as usize,
        )
    }

    /// Search Twitter/X users by query text.
    #[doc(alias = "searchUsers")]
    pub async fn search_users(
        &self,
        query: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserPage, AppError> {
        self.ensure_authenticated_session("search-users")?;
        let mut referer = Url::parse(&format!(
            "{}/search",
            self.web_base_url.trim_end_matches('/')
        ))
        .map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid twitter user search referer base url: {error}"
            ))
        })?;
        {
            let mut pairs = referer.query_pairs_mut();
            pairs.append_pair("q", query);
            pairs.append_pair("src", "typed_query");
        }
        let value = self
            .fetch_graphql_value(
                &self.api_urls.search_users(query, count, cursor)?,
                Some(referer.as_ref()),
            )
            .await?;

        parse_user_page_response(
            &value,
            &[
                "data",
                "search_by_raw_query",
                "search_timeline",
                "timeline",
                "instructions",
            ],
            &format!("twitter user search for `{query}`"),
            count.unwrap_or(20) as usize,
        )
    }
}

pub(super) fn parse_user_profile(user: &serde_json::Value) -> Result<TwitterUserProfile, AppError> {
    let id = string_at_path(user, &["rest_id"]).ok_or_else(|| AppError::UpstreamResponse {
        status: None,
        message: "twitter user response is missing `rest_id`".into(),
    })?;
    let screen_name = string_at_path(user, &["core", "screen_name"])
        .or_else(|| string_at_path(user, &["legacy", "screen_name"]))
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: "twitter user response is missing `screen_name`".into(),
        })?;
    let name = string_at_path(user, &["core", "name"])
        .or_else(|| string_at_path(user, &["legacy", "name"]))
        .unwrap_or_else(|| screen_name.clone());

    Ok(TwitterUserProfile {
        id,
        screen_name,
        name,
        created_at: twitter_datetime_to_rfc3339(
            string_at_path(user, &["core", "created_at"])
                .or_else(|| string_at_path(user, &["legacy", "created_at"]))
                .as_deref(),
        ),
        description: string_at_path(user, &["legacy", "description"]),
        location: string_at_path(user, &["location", "location"])
            .or_else(|| string_at_path(user, &["legacy", "location"])),
        avatar_url: string_at_path(user, &["avatar", "image_url"])
            .or_else(|| string_at_path(user, &["legacy", "profile_image_url_https"])),
        banner_url: string_at_path(user, &["legacy", "profile_banner_url"]),
        verified: bool_at_path(user, &["is_blue_verified"])
            .or_else(|| bool_at_path(user, &["verification", "verified"]))
            .unwrap_or(false),
        protected: bool_at_path(user, &["privacy", "protected"]).unwrap_or(false),
        followers_count: u64_at_path(user, &["legacy", "followers_count"]).unwrap_or_default(),
        following_count: u64_at_path(user, &["legacy", "friends_count"]).unwrap_or_default(),
        statuses_count: u64_at_path(user, &["legacy", "statuses_count"]).unwrap_or_default(),
        media_count: u64_at_path(user, &["legacy", "media_count"]).unwrap_or_default(),
        favourites_count: u64_at_path(user, &["legacy", "favourites_count"]).unwrap_or_default(),
        pinned_tweet_id: value_at_path(user, &["legacy", "pinned_tweet_ids_str"])
            .and_then(serde_json::Value::as_array)
            .and_then(|items| items.first())
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .and_then(normalize_optional_string),
        upstream_payload: normalize_upstream_payload(user),
    })
}

pub(super) fn parse_user_summary(user: &serde_json::Value) -> Result<TwitterUserSummary, AppError> {
    let profile = parse_user_profile(user)?;

    Ok(TwitterUserSummary {
        id: profile.id,
        screen_name: profile.screen_name,
        name: profile.name,
        avatar_url: profile.avatar_url,
        verified: profile.verified,
        upstream_payload: profile.upstream_payload,
    })
}

fn parse_user_list_page_response(
    value: &serde_json::Value,
    user: TwitterUserProfile,
    limit: usize,
) -> Result<TwitterUserListPage, AppError> {
    let page = parse_user_page_response(
        value,
        &[
            "data",
            "user",
            "result",
            "timeline",
            "timeline",
            "instructions",
        ],
        &format!("twitter user page for `{}`", user.screen_name),
        limit,
    )?;

    Ok(TwitterUserListPage {
        user,
        users: page.users,
        previous_cursor: page.previous_cursor,
        next_cursor: page.next_cursor,
        upstream_payload: page.upstream_payload,
    })
}

pub(super) fn parse_user_page_response(
    value: &serde_json::Value,
    instructions_path: &[&str],
    context: &str,
    limit: usize,
) -> Result<TwitterUserPage, AppError> {
    let instructions =
        array_at_path(value, instructions_path).ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("{context} is missing instructions"),
        })?;

    let mut users = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous_cursor = None;
    let mut next_cursor = None;

    for instruction in instructions {
        match instruction.get("type").and_then(serde_json::Value::as_str) {
            Some("TimelinePinEntry") => {
                if let Some(entry) = instruction.get("entry") {
                    collect_users_from_entry(entry, &mut seen, &mut users)?;
                    collect_cursor(entry, &mut previous_cursor, &mut next_cursor);
                }
            }
            Some("TimelineAddEntries") => {
                if let Some(entries) = instruction
                    .get("entries")
                    .and_then(serde_json::Value::as_array)
                {
                    for entry in entries {
                        collect_users_from_entry(entry, &mut seen, &mut users)?;
                        collect_cursor(entry, &mut previous_cursor, &mut next_cursor);
                    }
                }
            }
            _ => {}
        }
    }

    if users.len() > limit {
        users.truncate(limit);
    }

    Ok(TwitterUserPage {
        users,
        previous_cursor,
        next_cursor,
        upstream_payload: normalize_upstream_payload(value),
    })
}

fn collect_users_from_entry(
    entry: &serde_json::Value,
    seen: &mut BTreeSet<String>,
    users: &mut Vec<TwitterUserProfile>,
) -> Result<(), AppError> {
    if let Some(timeline_user) = find_timeline_user(entry) {
        let profile = parse_user_profile(timeline_user)?;
        if seen.insert(profile.id.clone()) {
            users.push(profile);
        }
    }

    Ok(())
}

fn find_timeline_user<'a>(value: &'a serde_json::Value) -> Option<&'a serde_json::Value> {
    match value {
        serde_json::Value::Object(map) => {
            let timeline_user_type = value
                .get("__typename")
                .or_else(|| value.get("itemType"))
                .and_then(serde_json::Value::as_str);
            if timeline_user_type == Some("TimelineUser") {
                return value_at_path(value, &["user_results", "result"])
                    .and_then(unwrap_user_result);
            }

            for child in map.values() {
                if let Some(found) = find_timeline_user(child) {
                    return Some(found);
                }
            }

            None
        }
        serde_json::Value::Array(items) => {
            for item in items {
                if let Some(found) = find_timeline_user(item) {
                    return Some(found);
                }
            }

            None
        }
        _ => None,
    }
}

fn collect_cursor(
    entry: &serde_json::Value,
    previous_cursor: &mut Option<String>,
    next_cursor: &mut Option<String>,
) {
    let Some(content) = entry.get("content") else {
        return;
    };

    let cursor_type = content
        .get("cursorType")
        .and_then(serde_json::Value::as_str);
    let cursor_value = content
        .get("value")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .and_then(normalize_optional_string);

    match cursor_type {
        Some("Top") if previous_cursor.is_none() => *previous_cursor = cursor_value,
        Some("Bottom") if next_cursor.is_none() => *next_cursor = cursor_value,
        _ => {}
    }
}

fn user_referer(web_base_url: &str, screen_name: &str) -> String {
    format!("{}/{}", web_base_url.trim_end_matches('/'), screen_name)
}

fn extend_unique_tweets(
    tweets: &mut Vec<crate::platforms::twitter::TwitterTweet>,
    seen: &mut BTreeSet<String>,
    page_tweets: Vec<crate::platforms::twitter::TwitterTweet>,
    limit: usize,
) {
    for tweet in page_tweets {
        if seen.insert(tweet.id.clone()) {
            tweets.push(tweet);
            if tweets.len() >= limit {
                break;
            }
        }
    }
}

fn collapse_upstream_pages(mut pages: Vec<Value>) -> Value {
    match pages.len() {
        0 => Value::Null,
        1 => pages.pop().unwrap_or(Value::Null),
        _ => Value::Array(pages),
    }
}

impl TwitterFetcher {
    fn authenticated_user_id(&self, capability: &str) -> Result<String, AppError> {
        let Some(cookie) = self.cookie_header() else {
            return Err(AppError::InvalidRequestConfig(format!(
                "twitter capability `{capability}` requires `AMAGI_TWITTER_COOKIE` with at least `auth_token`, `ct0`, and `twid`"
            )));
        };

        extract_twid_user_id(cookie).ok_or_else(|| {
            AppError::InvalidRequestConfig(format!(
                "twitter capability `{capability}` requires `AMAGI_TWITTER_COOKIE` with at least `auth_token`, `ct0`, and `twid`"
            ))
        })
    }
}

fn parse_user_likes_page_response(
    value: &serde_json::Value,
    screen_name: &str,
    web_base_url: &str,
    limit: usize,
) -> Result<TwitterTweetPage, AppError> {
    const LIKES_TIMELINE_PATHS: &[&[&str]] = &[
        &[
            "data",
            "user",
            "result",
            "timeline_v2",
            "timeline",
            "instructions",
        ],
        &[
            "data",
            "user",
            "result",
            "timeline",
            "timeline",
            "instructions",
        ],
        &["data", "user", "result", "timeline", "instructions"],
    ];

    let context = format!("twitter likes timeline for `{screen_name}`");
    for path in LIKES_TIMELINE_PATHS {
        if array_at_path(value, path).is_some() {
            return super::content::parse_tweet_page_response(
                value,
                path,
                &context,
                web_base_url,
                limit,
            );
        }
    }

    if value_at_path(value, &["data", "user", "result", "timeline"])
        .and_then(serde_json::Value::as_object)
        .is_some_and(|timeline| timeline.is_empty())
    {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!(
                "twitter likes timeline for `{screen_name}` is not exposed by the current X web API response"
            ),
        });
    }

    super::content::parse_tweet_page_response(
        value,
        LIKES_TIMELINE_PATHS[0],
        &context,
        web_base_url,
        limit,
    )
}
