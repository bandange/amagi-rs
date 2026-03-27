use crate::error::AppError;

use super::super::types::{
    BilibiliDynamicCard, BilibiliDynamicDetail, BilibiliLiveRoomInfo, BilibiliLiveRoomInit,
    BilibiliUploaderTotalViews, BilibiliUserCard, BilibiliUserDynamicList, BilibiliUserSpaceInfo,
};
use super::{BilibiliFetcher, requests};

impl BilibiliFetcher {
    /// Fetch one Bilibili user card payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchUserCard")]
    pub async fn fetch_user_card(&self, host_mid: u64) -> Result<BilibiliUserCard, AppError> {
        self.fetch_json(&requests::user_card(self.api_base_url.as_ref(), host_mid)?)
            .await
    }

    /// Fetch one page of Bilibili user dynamic items.
    ///
    /// # Errors
    ///
    /// Returns an error when the WBI keys cannot be resolved, the upstream
    /// request fails, or the response body contains a non-zero API status code.
    #[doc(alias = "fetchUserDynamicList")]
    pub async fn fetch_user_dynamic_list(
        &self,
        host_mid: u64,
    ) -> Result<BilibiliUserDynamicList, AppError> {
        let signed_url = self
            .sign_wbi_url(&requests::user_dynamic_list(
                self.api_base_url.as_ref(),
                host_mid,
            )?)
            .await?;
        self.fetch_json_with_referer(
            &signed_url,
            Some(&format!("https://space.bilibili.com/{host_mid}/dynamic")),
        )
        .await
    }

    /// Fetch one Bilibili user space payload signed with WBI parameters.
    ///
    /// # Errors
    ///
    /// Returns an error when the WBI keys cannot be resolved, the upstream
    /// request fails, or the response body contains a non-zero API status code.
    #[doc(alias = "fetchUserSpaceInfo")]
    pub async fn fetch_user_space_info(
        &self,
        host_mid: u64,
    ) -> Result<BilibiliUserSpaceInfo, AppError> {
        let signed_url = self
            .sign_wbi_url(&requests::user_space_info(
                self.api_base_url.as_ref(),
                host_mid,
            )?)
            .await?;
        self.fetch_json(&signed_url).await
    }

    /// Fetch the total play count payload for one Bilibili uploader.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero Bilibili API status code.
    #[doc(alias = "fetchUploaderTotalViews")]
    pub async fn fetch_uploader_total_views(
        &self,
        host_mid: u64,
    ) -> Result<BilibiliUploaderTotalViews, AppError> {
        self.fetch_json(&requests::uploader_total_views(
            self.api_base_url.as_ref(),
            host_mid,
        )?)
        .await
    }

    /// Fetch one Bilibili dynamic detail payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero API status code.
    #[doc(alias = "fetchDynamicDetail")]
    pub async fn fetch_dynamic_detail(
        &self,
        dynamic_id: &str,
    ) -> Result<BilibiliDynamicDetail, AppError> {
        self.fetch_json(&requests::dynamic_detail(
            self.api_base_url.as_ref(),
            dynamic_id,
        )?)
        .await
    }

    /// Fetch one Bilibili dynamic card payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero API status code.
    #[doc(alias = "fetchDynamicCard")]
    pub async fn fetch_dynamic_card(
        &self,
        dynamic_id: &str,
    ) -> Result<BilibiliDynamicCard, AppError> {
        self.fetch_json(&requests::dynamic_card(
            self.vc_base_url.as_ref(),
            dynamic_id,
        )?)
        .await
    }

    /// Fetch one Bilibili live room detail payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero API status code.
    #[doc(alias = "fetchLiveRoomInfo")]
    pub async fn fetch_live_room_info(
        &self,
        room_id: u64,
    ) -> Result<BilibiliLiveRoomInfo, AppError> {
        self.fetch_json(&requests::live_room_info(
            self.live_base_url.as_ref(),
            room_id,
        )?)
        .await
    }

    /// Fetch one Bilibili live room init payload.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response body
    /// contains a non-zero API status code.
    #[doc(alias = "fetchLiveRoomInitInfo")]
    pub async fn fetch_live_room_init(
        &self,
        room_id: u64,
    ) -> Result<BilibiliLiveRoomInit, AppError> {
        self.fetch_json(&requests::live_room_init(
            self.live_base_url.as_ref(),
            room_id,
        )?)
        .await
    }
}
