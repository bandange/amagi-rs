use crate::error::AppError;

use super::{
    super::types::{KuaishouUserProfile, KuaishouUserWorkList},
    KuaishouFetcher,
    profile::{KuaishouUserProfileSources, build_kuaishou_user_profile},
    requests,
    value::resolve_kuaishou_user_work_list,
};

impl KuaishouFetcher {
    /// Fetch the aggregated Kuaishou user profile page.
    ///
    /// # Errors
    ///
    /// Returns an error when the primary user-info request fails or the
    /// upstream responses cannot be decoded.
    #[doc(alias = "fetchUserProfile")]
    pub async fn fetch_user_profile(
        &self,
        principal_id: &str,
    ) -> Result<KuaishouUserProfile, AppError> {
        let referer_path = format!("profile/{principal_id}");
        let base_url = self.live_base_url.as_ref();

        let user_info_request = requests::user_info_by_id(base_url, principal_id)?;
        let sensitive_request = requests::user_sensitive_info(base_url, principal_id)?;
        let public_request = requests::profile_public(base_url, principal_id, None, None)?;
        let private_request = requests::profile_private(base_url, principal_id)?;
        let liked_request = requests::profile_liked(base_url, principal_id)?;
        let playback_request = requests::playback_list(base_url, principal_id)?;
        let interest_list_request = requests::profile_interest_list(base_url, principal_id)?;
        let interest_mask_request = requests::interest_mask_list(base_url)?;
        let category_config_request = requests::category_config(base_url)?;
        let category_data_request = requests::category_data(base_url)?;
        let category_classify_request = requests::category_classify(base_url)?;
        let live_detail_request = requests::live_detail(base_url, principal_id)?;

        let (
            user_info_payload,
            sensitive_payload,
            public_payload,
            private_payload,
            liked_payload,
            playback_payload,
            interest_list_payload,
            interest_mask_payload,
            category_config_payload,
            category_data_payload,
            category_classify_payload,
            live_detail_payload,
        ) = tokio::join!(
            self.send_live_api_request(&user_info_request, &referer_path, false),
            self.send_live_api_request(&sensitive_request, &referer_path, true),
            self.send_live_api_request(&public_request, &referer_path, true),
            self.send_live_api_request(&private_request, &referer_path, true),
            self.send_live_api_request(&liked_request, &referer_path, true),
            self.send_live_api_request(&playback_request, &referer_path, true),
            self.send_live_api_request(&interest_list_request, &referer_path, true),
            self.send_live_api_request(&interest_mask_request, &referer_path, true),
            self.send_live_api_request(&category_config_request, &referer_path, false),
            self.send_live_api_request(&category_data_request, &referer_path, false),
            self.send_live_api_request(&category_classify_request, &referer_path, false),
            self.send_live_api_request(&live_detail_request, &referer_path, true),
        );

        let user_info_payload = user_info_payload?;
        let sensitive_payload = sensitive_payload.ok();
        let public_payload = public_payload.ok();
        let private_payload = private_payload.ok();
        let liked_payload = liked_payload.ok();
        let playback_payload = playback_payload.ok();
        let interest_list_payload = interest_list_payload.ok();
        let interest_mask_payload = interest_mask_payload.ok();
        let category_config_payload = category_config_payload.ok();
        let category_data_payload = category_data_payload.ok();
        let category_classify_payload = category_classify_payload.ok();
        let live_detail_payload = live_detail_payload.ok();

        Ok(build_kuaishou_user_profile(
            principal_id,
            KuaishouUserProfileSources {
                user_info_payload: &user_info_payload,
                sensitive_payload: sensitive_payload.as_ref(),
                public_payload: public_payload.as_ref(),
                private_payload: private_payload.as_ref(),
                liked_payload: liked_payload.as_ref(),
                playback_payload: playback_payload.as_ref(),
                interest_list_payload: interest_list_payload.as_ref(),
                interest_mask_payload: interest_mask_payload.as_ref(),
                category_config_payload: category_config_payload.as_ref(),
                category_data_payload: category_data_payload.as_ref(),
                category_classify_payload: category_classify_payload.as_ref(),
                live_detail_payload: live_detail_payload.as_ref(),
            },
        ))
    }

    /// Fetch paginated public works for one Kuaishou user.
    ///
    /// # Errors
    ///
    /// Returns an error when the upstream request fails or the response cannot
    /// be decoded.
    #[doc(alias = "fetchUserWorkList")]
    pub async fn fetch_user_work_list(
        &self,
        principal_id: &str,
        count: Option<u32>,
        pcursor: Option<&str>,
    ) -> Result<KuaishouUserWorkList, AppError> {
        let referer_path = format!("profile/{principal_id}");
        let request =
            requests::user_work_list(self.live_base_url.as_ref(), principal_id, count, pcursor)?;
        let payload = self
            .send_live_api_request(&request, &referer_path, true)
            .await?;

        Ok(resolve_kuaishou_user_work_list(principal_id, &payload))
    }
}
