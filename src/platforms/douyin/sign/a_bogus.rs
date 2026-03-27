use reqwest::Url;

use crate::error::AppError;
use crate::platforms::douyin::sign::tokens::DEFAULT_USER_AGENT;
use crate::platforms::internal::random::{PseudoRandom, now_unix_ms};
use crate::platforms::internal::rc4::rc4_encrypt;
use crate::platforms::internal::sm3::sm3_sum;

const ALPHABET_S3: &[u8; 64] = b"ckdp1h4ZKsUB80/Mfvw36XIgR25+WQAlEi7NLboqYTOPuzmFjJnryx9HVGDaStCe";
const ALPHABET_S4: &[u8; 64] = b"Dkdpgh2ZmsQB80/MfvV36XI1R45-WUAlEixNLwoqYTOPuzKFjJnry79HbGcaStCe";

/// Default browser-environment fingerprint used by the original signer.
pub const DEFAULT_WINDOW_ENV: &str =
    "1536|747|1536|834|0|30|0|0|1536|834|1536|864|1525|747|24|24|Win32";

/// Remove the Edge token from a user-agent before Douyin `a_bogus` signing.
pub fn clean_user_agent_for_signing(user_agent: &str) -> String {
    user_agent
        .split_whitespace()
        .filter(|part| !part.starts_with("Edg/"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a browser-style Douyin `a_bogus` signature.
pub fn generate_a_bogus(url: &str, user_agent: Option<&str>) -> Result<String, AppError> {
    let start_ms = now_unix_ms();
    let mut random = PseudoRandom::from_system();
    let random_values = [
        random.next_mod(10_000),
        random.next_mod(10_000),
        random.next_mod(10_000),
    ];
    let end_ms = now_unix_ms();

    generate_a_bogus_with_options(
        url,
        user_agent.unwrap_or(DEFAULT_USER_AGENT),
        start_ms,
        end_ms,
        random_values,
        DEFAULT_WINDOW_ENV,
    )
}

/// Generate a deterministic Douyin `a_bogus` signature from fixed timing and entropy.
pub fn generate_a_bogus_with_options(
    url: &str,
    user_agent: &str,
    start_ms: u64,
    end_ms: u64,
    random_values: [u32; 3],
    window_env: &str,
) -> Result<String, AppError> {
    let parsed = Url::parse(url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Douyin URL for a_bogus {url}: {error}"))
    })?;
    let url_search_params = parsed.query().unwrap_or_default();
    let cleaned_user_agent = clean_user_agent_for_signing(user_agent);

    let mut payload = generate_random_prefix(random_values);
    payload.extend(generate_rc4_bb_bytes(
        url_search_params,
        &cleaned_user_agent,
        window_env,
        start_ms,
        end_ms,
    ));

    Ok(format!("{}=", custom_encode(&payload, ALPHABET_S4)))
}

/// Append an `a_bogus` signature to a request URL.
pub fn build_signed_url_with_a_bogus(
    url: &str,
    user_agent: Option<&str>,
) -> Result<String, AppError> {
    let signature = generate_a_bogus(url, user_agent)?;
    let separator = if url.contains('?') { '&' } else { '?' };
    Ok(format!("{url}{separator}a_bogus={signature}"))
}

fn generate_random_prefix(random_values: [u32; 3]) -> Vec<u8> {
    let mut output = Vec::with_capacity(12);
    output.extend(generate_random_chunk(random_values[0], [3, 45]));
    output.extend(generate_random_chunk(random_values[1], [1, 0]));
    output.extend(generate_random_chunk(random_values[2], [1, 5]));
    output
}

fn generate_random_chunk(random: u32, option: [u8; 2]) -> [u8; 4] {
    [
        ((random as u8) & 170) | (option[0] & 85),
        ((random as u8) & 85) | (option[0] & 170),
        (((random >> 8) as u8) & 170) | (option[1] & 85),
        (((random >> 8) as u8) & 85) | (option[1] & 170),
    ]
}

fn generate_rc4_bb_bytes(
    url_search_params: &str,
    user_agent: &str,
    window_env: &str,
    start_ms: u64,
    end_ms: u64,
) -> Vec<u8> {
    let url_hash = sm3_sum(&sm3_sum(format!("{url_search_params}cus").as_bytes()));
    let suffix_hash = sm3_sum(&sm3_sum(b"cus"));
    let ua_rc4 = rc4_encrypt(&[0, 1, 14], user_agent.as_bytes());
    let ua_hash = sm3_sum(custom_encode(&ua_rc4, ALPHABET_S3).as_bytes());

    let window_env_bytes = window_env.as_bytes();
    let mut values = [0u64; 73];

    values[8] = 3;
    values[10] = end_ms;
    values[16] = start_ms;
    values[18] = 44;

    values[20] = (start_ms >> 24) & 255;
    values[21] = (start_ms >> 16) & 255;
    values[22] = (start_ms >> 8) & 255;
    values[23] = start_ms & 255;
    values[24] = (start_ms >> 32) & 255;
    values[25] = (start_ms >> 40) & 255;

    values[29] = 0;
    values[30] = 0;
    values[31] = 1;
    values[34] = 0;
    values[35] = 0;
    values[36] = 0;
    values[37] = 14;

    values[38] = url_hash[21] as u64;
    values[39] = url_hash[22] as u64;
    values[40] = suffix_hash[21] as u64;
    values[41] = suffix_hash[22] as u64;
    values[42] = ua_hash[23] as u64;
    values[43] = ua_hash[24] as u64;

    values[44] = (end_ms >> 24) & 255;
    values[45] = (end_ms >> 16) & 255;
    values[46] = (end_ms >> 8) & 255;
    values[47] = end_ms & 255;
    values[48] = values[8];
    values[49] = (end_ms >> 32) & 255;
    values[50] = (end_ms >> 40) & 255;

    values[51] = 6241;
    values[52] = (values[51] >> 24) & 255;
    values[53] = (values[51] >> 16) & 255;
    values[54] = (values[51] >> 8) & 255;
    values[55] = values[51] & 255;

    values[56] = 6383;
    values[57] = values[56] & 255;
    values[58] = (values[56] >> 8) & 255;
    values[59] = (values[56] >> 16) & 255;
    values[60] = (values[56] >> 24) & 255;

    values[64] = window_env_bytes.len() as u64;
    values[65] = values[64] & 255;
    values[66] = (values[64] >> 8) & 255;
    values[69] = 0;
    values[70] = 0;
    values[71] = 0;

    let checksum_indexes = [
        18, 20, 26, 30, 38, 40, 42, 21, 27, 31, 35, 39, 41, 43, 22, 28, 32, 36, 23, 29, 33, 37, 44,
        45, 46, 47, 48, 49, 50, 24, 25, 52, 53, 54, 55, 57, 58, 59, 60, 65, 66, 70, 71,
    ];
    let checksum = checksum_indexes
        .into_iter()
        .fold(0u8, |accumulator, index| accumulator ^ values[index] as u8);

    let mut payload = vec![
        values[18] as u8,
        values[20] as u8,
        values[52] as u8,
        values[26] as u8,
        values[30] as u8,
        values[34] as u8,
        values[58] as u8,
        values[38] as u8,
        values[40] as u8,
        values[53] as u8,
        values[42] as u8,
        values[21] as u8,
        values[27] as u8,
        values[54] as u8,
        values[55] as u8,
        values[31] as u8,
        values[35] as u8,
        values[57] as u8,
        values[39] as u8,
        values[41] as u8,
        values[43] as u8,
        values[22] as u8,
        values[28] as u8,
        values[32] as u8,
        values[60] as u8,
        values[36] as u8,
        values[23] as u8,
        values[29] as u8,
        values[33] as u8,
        values[37] as u8,
        values[44] as u8,
        values[45] as u8,
        values[59] as u8,
        values[46] as u8,
        values[47] as u8,
        values[48] as u8,
        values[49] as u8,
        values[50] as u8,
        values[24] as u8,
        values[25] as u8,
        values[65] as u8,
        values[66] as u8,
        values[70] as u8,
        values[71] as u8,
    ];
    payload.extend_from_slice(window_env_bytes);
    payload.push(checksum);

    rc4_encrypt(&[121], &payload)
}

fn custom_encode(bytes: &[u8], alphabet: &[u8; 64]) -> String {
    let full_chunks = bytes.len() / 3;
    let remainder = bytes.len() % 3;
    let extra_chars = match remainder {
        0 => 0,
        1 => 2,
        _ => 3,
    };
    let mut output = String::with_capacity(full_chunks * 4 + extra_chars);

    for chunk in bytes.chunks(3) {
        let block = ((chunk[0] as u32) << 16)
            | ((chunk.get(1).copied().unwrap_or_default() as u32) << 8)
            | (chunk.get(2).copied().unwrap_or_default() as u32);
        output.push(alphabet[((block >> 18) & 0x3f) as usize] as char);
        output.push(alphabet[((block >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            output.push(alphabet[((block >> 6) & 0x3f) as usize] as char);
        }
        if chunk.len() > 2 {
            output.push(alphabet[(block & 0x3f) as usize] as char);
        }
    }
    output
}
