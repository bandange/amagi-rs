use std::borrow::Cow;

use reqwest::Url;
use serde_json::{Value, json};

use crate::error::AppError;

const TWITTER_GRAPHQL_BASE_URL: &str = "https://x.com/i/api/graphql";
const TWITTER_GUEST_ACTIVATE_URL: &str = "https://api.twitter.com/1.1/guest/activate.json";

const USER_BY_SCREEN_NAME_OPERATION_ID: &str = "IGgvgiOx4QZndDHuD3x9TQ";
const USER_BY_REST_ID_OPERATION_ID: &str = "VQfQ9wwYdk6j_u2O4vt64Q";
const USER_TWEETS_OPERATION_ID: &str = "FOlovQsiHGDls3c0Q_HaSQ";
const USER_TWEETS_AND_REPLIES_OPERATION_ID: &str = "EJTxTKSH-byy7X46AhtKeA";
const USER_MEDIA_OPERATION_ID: &str = "SjiAp7wyuCUBkKAJJObU8w";
const FOLLOWERS_OPERATION_ID: &str = "-FpGYzBsUxUOecYYfso0yA";
const FOLLOWING_OPERATION_ID: &str = "UCFedrkjMz7PeEAWCWhqFw";
const LIKES_OPERATION_ID: &str = "jTVU5QdKqziEc4GsLNvhMQ";
const BOOKMARKS_OPERATION_ID: &str = "-LGfdImKeQz0xS_jjUwzlA";
const HOME_LATEST_TIMELINE_OPERATION_ID: &str = "_qO7FJzShSKYWi9gtboE6A";
const HOME_TIMELINE_OPERATION_ID: &str = "V7xdnRnvW6a8vIsMr9xK7A";
const SEARCH_TIMELINE_OPERATION_ID: &str = "GcXk9vN_d1jUfHNqLacXQA";
const USER_SEARCH_TIMELINE_OPERATION_ID: &str = "M1jEez78PEfVfbQLvlWMvQ";
const TWEET_RESULT_BY_REST_ID_OPERATION_ID: &str = "aFvUsJm2c-oDkJV75blV6g";
const TWEET_DETAIL_OPERATION_ID: &str = "97JF30KziU00483E_8elBA";
const FAVORITERS_OPERATION_ID: &str = "b3OrdeHDQfb9zRMC0fV3bw";
const RETWEETERS_OPERATION_ID: &str = "wfglZEC0MRgBdxMa_1a5YQ";
const AUDIO_SPACE_BY_ID_OPERATION_ID: &str = "rR7CQrr8kxb6fatlUaB61Q";

/// Public Twitter/X API URL builder.
#[derive(Debug, Clone)]
#[doc(alias = "twitterApiUrls")]
pub struct TwitterApiUrls {
    graphql_base_url: Cow<'static, str>,
    guest_activate_url: Cow<'static, str>,
}

impl Default for TwitterApiUrls {
    fn default() -> Self {
        Self::new()
    }
}

impl TwitterApiUrls {
    /// Create a URL builder using the default X web endpoints.
    pub fn new() -> Self {
        Self {
            graphql_base_url: Cow::Borrowed(TWITTER_GRAPHQL_BASE_URL),
            guest_activate_url: Cow::Borrowed(TWITTER_GUEST_ACTIVATE_URL),
        }
    }

