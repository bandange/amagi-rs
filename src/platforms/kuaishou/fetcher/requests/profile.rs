use crate::error::AppError;

use super::super::super::sign::KuaishouLiveApiRequest;
use super::shared::create_live_api_request;

const DEFAULT_PAGE_SIZE: u32 = 12;

pub(crate) fn user_info_by_id(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "userInfoById",
        "/live_api/baseuser/userinfo/byid",
        &[
            ("caver", "2".to_owned()),
            ("principalId", principal_id.to_owned()),
        ],
    )?
    .with_sign_path("/rest/k/user/info"))
}

pub(crate) fn user_sensitive_info(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "userSensitiveInfo",
        "/live_api/baseuser/userinfo/sensitive",
        &[
            ("caver", "2".to_owned()),
            ("principalId", principal_id.to_owned()),
        ],
    )?
    .with_sign_path("/rest/k/user/info/sensitive"))
}

pub(crate) fn profile_public(
    base_url: &str,
    principal_id: &str,
    count: Option<u32>,
    pcursor: Option<&str>,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "profilePublic",
        "/live_api/profile/public",
        &[
            ("caver", "2".to_owned()),
            ("count", count.unwrap_or(DEFAULT_PAGE_SIZE).to_string()),
            ("hasMore", "true".to_owned()),
            ("pcursor", pcursor.unwrap_or_default().to_owned()),
            ("principalId", principal_id.to_owned()),
            ("privacy", "public".to_owned()),
        ],
    )?
    .with_sign_path("/rest/k/feed/profile"))
}

pub(crate) fn user_work_list(
    base_url: &str,
    principal_id: &str,
    count: Option<u32>,
    pcursor: Option<&str>,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(profile_public(base_url, principal_id, count, pcursor)?
        .with_sign_path("/rest/k/feed/profile"))
}

pub(crate) fn profile_private(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "profilePrivate",
        "/live_api/profile/private",
        &[
            ("caver", "2".to_owned()),
            ("count", DEFAULT_PAGE_SIZE.to_string()),
            ("hasMore", "true".to_owned()),
            ("pcursor", String::new()),
            ("principalId", principal_id.to_owned()),
            ("privacy", "private".to_owned()),
        ],
    )?
    .with_requires_sign(false))
}

pub(crate) fn profile_liked(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    Ok(create_live_api_request(
        base_url,
        "profileLiked",
        "/live_api/profile/liked",
        &[
            ("caver", "2".to_owned()),
            ("count", DEFAULT_PAGE_SIZE.to_string()),
            ("hasMore", "true".to_owned()),
            ("pcursor", String::new()),
            ("principalId", principal_id.to_owned()),
            ("privacy", "liked".to_owned()),
        ],
    )?
    .with_requires_sign(false))
}

pub(crate) fn profile_interest_list(
    base_url: &str,
    principal_id: &str,
) -> Result<KuaishouLiveApiRequest, AppError> {
    create_live_api_request(
        base_url,
        "profileInterestList",
        "/live_api/profile/interestlist",
        &[
            ("caver", "2".to_owned()),
            ("limit", "4".to_owned()),
            ("principalId", principal_id.to_owned()),
        ],
    )
}
