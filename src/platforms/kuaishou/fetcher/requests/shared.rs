use reqwest::Url;

use crate::error::AppError;

use super::super::super::sign::KuaishouLiveApiRequest;

pub(super) fn create_live_api_request(
    base_url: &str,
    request_type: &str,
    pathname: &str,
    query: &[(&str, String)],
) -> Result<KuaishouLiveApiRequest, AppError> {
    let mut url = Url::parse(base_url).map_err(|error| {
        AppError::InvalidRequestConfig(format!(
            "invalid Kuaishou live base URL `{base_url}`: {error}"
        ))
    })?;

    url.set_path(pathname);
    url.set_query(None);

    {
        let mut pairs = url.query_pairs_mut();
        for (key, value) in query {
            pairs.append_pair(key, value);
        }
    }

    Ok(KuaishouLiveApiRequest::new(
        request_type.to_owned(),
        url.to_string(),
    ))
}