    /// Create a URL builder with explicit upstream overrides.
    pub fn with_base_urls(
        graphql_base_url: impl Into<Cow<'static, str>>,
        guest_activate_url: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            graphql_base_url: graphql_base_url.into(),
            guest_activate_url: guest_activate_url.into(),
        }
    }

    /// Return the guest-token activation URL.
    pub fn guest_activate(&self) -> &str {
        self.guest_activate_url.as_ref()
    }

    /// Build the user-profile URL for one screen name.
    #[doc(alias = "getUserProfile")]
    pub fn user_profile(&self, screen_name: &str) -> Result<String, AppError> {
        self.build_graphql_url(
            USER_BY_SCREEN_NAME_OPERATION_ID,
            "UserByScreenName",
            &json!({
                "screen_name": screen_name,
                "withGrokTranslatedBio": false,
            }),
            &current_user_profile_features(),
            Some(&current_user_profile_field_toggles()),
        )
    }

    /// Build the user-profile URL for one user rest id.
    #[doc(alias = "getUserProfileById")]
    pub fn user_profile_by_id(&self, user_id: &str) -> Result<String, AppError> {
        self.build_graphql_url(
            USER_BY_REST_ID_OPERATION_ID,
            "UserByRestId",
            &json!({
                "userId": user_id,
            }),
            &current_user_profile_features(),
            None,
        )
    }

    /// Build the user-timeline URL for one user id.
    #[doc(alias = "getUserTimeline")]
    pub fn user_timeline(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            USER_TWEETS_OPERATION_ID,
            "UserTweets",
            &json!({
                "userId": user_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": true,
                "withQuickPromoteEligibilityTweetFields": true,
                "withVoice": true,
            }),
            &current_user_timeline_features(),
            Some(&current_timeline_field_toggles()),
        )
    }

    /// Build the user-replies URL for one user id.
    #[doc(alias = "getUserReplies")]
    pub fn user_replies(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            USER_TWEETS_AND_REPLIES_OPERATION_ID,
            "UserTweetsAndReplies",
            &json!({
                "userId": user_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": true,
                "withCommunity": true,
                "withVoice": true,
            }),
            &current_user_timeline_features(),
            Some(&current_timeline_field_toggles()),
        )
    }

    /// Build the user-media URL for one user id.
    #[doc(alias = "getUserMedia")]
    pub fn user_media(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        let upstream_count = normalize_user_media_count(count);
        self.build_graphql_url(
            USER_MEDIA_OPERATION_ID,
            "UserMedia",
            &json!({
                "userId": user_id,
                "count": upstream_count,
                "cursor": cursor,
                "includePromotedContent": false,
                "withClientEventToken": false,
                "withBirdwatchNotes": false,
                "withVoice": true,
            }),
            &current_user_timeline_features(),
            Some(&current_timeline_field_toggles()),
        )
    }

    /// Build the followers URL for one user id.
    #[doc(alias = "getUserFollowers")]
    pub fn user_followers(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            FOLLOWERS_OPERATION_ID,
            "Followers",
            &json!({
                "userId": user_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": false,
                "withGrokTranslatedBio": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the following URL for one user id.
    #[doc(alias = "getUserFollowing")]
    pub fn user_following(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            FOLLOWING_OPERATION_ID,
            "Following",
            &json!({
                "userId": user_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": false,
                "withGrokTranslatedBio": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the authenticated user-likes URL for one user id.
    #[doc(alias = "getUserLikes")]
    pub fn user_likes(
        &self,
        user_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            LIKES_OPERATION_ID,
            "Likes",
            &json!({
                "userId": user_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": false,
                "withClientEventToken": false,
                "withBirdwatchNotes": false,
                "withVoice": true,
            }),
            &current_user_timeline_features(),
            Some(&current_timeline_field_toggles()),
        )
    }

    /// Build the bookmarks URL for the authenticated user.
    #[doc(alias = "getUserBookmarks")]
    pub fn user_bookmarks(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            BOOKMARKS_OPERATION_ID,
            "Bookmarks",
            &json!({
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the followed-feed URL for the authenticated user.
    #[doc(alias = "getFollowedFeed")]
    pub fn user_followed(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            HOME_LATEST_TIMELINE_OPERATION_ID,
            "HomeLatestTimeline",
            &json!({
                "count": count.unwrap_or(35),
                "cursor": cursor,
                "includePromotedContent": false,
                "latestControlAvailable": true,
                "withCommunity": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the recommended-feed URL for the authenticated user.
    #[doc(alias = "getRecommendedFeed")]
    pub fn user_recommended(
        &self,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            HOME_TIMELINE_OPERATION_ID,
            "HomeTimeline",
            &json!({
                "count": count.unwrap_or(35),
                "cursor": cursor,
                "includePromotedContent": false,
                "latestControlAvailable": true,
                "withCommunity": false,
                "seenTweetIds": [],
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the tweet-search URL for one raw query.
    #[doc(alias = "searchTweets")]
    pub fn search_tweets(
        &self,
        raw_query: &str,
        search_product: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            SEARCH_TIMELINE_OPERATION_ID,
            "SearchTimeline",
            &json!({
                "rawQuery": raw_query,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "querySource": "typed_query",
                "product": search_product,
                "withGrokTranslatedBio": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the user-search URL for one raw query.
    #[doc(alias = "searchUsers")]
    pub fn search_users(
        &self,
        raw_query: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            USER_SEARCH_TIMELINE_OPERATION_ID,
            "SearchTimeline",
            &json!({
                "rawQuery": raw_query,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "querySource": "typed_query",
                "product": "People",
                "withGrokTranslatedBio": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the tweet-detail URL for one tweet id.
    #[doc(alias = "getTweetDetail")]
    pub fn tweet_detail(&self, tweet_id: &str) -> Result<String, AppError> {
        self.build_graphql_url(
            TWEET_RESULT_BY_REST_ID_OPERATION_ID,
            "TweetResultByRestId",
            &json!({
                "tweetId": tweet_id,
                "referrer": "home",
                "with_rux_injections": false,
                "includePromotedContent": false,
                "withCommunity": false,
                "withQuickPromoteEligibilityTweetFields": false,
                "withBirdwatchNotes": false,
                "withVoice": false,
                "withV2Timeline": false,
            }),
            &json!({
                "creator_subscriptions_tweet_preview_api_enabled": true,
                "premium_content_api_read_enabled": false,
                "communities_web_enable_tweet_community_results_fetch": true,
                "c9s_tweet_anatomy_moderator_badge_enabled": true,
                "responsive_web_grok_analyze_button_fetch_trends_enabled": false,
                "responsive_web_grok_analyze_post_followups_enabled": false,
                "responsive_web_jetfuel_frame": false,
                "responsive_web_grok_share_attachment_enabled": true,
                "articles_preview_enabled": true,
                "responsive_web_edit_tweet_api_enabled": true,
                "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
                "view_counts_everywhere_api_enabled": true,
                "longform_notetweets_consumption_enabled": true,
                "responsive_web_twitter_article_tweet_consumption_enabled": true,
                "tweet_awards_web_tipping_enabled": false,
                "responsive_web_grok_show_grok_translated_post": false,
                "responsive_web_grok_analysis_button_from_backend": false,
                "creator_subscriptions_quote_tweet_preview_enabled": false,
                "freedom_of_speech_not_reach_fetch_enabled": true,
                "standardized_nudges_misinfo": true,
                "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
                "longform_notetweets_rich_text_read_enabled": true,
                "longform_notetweets_inline_media_enabled": true,
                "profile_label_improvements_pcf_label_in_post_enabled": true,
                "rweb_tipjar_consumption_enabled": true,
                "verified_phone_label_enabled": true,
                "responsive_web_grok_image_annotation_enabled": true,
                "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
                "responsive_web_graphql_timeline_navigation_enabled": true,
                "responsive_web_enhance_cards_enabled": false,
            }),
            None,
        )
    }

    /// Build the tweet-replies URL for one tweet id.
    #[doc(alias = "getTweetReplies")]
    pub fn tweet_replies(
        &self,
        tweet_id: &str,
        cursor: Option<&str>,
        ranking_mode: &str,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            TWEET_DETAIL_OPERATION_ID,
            "TweetDetail",
            &json!({
                "focalTweetId": tweet_id,
                "cursor": cursor,
                "referrer": "tweet",
                "with_rux_injections": false,
                "rankingMode": ranking_mode,
                "includePromotedContent": true,
                "withCommunity": true,
                "withQuickPromoteEligibilityTweetFields": true,
                "withBirdwatchNotes": true,
                "withVoice": true,
            }),
            &current_user_timeline_features(),
            Some(&current_tweet_replies_field_toggles()),
        )
    }

    /// Build the tweet-likers URL for one tweet id.
    #[doc(alias = "getTweetLikers")]
    pub fn tweet_likers(
        &self,
        tweet_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            FAVORITERS_OPERATION_ID,
            "Favoriters",
            &json!({
                "tweetId": tweet_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "enableRanking": false,
                "includePromotedContent": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the tweet-retweeters URL for one tweet id.
    #[doc(alias = "getTweetRetweeters")]
    pub fn tweet_retweeters(
        &self,
        tweet_id: &str,
        count: Option<u32>,
        cursor: Option<&str>,
    ) -> Result<String, AppError> {
        self.build_graphql_url(
            RETWEETERS_OPERATION_ID,
            "Retweeters",
            &json!({
                "tweetId": tweet_id,
                "count": count.unwrap_or(20),
                "cursor": cursor,
                "includePromotedContent": false,
            }),
            &current_user_timeline_features(),
            None,
        )
    }

    /// Build the space-detail URL for one space id.
    #[doc(alias = "getSpaceDetail")]
    pub fn space_detail(&self, space_id: &str) -> Result<String, AppError> {
        self.build_graphql_url(
            AUDIO_SPACE_BY_ID_OPERATION_ID,
            "AudioSpaceById",
            &json!({
                "id": space_id,
                "isMetatagsQuery": false,
                "withReplays": true,
                "withListeners": false,
            }),
            &json!({
                "spaces_2022_h2_spaces_communities": true,
                "spaces_2022_h2_clipping": true,
                "creator_subscriptions_tweet_preview_api_enabled": true,
                "profile_label_improvements_pcf_label_in_post_enabled": true,
                "responsive_web_profile_redirect_enabled": false,
                "rweb_tipjar_consumption_enabled": false,
                "verified_phone_label_enabled": false,
                "premium_content_api_read_enabled": false,
                "communities_web_enable_tweet_community_results_fetch": true,
                "c9s_tweet_anatomy_moderator_badge_enabled": true,
                "responsive_web_grok_analyze_button_fetch_trends_enabled": false,
                "responsive_web_grok_analyze_post_followups_enabled": true,
                "responsive_web_jetfuel_frame": true,
                "responsive_web_grok_share_attachment_enabled": true,
                "responsive_web_grok_annotations_enabled": false,
                "articles_preview_enabled": true,
                "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
                "responsive_web_edit_tweet_api_enabled": true,
                "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
                "view_counts_everywhere_api_enabled": true,
                "longform_notetweets_consumption_enabled": true,
                "responsive_web_twitter_article_tweet_consumption_enabled": true,
                "tweet_awards_web_tipping_enabled": false,
                "responsive_web_grok_show_grok_translated_post": false,
                "responsive_web_grok_analysis_button_from_backend": true,
                "post_ctas_fetch_enabled": true,
                "creator_subscriptions_quote_tweet_preview_enabled": false,
                "freedom_of_speech_not_reach_fetch_enabled": true,
                "standardized_nudges_misinfo": true,
                "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
                "longform_notetweets_rich_text_read_enabled": true,
                "longform_notetweets_inline_media_enabled": true,
                "responsive_web_grok_image_annotation_enabled": true,
                "responsive_web_grok_imagine_annotation_enabled": true,
                "responsive_web_graphql_timeline_navigation_enabled": true,
                "responsive_web_grok_community_note_auto_translation_is_enabled": false,
                "responsive_web_enhance_cards_enabled": false,
            }),
            None,
        )
    }

    fn build_graphql_url(
        &self,
        operation_id: &str,
        operation_name: &str,
        variables: &Value,
        features: &Value,
        field_toggles: Option<&Value>,
    ) -> Result<String, AppError> {
        let variables = compact_graphql_value(variables.clone());
        let features = compact_graphql_value(features.clone());
        let field_toggles = field_toggles.cloned().map(compact_graphql_value);
        let mut url = Url::parse(&format!(
            "{}/{operation_id}/{operation_name}",
            self.graphql_base_url.as_ref()
        ))
        .map_err(|error| AppError::InvalidRequestConfig(format!("invalid twitter url: {error}")))?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("variables", &variables.to_string());
            query.append_pair("features", &features.to_string());

            if let Some(field_toggles) = field_toggles.as_ref() {
                query.append_pair("fieldToggles", &field_toggles.to_string());
            }
        }

        Ok(url.to_string())
    }
}

fn current_user_profile_features() -> Value {
    json!({
        "hidden_profile_subscriptions_enabled": true,
        "profile_label_improvements_pcf_label_in_post_enabled": true,
        "responsive_web_profile_redirect_enabled": false,
        "rweb_tipjar_consumption_enabled": false,
        "verified_phone_label_enabled": false,
        "subscriptions_verification_info_is_identity_verified_enabled": true,
        "subscriptions_verification_info_verified_since_enabled": true,
        "highlights_tweets_tab_ui_enabled": true,
        "responsive_web_twitter_article_notes_tab_enabled": true,
        "subscriptions_feature_can_gift_premium": true,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "responsive_web_graphql_timeline_navigation_enabled": true,
    })
}

fn current_user_profile_field_toggles() -> Value {
    json!({
        "withPayments": false,
        "withAuxiliaryUserLabels": true,
    })
}

fn current_user_timeline_features() -> Value {
    json!({
        "rweb_video_screen_enabled": false,
        "profile_label_improvements_pcf_label_in_post_enabled": true,
        "responsive_web_profile_redirect_enabled": false,
        "rweb_tipjar_consumption_enabled": false,
        "verified_phone_label_enabled": false,
        "creator_subscriptions_tweet_preview_api_enabled": true,
        "responsive_web_graphql_timeline_navigation_enabled": true,
        "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
        "premium_content_api_read_enabled": false,
        "communities_web_enable_tweet_community_results_fetch": true,
        "c9s_tweet_anatomy_moderator_badge_enabled": true,
        "responsive_web_grok_analyze_button_fetch_trends_enabled": false,
        "responsive_web_grok_analyze_post_followups_enabled": true,
        "responsive_web_jetfuel_frame": true,
        "responsive_web_grok_share_attachment_enabled": true,
        "responsive_web_grok_annotations_enabled": true,
        "articles_preview_enabled": true,
        "responsive_web_edit_tweet_api_enabled": true,
        "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
        "view_counts_everywhere_api_enabled": true,
        "longform_notetweets_consumption_enabled": true,
        "responsive_web_twitter_article_tweet_consumption_enabled": true,
        "content_disclosure_indicator_enabled": true,
        "content_disclosure_ai_generated_indicator_enabled": true,
        "responsive_web_grok_show_grok_translated_post": false,
        "responsive_web_grok_analysis_button_from_backend": true,
        "post_ctas_fetch_enabled": false,
        "freedom_of_speech_not_reach_fetch_enabled": true,
        "standardized_nudges_misinfo": true,
        "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
        "longform_notetweets_rich_text_read_enabled": true,
        "longform_notetweets_inline_media_enabled": false,
        "responsive_web_grok_image_annotation_enabled": true,
        "responsive_web_grok_imagine_annotation_enabled": true,
        "responsive_web_grok_community_note_auto_translation_is_enabled": false,
        "responsive_web_enhance_cards_enabled": false,
    })
}

fn current_timeline_field_toggles() -> Value {
    json!({
        "withArticlePlainText": false,
    })
}

fn current_tweet_replies_field_toggles() -> Value {
    json!({
        "withArticleRichContentState": true,
        "withArticlePlainText": false,
        "withGrokAnalyze": false,
        "withDisallowedReplyControls": false,
    })
}

fn normalize_user_media_count(count: Option<u32>) -> u32 {
    count.unwrap_or(20).max(2)
}

fn compact_graphql_value(mut value: Value) -> Value {
    strip_null_fields(&mut value);
    value
}

fn strip_null_fields(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for child in map.values_mut() {
                strip_null_fields(child);
            }
            map.retain(|_, child| !child.is_null());
        }
        Value::Array(items) => {
            for item in items {
                strip_null_fields(item);
            }
        }
        _ => {}
    }
}

/// Create a public Twitter/X URL builder.
#[doc(alias = "createTwitterApiUrls")]
pub fn create_twitter_api_urls() -> TwitterApiUrls {
    TwitterApiUrls::new()
}
