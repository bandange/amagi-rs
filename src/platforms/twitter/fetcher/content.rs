use std::collections::BTreeSet;

use reqwest::Url;
use serde_json::Value;

use crate::error::AppError;

use super::TwitterFetcher;
use super::transport::{
    array_at_path, normalize_optional_string, normalize_upstream_payload, string_at_path,
    twitter_datetime_to_rfc3339, u64_at_path, unwrap_tweet_result, unwrap_user_result,
    value_at_path,
};
use super::user::parse_user_summary;
use crate::platforms::twitter::{
    TwitterHashtagEntity, TwitterMediaEntity, TwitterSymbolEntity, TwitterTimestampEntity,
    TwitterTweet, TwitterTweetEntities, TwitterTweetPage, TwitterTweetRepliesSortMode,
    TwitterTweetSearchMode, TwitterTweetSearchPage, TwitterUrlEntity, TwitterUserMentionEntity,
    TwitterUserPage, TwitterUserProfile, TwitterUserTimeline,
};

impl TwitterFetcher {
    /// Search tweets through the Twitter/X web search timeline.
    #[doc(alias = "searchTweets")]
    pub async fn search_tweets(
        &self,
        query: &str,
        search_type: Option<TwitterTweetSearchMode>,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterTweetSearchPage, AppError> {
        self.ensure_authenticated_session("search-tweets")?;
        let search_type = search_type.unwrap_or_default();
        let value = self
            .fetch_graphql_value(
                &self.api_urls.search_tweets(
                    query,
                    search_type.graphql_product(),
                    count,
                    cursor,
                )?,
                Some(&search_referer(
                    self.web_base_url.as_ref(),
                    query,
                    search_type,
                )?),
            )
            .await?;

        parse_tweet_search_response(
            &value,
            query,
            search_type,
            self.web_base_url.as_ref(),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch one tweet detail by tweet id.
    #[doc(alias = "fetchTweetDetail")]
    pub async fn fetch_tweet_detail(&self, tweet_id: &str) -> Result<TwitterTweet, AppError> {
        let value = self
            .fetch_graphql_value(
                &self.api_urls.tweet_detail(tweet_id)?,
                Some(&format!(
                    "{}/i/status/{tweet_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;
        let tweet = value_at_path(&value, &["data", "tweetResult", "result"])
            .and_then(unwrap_tweet_result)
            .ok_or_else(|| AppError::UpstreamResponse {
                status: None,
                message: format!("twitter tweet `{tweet_id}` was not found"),
            })?;

        parse_tweet(tweet, self.web_base_url.as_ref())
    }

    /// Fetch one page of replies for a tweet id.
    #[doc(alias = "fetchTweetReplies")]
    pub async fn fetch_tweet_replies(
        &self,
        tweet_id: &str,
        cursor: Option<&str>,
        sort_by: Option<TwitterTweetRepliesSortMode>,
    ) -> Result<TwitterTweetPage, AppError> {
        let ranking_mode = sort_by.unwrap_or_default().graphql_ranking_mode();
        let value = self
            .fetch_graphql_value(
                &self
                    .api_urls
                    .tweet_replies(tweet_id, cursor, ranking_mode)?,
                Some(&format!(
                    "{}/i/status/{tweet_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;

        parse_tweet_page_response(
            &value,
            &[
                "data",
                "threaded_conversation_with_injections_v2",
                "instructions",
            ],
            &format!("twitter tweet replies for `{tweet_id}`"),
            self.web_base_url.as_ref(),
            usize::MAX,
        )
    }

    /// Fetch one page of likers for a tweet id.
    #[doc(alias = "fetchTweetLikers")]
    pub async fn fetch_tweet_likers(
        &self,
        tweet_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserPage, AppError> {
        self.ensure_authenticated_session("tweet-likers")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.tweet_likers(tweet_id, count, cursor)?,
                Some(&format!(
                    "{}/i/status/{tweet_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;

        super::user::parse_user_page_response(
            &value,
            &["data", "favoriters_timeline", "timeline", "instructions"],
            &format!("twitter tweet likers for `{tweet_id}`"),
            count.unwrap_or(20) as usize,
        )
    }

    /// Fetch one page of retweeters for a tweet id.
    #[doc(alias = "fetchTweetRetweeters")]
    pub async fn fetch_tweet_retweeters(
        &self,
        tweet_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<TwitterUserPage, AppError> {
        self.ensure_authenticated_session("tweet-retweeters")?;
        let value = self
            .fetch_graphql_value(
                &self.api_urls.tweet_retweeters(tweet_id, count, cursor)?,
                Some(&format!(
                    "{}/i/status/{tweet_id}",
                    self.web_base_url.trim_end_matches('/')
                )),
            )
            .await?;

        super::user::parse_user_page_response(
            &value,
            &["data", "retweeters_timeline", "timeline", "instructions"],
            &format!("twitter tweet retweeters for `{tweet_id}`"),
            count.unwrap_or(20) as usize,
        )
    }
}

pub(super) fn parse_user_timeline_response(
    value: &Value,
    user: TwitterUserProfile,
    web_base_url: &str,
    limit: usize,
) -> Result<TwitterUserTimeline, AppError> {
    let page = parse_tweet_page_response(
        value,
        &[
            "data",
            "user",
            "result",
            "timeline",
            "timeline",
            "instructions",
        ],
        &format!("twitter timeline for `{}`", user.screen_name),
        web_base_url,
        limit,
    )?;

    Ok(TwitterUserTimeline {
        user,
        tweets: page.tweets,
        previous_cursor: page.previous_cursor,
        next_cursor: page.next_cursor,
        upstream_payload: page.upstream_payload,
    })
}

fn parse_tweet_search_response(
    value: &Value,
    query: &str,
    search_type: TwitterTweetSearchMode,
    web_base_url: &str,
    limit: usize,
) -> Result<TwitterTweetSearchPage, AppError> {
    let page = parse_tweet_page_response(
        value,
        &[
            "data",
            "search_by_raw_query",
            "search_timeline",
            "timeline",
            "instructions",
        ],
        &format!("twitter search timeline for `{query}`"),
        web_base_url,
        limit,
    )?;

    Ok(TwitterTweetSearchPage {
        query: query.to_owned(),
        search_type,
        tweets: page.tweets,
        previous_cursor: page.previous_cursor,
        next_cursor: page.next_cursor,
        upstream_payload: page.upstream_payload,
    })
}

pub(super) fn parse_tweet(tweet: &Value, web_base_url: &str) -> Result<TwitterTweet, AppError> {
    let tweet = unwrap_tweet_result(tweet).ok_or_else(|| AppError::UpstreamResponse {
        status: None,
        message: "twitter tweet result is not a tweet payload".into(),
    })?;
    let id = string_at_path(tweet, &["rest_id"]).ok_or_else(|| AppError::UpstreamResponse {
        status: None,
        message: "twitter tweet response is missing `rest_id`".into(),
    })?;
    let author = value_at_path(tweet, &["core", "user_results", "result"])
        .and_then(unwrap_user_result)
        .ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("twitter tweet `{id}` is missing author data"),
        })
        .and_then(parse_user_summary)?;
    let full_text = string_at_path(
        tweet,
        &["note_tweet", "note_tweet_results", "result", "text"],
    )
    .or_else(|| string_at_path(tweet, &["legacy", "full_text"]))
    .unwrap_or_default();

    Ok(TwitterTweet {
        id: id.clone(),
        conversation_id: string_at_path(tweet, &["legacy", "conversation_id_str"]),
        author: author.clone(),
        url: format!(
            "{}/{}/status/{id}",
            web_base_url.trim_end_matches('/'),
            author.screen_name
        ),
        created_at: twitter_datetime_to_rfc3339(
            string_at_path(tweet, &["legacy", "created_at"]).as_deref(),
        ),
        full_text,
        language: string_at_path(tweet, &["legacy", "lang"]),
        source: string_at_path(tweet, &["source"]),
        reply_to_tweet_id: string_at_path(tweet, &["legacy", "in_reply_to_status_id_str"]),
        quoted_tweet: value_at_path(tweet, &["quoted_status_result", "result"])
            .and_then(unwrap_tweet_result)
            .map(|value| parse_tweet(value, web_base_url))
            .transpose()?
            .map(Box::new),
        retweeted_tweet: value_at_path(tweet, &["legacy", "retweeted_status_result", "result"])
            .and_then(unwrap_tweet_result)
            .map(|value| parse_tweet(value, web_base_url))
            .transpose()?
            .map(Box::new),
        upstream_payload: normalize_upstream_payload(tweet),
        entities: parse_tweet_entities(tweet),
        media: parse_media_entities(tweet),
        reply_count: u64_at_path(tweet, &["legacy", "reply_count"]).unwrap_or_default(),
        retweet_count: u64_at_path(tweet, &["legacy", "retweet_count"]).unwrap_or_default(),
        quote_count: u64_at_path(tweet, &["legacy", "quote_count"]).unwrap_or_default(),
        favorite_count: u64_at_path(tweet, &["legacy", "favorite_count"]).unwrap_or_default(),
        bookmark_count: u64_at_path(tweet, &["legacy", "bookmark_count"]),
        view_count: u64_at_path(tweet, &["views", "count"]),
    })
}

pub(super) fn parse_tweet_page_response(
    value: &Value,
    instructions_path: &[&str],
    context: &str,
    web_base_url: &str,
    limit: usize,
) -> Result<TwitterTweetPage, AppError> {
    let instructions =
        array_at_path(value, instructions_path).ok_or_else(|| AppError::UpstreamResponse {
            status: None,
            message: format!("{context} is missing instructions"),
        })?;
    let page = parse_timeline_entries(instructions, web_base_url, limit)?;

    Ok(TwitterTweetPage {
        tweets: page.tweets,
        previous_cursor: page.previous_cursor,
        next_cursor: page.next_cursor,
        upstream_payload: normalize_upstream_payload(value),
    })
}

#[derive(Debug, Default)]
struct ParsedTweetPage {
    tweets: Vec<TwitterTweet>,
    previous_cursor: Option<String>,
    next_cursor: Option<String>,
}

fn parse_timeline_entries(
    instructions: &[Value],
    web_base_url: &str,
    limit: usize,
) -> Result<ParsedTweetPage, AppError> {
    let mut tweets = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous_cursor = None;
    let mut next_cursor = None;

    for instruction in instructions {
        match instruction.get("type").and_then(Value::as_str) {
            Some("TimelinePinEntry") => {
                if let Some(entry) = instruction.get("entry") {
                    collect_entry(entry, web_base_url, &mut seen, &mut tweets)?;
                    collect_cursor(entry, &mut previous_cursor, &mut next_cursor);
                }
            }
            Some("TimelineAddEntries") => {
                if let Some(entries) = instruction.get("entries").and_then(Value::as_array) {
                    for entry in entries {
                        collect_entry(entry, web_base_url, &mut seen, &mut tweets)?;
                        collect_cursor(entry, &mut previous_cursor, &mut next_cursor);
                    }
                }
            }
            Some("TimelineAddToModule") => {
                if let Some(items) = instruction.get("moduleItems").and_then(Value::as_array) {
                    for item in items {
                        collect_module_item(item, web_base_url, &mut seen, &mut tweets)?;
                    }
                }
            }
            _ => {}
        }
    }

    if tweets.len() > limit {
        tweets.truncate(limit);
    }

    Ok(ParsedTweetPage {
        tweets,
        previous_cursor,
        next_cursor,
    })
}

fn search_referer(
    web_base_url: &str,
    query: &str,
    search_type: TwitterTweetSearchMode,
) -> Result<String, AppError> {
    let mut url =
        Url::parse(&format!("{}/search", web_base_url.trim_end_matches('/'))).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid twitter search referer base url: {error}"
            ))
        })?;
    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("q", query);
        pairs.append_pair("src", "typed_query");
        if matches!(search_type, TwitterTweetSearchMode::Latest) {
            pairs.append_pair("f", "live");
        }
    }

    Ok(url.to_string())
}

fn collect_entry(
    entry: &Value,
    web_base_url: &str,
    seen: &mut BTreeSet<String>,
    tweets: &mut Vec<TwitterTweet>,
) -> Result<(), AppError> {
    let Some(content) = entry.get("content") else {
        return Ok(());
    };

    if let Some(tweet_result) = value_at_path(content, &["itemContent", "tweet_results", "result"])
        .and_then(unwrap_tweet_result)
    {
        push_tweet(tweet_result, web_base_url, seen, tweets)?;
        return Ok(());
    }

    if let Some(items) = content.get("items").and_then(Value::as_array) {
        for item in items {
            collect_module_item(item, web_base_url, seen, tweets)?;
        }
    }

    Ok(())
}

fn collect_module_item(
    item: &Value,
    web_base_url: &str,
    seen: &mut BTreeSet<String>,
    tweets: &mut Vec<TwitterTweet>,
) -> Result<(), AppError> {
    if let Some(tweet_result) =
        value_at_path(item, &["item", "itemContent", "tweet_results", "result"])
            .and_then(unwrap_tweet_result)
            .or_else(|| {
                value_at_path(item, &["itemContent", "tweet_results", "result"])
                    .and_then(unwrap_tweet_result)
            })
    {
        push_tweet(tweet_result, web_base_url, seen, tweets)?;
    }

    Ok(())
}

fn collect_cursor(
    entry: &Value,
    previous_cursor: &mut Option<String>,
    next_cursor: &mut Option<String>,
) {
    let Some(content) = entry.get("content") else {
        return;
    };

    let cursor_type = content.get("cursorType").and_then(Value::as_str);
    let cursor_value = content
        .get("value")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .and_then(normalize_optional_string);

    match cursor_type {
        Some("Top") if previous_cursor.is_none() => *previous_cursor = cursor_value,
        Some("Bottom") if next_cursor.is_none() => *next_cursor = cursor_value,
        _ => {}
    }
}

fn push_tweet(
    tweet: &Value,
    web_base_url: &str,
    seen: &mut BTreeSet<String>,
    tweets: &mut Vec<TwitterTweet>,
) -> Result<(), AppError> {
    let id = string_at_path(tweet, &["rest_id"]).ok_or_else(|| AppError::UpstreamResponse {
        status: None,
        message: "twitter timeline tweet is missing `rest_id`".into(),
    })?;

    if seen.insert(id) {
        tweets.push(parse_tweet(tweet, web_base_url)?);
    }

    Ok(())
}

fn parse_media_entities(tweet: &Value) -> Vec<TwitterMediaEntity> {
    value_at_path(tweet, &["legacy", "extended_entities", "media"])
        .and_then(Value::as_array)
        .or_else(|| {
            value_at_path(tweet, &["legacy", "entities", "media"]).and_then(Value::as_array)
        })
        .map(|items| {
            items
                .iter()
                .map(|item| TwitterMediaEntity {
                    media_type: string_at_path(item, &["type"]).unwrap_or_else(|| "unknown".into()),
                    media_url: string_at_path(item, &["media_url_https"])
                        .or_else(|| string_at_path(item, &["media_url"])),
                    preview_image_url: string_at_path(item, &["media_url_https"])
                        .or_else(|| string_at_path(item, &["media_url"])),
                    expanded_url: string_at_path(item, &["expanded_url"]),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_tweet_entities(tweet: &Value) -> TwitterTweetEntities {
    let root = value_at_path(
        tweet,
        &["note_tweet", "note_tweet_results", "result", "entity_set"],
    )
    .or_else(|| value_at_path(tweet, &["legacy", "entities"]));

    let Some(root) = root else {
        return TwitterTweetEntities::default();
    };

    TwitterTweetEntities {
        urls: value_at_path(root, &["urls"])
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let entity = TwitterUrlEntity {
                            url: string_at_path(item, &["url"]),
                            expanded_url: string_at_path(item, &["expanded_url"]),
                            display_url: string_at_path(item, &["display_url"]),
                            unwound_url: string_at_path(item, &["unwound_url"]),
                            indices: parse_indices(item),
                        };

                        (entity.url.is_some()
                            || entity.expanded_url.is_some()
                            || entity.display_url.is_some()
                            || entity.unwound_url.is_some()
                            || entity.indices.is_some())
                        .then_some(entity)
                    })
                    .collect()
            })
            .unwrap_or_default(),
        user_mentions: value_at_path(root, &["user_mentions"])
            .and_then(Value::as_array)
            .or_else(|| value_at_path(root, &["mentions"]).and_then(Value::as_array))
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let entity = TwitterUserMentionEntity {
                            id: string_at_path(item, &["id_str"])
                                .or_else(|| string_at_path(item, &["id"])),
                            screen_name: string_at_path(item, &["screen_name"]),
                            name: string_at_path(item, &["name"]),
                            indices: parse_indices(item),
                        };

                        (entity.id.is_some()
                            || entity.screen_name.is_some()
                            || entity.name.is_some()
                            || entity.indices.is_some())
                        .then_some(entity)
                    })
                    .collect()
            })
            .unwrap_or_default(),
        hashtags: value_at_path(root, &["hashtags"])
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let text = string_at_path(item, &["text"])?;
                        Some(TwitterHashtagEntity {
                            text,
                            indices: parse_indices(item),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        symbols: value_at_path(root, &["symbols"])
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let text = string_at_path(item, &["text"])?;
                        Some(TwitterSymbolEntity {
                            text,
                            indices: parse_indices(item),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
        timestamps: value_at_path(root, &["timestamps"])
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let text = string_at_path(item, &["text"])?;
                        Some(TwitterTimestampEntity {
                            text,
                            indices: parse_indices(item),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default(),
    }
}

fn parse_indices(item: &Value) -> Option<[u32; 2]> {
    let indices = value_at_path(item, &["indices"])?.as_array()?;
    if indices.len() != 2 {
        return None;
    }

    Some([value_to_u32(&indices[0])?, value_to_u32(&indices[1])?])
}

fn value_to_u32(value: &Value) -> Option<u32> {
    match value {
        Value::Number(number) => number.as_u64().and_then(|value| u32::try_from(value).ok()),
        Value::String(text) => text.parse::<u32>().ok(),
        _ => None,
    }
}
