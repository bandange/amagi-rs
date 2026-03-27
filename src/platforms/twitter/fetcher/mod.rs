mod content;
mod space;
mod transport;
mod user;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

use super::api::TwitterApiUrls;
use super::sign::ClientTransactionState;
use crate::client::{PlatformClient, RequestProfile};

const CLIENT_TRANSACTION_CACHE_TTL_SECS: u64 = 900;

#[derive(Debug, Clone)]
struct CachedTransactionState {
    state: ClientTransactionState,
    expires_at_unix_secs: u64,
}

type TransactionStateHandle = Arc<Mutex<Option<CachedTransactionState>>>;

static SHARED_TRANSACTION_STATE_HANDLES: OnceLock<Mutex<BTreeMap<String, TransactionStateHandle>>> =
    OnceLock::new();

/// Rust-native Twitter/X fetcher backed by the public web protocol.
#[derive(Debug, Clone)]
pub struct TwitterFetcher {
    request_profile: RequestProfile,
    api_urls: TwitterApiUrls,
    web_base_url: Cow<'static, str>,
    transaction_state: Arc<Mutex<Option<CachedTransactionState>>>,
}

impl TwitterFetcher {
    /// Create a fetcher from a Twitter-scoped [`PlatformClient`].
    pub fn new(client: PlatformClient) -> Self {
        let web_base_url = Cow::Borrowed("https://x.com");
        Self {
            request_profile: client.request_profile(),
            api_urls: TwitterApiUrls::new(),
            transaction_state: shared_transaction_state_handle(web_base_url.as_ref()),
            web_base_url,
        }
    }

    /// Create a fetcher from a raw cookie and optional request overrides.
    #[doc(alias = "createBoundTwitterFetcher")]
    pub fn from_cookie(cookie: impl Into<String>, request: crate::client::RequestConfig) -> Self {
        Self::new(PlatformClient {
            platform: crate::catalog::Platform::Twitter,
            cookie: Some(cookie.into()),
            request,
        })
    }

    /// Return the resolved request profile bound to this fetcher.
    pub fn request_profile(&self) -> &RequestProfile {
        &self.request_profile
    }
}

fn shared_transaction_state_handle(web_base_url: &str) -> TransactionStateHandle {
    let handles = SHARED_TRANSACTION_STATE_HANDLES.get_or_init(|| Mutex::new(BTreeMap::new()));
    let Ok(mut handles) = handles.lock() else {
        return Arc::new(Mutex::new(None));
    };

    handles
        .entry(web_base_url.to_owned())
        .or_insert_with(|| Arc::new(Mutex::new(None)))
        .clone()
}
