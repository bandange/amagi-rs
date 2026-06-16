//! Xiaohongshu pure-protocol signing surface.
//!
//! The current Rust implementation follows the `mns0301` request signature
//! layout and the pure-protocol `x-s-common` flow used by the verified
//! `xhshow-ts` implementation, without requiring browser runtime state.

mod codec;
mod fingerprint;
mod payload;
mod signer;
mod types;
mod x3;

pub use signer::XiaohongshuSigner;
pub use types::{
    CookieJar, XiaohongshuBrowserState, XiaohongshuHeaders, XiaohongshuMethod,
    XiaohongshuMnsv2Input, XiaohongshuXsCommonPayload, XiaohongshuXsEnvelope,
};
