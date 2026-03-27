//! Public Kuaishou request descriptor builders migrated from the TypeScript platform layer.

use crate::error::AppError;

use super::super::{
    KuaishouLiveApiRequest,
    fetcher::requests::{
        category_classify, category_config, category_data, emoji_list_request, interest_mask_list,
        live_detail_with_auth_token, live_gift_list, live_reco, live_websocket_info, playback_list,
        profile_interest_list, profile_liked, profile_private, profile_public, user_info_by_id,
        user_sensitive_info, user_work_list, video_work_request, work_comments_request,
    },
};
use super::types::KuaishouGraphqlRequest;

pub(super) const KUAISHOU_GRAPHQL_ENDPOINT: &str = "https://www.kuaishou.com/graphql";
pub(super) const KUAISHOU_LIVE_BASE_URL: &str = "https://live.kuaishou.com";

/// Public Kuaishou API request builder.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(alias = "kuaishouApiUrls")]
pub struct KuaishouApiUrls {
    graphql_endpoint: String,
    live_base_url: String,
}

impl Default for KuaishouApiUrls {
    fn default() -> Self {
        Self::new()
    }
}

impl KuaishouApiUrls {
    /// Create a Kuaishou API builder with the default platform endpoints.
    pub fn new() -> Self {
        Self::with_base_urls(KUAISHOU_GRAPHQL_ENDPOINT, KUAISHOU_LIVE_BASE_URL)
    }

    /// Create a Kuaishou API builder with explicit platform endpoints.
    pub fn with_base_urls(
        graphql_endpoint: impl Into<String>,
        live_base_url: impl Into<String>,
    ) -> Self {
        Self {
            graphql_endpoint: graphql_endpoint.into(),
            live_base_url: live_base_url.into(),
        }
    }

    /// Build the GraphQL request used to fetch one Kuaishou video work.
    #[doc(alias = "videoWork")]
    pub fn video_work(&self, photo_id: &str) -> KuaishouGraphqlRequest {
        KuaishouGraphqlRequest {
            request_type: "visionVideoDetail".to_owned(),
            url: self.graphql_endpoint.clone(),
            body: video_work_request(photo_id),
        }
    }

    /// Build the GraphQL request used to fetch Kuaishou work comments.
    #[doc(alias = "comments")]
    pub fn work_comments(&self, photo_id: &str) -> KuaishouGraphqlRequest {
        KuaishouGraphqlRequest {
            request_type: "commentListQuery".to_owned(),
            url: self.graphql_endpoint.clone(),
            body: work_comments_request(photo_id),
        }
    }

    /// Build the GraphQL request used to fetch the Kuaishou emoji catalog.
    #[doc(alias = "emojiList")]
    pub fn emoji_list(&self) -> KuaishouGraphqlRequest {
        KuaishouGraphqlRequest {
            request_type: "visionBaseEmoticons".to_owned(),
            url: self.graphql_endpoint.clone(),
            body: emoji_list_request(),
        }
    }

    /// Build the `userInfoById` live-api request.
    #[doc(alias = "userInfoById")]
    pub fn user_info_by_id(&self, principal_id: &str) -> Result<KuaishouLiveApiRequest, AppError> {
        user_info_by_id(&self.live_base_url, principal_id)
    }

    /// Build the `userSensitiveInfo` live-api request.
    #[doc(alias = "userSensitiveInfo")]
    pub fn user_sensitive_info(
        &self,
        principal_id: &str,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        user_sensitive_info(&self.live_base_url, principal_id)
    }

    /// Build the `profilePublic` live-api request.
    #[doc(alias = "profilePublic")]
    pub fn profile_public(
        &self,
        principal_id: &str,
        count: Option<u32>,
        pcursor: Option<&str>,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        profile_public(&self.live_base_url, principal_id, count, pcursor)
    }

    /// Build the `userWorkList` live-api request.
    #[doc(alias = "userWorkList")]
    pub fn user_work_list(
        &self,
        principal_id: &str,
        count: Option<u32>,
        pcursor: Option<&str>,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        user_work_list(&self.live_base_url, principal_id, count, pcursor)
    }

    /// Build the `profilePrivate` live-api request.
    #[doc(alias = "profilePrivate")]
    pub fn profile_private(&self, principal_id: &str) -> Result<KuaishouLiveApiRequest, AppError> {
        profile_private(&self.live_base_url, principal_id)
    }

    /// Build the `profileLiked` live-api request.
    #[doc(alias = "profileLiked")]
    pub fn profile_liked(&self, principal_id: &str) -> Result<KuaishouLiveApiRequest, AppError> {
        profile_liked(&self.live_base_url, principal_id)
    }

    /// Build the `profileInterestList` live-api request.
    #[doc(alias = "profileInterestList")]
    pub fn profile_interest_list(
        &self,
        principal_id: &str,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        profile_interest_list(&self.live_base_url, principal_id)
    }

    /// Build the `playbackList` live-api request.
    #[doc(alias = "playbackList")]
    pub fn playback_list(&self, principal_id: &str) -> Result<KuaishouLiveApiRequest, AppError> {
        playback_list(&self.live_base_url, principal_id)
    }

    /// Build the `interestMaskList` live-api request.
    #[doc(alias = "interestMaskList")]
    pub fn interest_mask_list(&self) -> Result<KuaishouLiveApiRequest, AppError> {
        interest_mask_list(&self.live_base_url)
    }

    /// Build the `categoryConfig` live-api request.
    #[doc(alias = "categoryConfig")]
    pub fn category_config(&self) -> Result<KuaishouLiveApiRequest, AppError> {
        category_config(&self.live_base_url)
    }

    /// Build the `categoryData` live-api request.
    #[doc(alias = "categoryData")]
    pub fn category_data(&self) -> Result<KuaishouLiveApiRequest, AppError> {
        category_data(&self.live_base_url)
    }

    /// Build the `categoryClassify` live-api request.
    #[doc(alias = "categoryClassify")]
    pub fn category_classify(&self) -> Result<KuaishouLiveApiRequest, AppError> {
        category_classify(&self.live_base_url)
    }

    /// Build the `liveDetail` live-api request.
    #[doc(alias = "liveDetail")]
    pub fn live_detail(
        &self,
        principal_id: &str,
        auth_token: Option<&str>,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        live_detail_with_auth_token(&self.live_base_url, principal_id, auth_token)
    }

    /// Build the `liveGiftList` live-api request.
    #[doc(alias = "liveGiftList")]
    pub fn live_gift_list(&self, live_stream_id: &str) -> Result<KuaishouLiveApiRequest, AppError> {
        live_gift_list(&self.live_base_url, live_stream_id)
    }

    /// Build the `liveWebsocketInfo` live-api request.
    #[doc(alias = "liveWebsocketInfo")]
    pub fn live_websocket_info(
        &self,
        live_stream_id: &str,
    ) -> Result<KuaishouLiveApiRequest, AppError> {
        live_websocket_info(&self.live_base_url, live_stream_id)
    }

    /// Build the `liveReco` live-api request.
    #[doc(alias = "liveReco")]
    pub fn live_reco(&self, game_id: Option<&str>) -> Result<KuaishouLiveApiRequest, AppError> {
        live_reco(&self.live_base_url, game_id)
    }
}

/// Create a public Kuaishou API builder.
#[doc(alias = "createKuaishouApiUrls")]
pub fn create_kuaishou_api_urls() -> KuaishouApiUrls {
    KuaishouApiUrls::new()
}
