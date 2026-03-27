//! Douyin request-signing and token-generation helpers.

mod a_bogus;
mod tokens;
mod x_bogus;

pub use a_bogus::{
    DEFAULT_WINDOW_ENV, build_signed_url_with_a_bogus, clean_user_agent_for_signing,
    generate_a_bogus, generate_a_bogus_with_options,
};
pub use tokens::{
    DEFAULT_USER_AGENT, generate_ms_token, generate_ms_token_from_entropy, generate_verify_fp,
    generate_verify_fp_with_entropy,
};
pub use x_bogus::{build_signed_url_with_x_bogus, generate_x_bogus, generate_x_bogus_at};
