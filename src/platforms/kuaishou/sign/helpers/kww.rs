use std::{
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};

use super::crypto::encrypt_kuaishou_anonymous_kww_seed;

const KUAISHOU_ANONYMOUS_KWW_KEY: &[u8; 16] = b"K8wm5PvY9nX7qJc2";
const KUAISHOU_ANONYMOUS_KWW_SUFFIX: &str = "###ssrd";
const KUAISHOU_ANONYMOUS_KWW_ALPHABET: &[u8; 62] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

static KUAISHOU_ANONYMOUS_KWW_CACHE: OnceLock<Mutex<Option<String>>> = OnceLock::new();
static KUAISHOU_RANDOM_COUNTER: AtomicU64 = AtomicU64::new(0);

fn current_timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn next_pseudo_random_u64() -> u64 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let counter = KUAISHOU_RANDOM_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    let mut value = time ^ counter.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    value ^= value >> 12;
    value ^= value << 25;
    value ^= value >> 27;
    value.wrapping_mul(0x2545_F491_4F6C_DD1D)
}

fn generate_kuaishou_anonymous_kww_seed() -> String {
    let mut random_part = String::with_capacity(8);

    for _ in 0..8 {
        let index =
            (next_pseudo_random_u64() % KUAISHOU_ANONYMOUS_KWW_ALPHABET.len() as u64) as usize;
        random_part.push(KUAISHOU_ANONYMOUS_KWW_ALPHABET[index] as char);
    }

    format!("{}|{random_part}", current_timestamp_millis())
}

/// Extract a cookie value from the raw cookie header string.
pub fn extract_cookie_value(cookie: Option<&str>, key: &str) -> String {
    cookie
        .filter(|value| !value.trim().is_empty())
        .and_then(|value| {
            value.split(';').find_map(|segment| {
                let (candidate_key, candidate_value) = segment.trim().split_once('=')?;
                (candidate_key == key).then(|| candidate_value.to_owned())
            })
        })
        .unwrap_or_default()
}

/// Derive the cached anonymous `kww` value used when `kwfv1` is absent.
pub fn derive_kuaishou_anonymous_kww() -> String {
    let cache = KUAISHOU_ANONYMOUS_KWW_CACHE.get_or_init(|| Mutex::new(None));
    let mut value = cache
        .lock()
        .expect("kuaishou anonymous kww lock should not be poisoned");

    if let Some(cached) = value.as_ref() {
        return cached.clone();
    }

    let generated =
        derive_kuaishou_anonymous_kww_from_seed(&generate_kuaishou_anonymous_kww_seed());
    *value = Some(generated.clone());

    generated
}

/// Derive the `kww` header value from the cookie or the anonymous fallback.
pub fn derive_kuaishou_kww(cookie: Option<&str>) -> String {
    let kwfv1 = extract_cookie_value(cookie, "kwfv1");

    if kwfv1.is_empty() {
        derive_kuaishou_anonymous_kww()
    } else {
        kwfv1
    }
}

pub(crate) fn derive_kuaishou_anonymous_kww_from_seed(seed: &str) -> String {
    format!(
        "{}{}",
        encrypt_kuaishou_anonymous_kww_seed(seed, KUAISHOU_ANONYMOUS_KWW_KEY),
        KUAISHOU_ANONYMOUS_KWW_SUFFIX
    )
}
