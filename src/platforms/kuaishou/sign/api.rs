use std::collections::BTreeMap;

use reqwest::Url;

use crate::error::AppError;

use super::helpers::build_kuaishou_hxfalcon_sign_input;
use super::state::reserve_kuaishou_runtime_state;
use super::{
    derive_kuaishou_kww, derive_kuaishou_pure_signature, derive_kuaishou_secs_state,
    types::{
        KuaishouGeneratedHxfalcon, KuaishouHxfalconPayload, KuaishouLiveApiRequest,
        KuaishouLiveApiSignature, KuaishouPureSignContext,
    },
};

fn current_timestamp_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn next_random_fraction() -> f64 {
    let value = current_timestamp_millis()
        .wrapping_mul(1_103_515_245)
        .wrapping_add(12_345);
    let normalized = (value >> 11) & ((1u64 << 53) - 1);

    normalized as f64 / ((1u64 << 53) as f64)
}

fn set_query_param(url: &mut Url, key: &str, value: &str) {
    let mut updated = false;
    let mut pairs = Vec::new();

    for (existing_key, existing_value) in url.query_pairs() {
        if existing_key == key {
            if !updated {
                pairs.push((existing_key.into_owned(), value.to_owned()));
                updated = true;
            }
        } else {
            pairs.push((existing_key.into_owned(), existing_value.into_owned()));
        }
    }

    if !updated {
        pairs.push((key.to_owned(), value.to_owned()));
    }

    url.set_query(None);
    let mut query_pairs = url.query_pairs_mut();

    for (pair_key, pair_value) in pairs {
        query_pairs.append_pair(&pair_key, &pair_value);
    }
}

fn build_signed_live_api_url(
    url: &str,
    cookie: Option<&str>,
    cat_version: &str,
    sign_result: &str,
) -> Result<(String, BTreeMap<String, String>), AppError> {
    let mut signed_url = Url::parse(url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Kuaishou live_api URL {url}: {error}"))
    })?;
    let mut headers = BTreeMap::new();
    let kww = derive_kuaishou_kww(cookie);

    set_query_param(&mut signed_url, "__NS_hxfalcon", sign_result);
    set_query_param(&mut signed_url, "caver", cat_version);

    if !kww.is_empty() {
        headers.insert("kww".to_owned(), kww);
    }

    Ok((signed_url.to_string(), headers))
}

/// Return the current Kuaishou pure-sign `caver` value.
pub fn get_cat_version() -> String {
    super::get_kuaishou_pure_runtime_state().cat_version
}

/// Generate the Kuaishou `kww` header value from the cookie or anonymous fallback.
pub fn generate_kww(cookie: Option<&str>) -> String {
    derive_kuaishou_kww(cookie)
}

/// Generate `__NS_hxfalcon` materials from a normalized payload.
pub fn generate_hxfalcon_from_payload(
    payload: &KuaishouHxfalconPayload,
) -> KuaishouGeneratedHxfalcon {
    let sign_input = build_kuaishou_hxfalcon_sign_input(payload);
    let runtime_state = reserve_kuaishou_runtime_state();
    let cat_version = runtime_state.cat_version.clone();
    let sign_result = derive_kuaishou_pure_signature(&KuaishouPureSignContext {
        count: runtime_state.count,
        script_count: Some(0),
        secs: Some(derive_kuaishou_secs_state(runtime_state.count, None)),
        random_value: next_random_fraction(),
        sign_input: sign_input.clone(),
        startup_random: runtime_state.startup_random,
        timestamp: current_timestamp_millis(),
    })
    .sign_result;

    KuaishouGeneratedHxfalcon {
        sign_result,
        sign_input,
        cat_version,
    }
}

/// Sign a Kuaishou `live_api` URL.
pub fn sign_live_api_url(
    url: &str,
    cookie: Option<&str>,
    sign_path: Option<&str>,
) -> Result<KuaishouLiveApiSignature, AppError> {
    let payload = super::build_kuaishou_hxfalcon_payload(url, sign_path)?;
    let generated = generate_hxfalcon_from_payload(&payload);
    let (url, headers) =
        build_signed_live_api_url(url, cookie, &generated.cat_version, &generated.sign_result)?;

    Ok(KuaishouLiveApiSignature {
        url,
        headers,
        sign_result: generated.sign_result,
        sign_input: generated.sign_input,
        cat_version: generated.cat_version,
    })
}

/// Sign a structured Kuaishou `live_api` request description.
pub fn sign_live_api_request(
    request: &KuaishouLiveApiRequest,
    cookie: Option<&str>,
) -> Result<KuaishouLiveApiSignature, AppError> {
    sign_live_api_url(&request.url, cookie, request.sign_path.as_deref())
}
