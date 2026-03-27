use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use crate::client::{PlatformClient, RequestProfile};

mod content;
pub(crate) mod requests;
mod search;
mod transport;
mod user;

use crate::error::AppError;

use super::{CookieJar, XiaohongshuBrowserState, XiaohongshuSession, XiaohongshuSigner};

const XIAOHONGSHU_API_BASE_URL: &str = "https://edith.xiaohongshu.com";
const XIAOHONGSHU_WEB_BASE_URL: &str = "https://www.xiaohongshu.com";

#[derive(Debug)]
struct XiaohongshuSigningState {
    signer: XiaohongshuSigner,
    session: XiaohongshuSession,
    cookies: CookieJar,
}

/// Xiaohongshu fetcher surface.
#[derive(Debug, Clone)]
pub struct XiaohongshuFetcher {
    request_profile: RequestProfile,
    api_base_url: Cow<'static, str>,
    web_base_url: Cow<'static, str>,
    signing: Arc<Mutex<XiaohongshuSigningState>>,
}

impl XiaohongshuFetcher {
    /// Create a fetcher from a Xiaohongshu-scoped [`PlatformClient`].
    pub fn new(client: PlatformClient) -> Self {
        let request_profile = client.request_profile();
        let cookie = request_profile
            .headers
            .get("cookie")
            .or_else(|| request_profile.headers.get("Cookie"))
            .cloned()
            .unwrap_or_default();

        Self {
            request_profile,
            api_base_url: Cow::Borrowed(XIAOHONGSHU_API_BASE_URL),
            web_base_url: Cow::Borrowed(XIAOHONGSHU_WEB_BASE_URL),
            signing: Arc::new(Mutex::new(XiaohongshuSigningState {
                signer: XiaohongshuSigner::new(),
                session: XiaohongshuSession::new(),
                cookies: CookieJar::parse(&cookie),
            })),
        }
    }

    /// Create a fetcher from a raw Xiaohongshu cookie and optional request overrides.
    #[doc(alias = "createBoundXiaohongshuFetcher")]
    pub fn from_cookie(cookie: impl Into<String>, request: crate::client::RequestConfig) -> Self {
        Self::new(PlatformClient {
            platform: crate::catalog::Platform::Xiaohongshu,
            cookie: Some(cookie.into()),
            request,
        })
    }

    /// Return the resolved request profile bound to this fetcher.
    pub fn request_profile(&self) -> &RequestProfile {
        &self.request_profile
    }

    /// Bind a legacy browser state placeholder for compatibility.
    ///
    /// The current pure-protocol signer does not require browser runtime state,
    /// so this hook is effectively a no-op compatibility surface.
    pub fn set_browser_state(
        &self,
        browser_state: XiaohongshuBrowserState,
    ) -> Result<(), AppError> {
        let mut signing = self.signing.lock().map_err(|_| {
            AppError::InvalidRequestConfig("xiaohongshu signer lock poisoned".into())
        })?;
        signing.signer.set_browser_state(browser_state);
        Ok(())
    }
}
