use reqwest::Url;

use crate::error::AppError;
use crate::platforms::douyin::sign::tokens::DEFAULT_USER_AGENT;
use crate::platforms::internal::base64::encode_base64;
use crate::platforms::internal::md5::{md5_digest, md5_hex};
use crate::platforms::internal::random::now_unix_secs;
use crate::platforms::internal::rc4::rc4_encrypt;

const BASE64_CHARSET: &[u8; 65] =
    b"Dkdpgh4ZKsQB80/Mfvw36XI1R25-WUAlEi7NLboqYTOPuzmFjJnryx9HVGcaStCe=";

/// Generate a browser-style Douyin `X-Bogus` signature.
pub fn generate_x_bogus(url: &str, user_agent: Option<&str>) -> Result<String, AppError> {
    generate_x_bogus_at(
        url,
        user_agent.unwrap_or(DEFAULT_USER_AGENT),
        now_unix_secs(),
    )
}

/// Generate a deterministic Douyin `X-Bogus` signature for a fixed timestamp.
pub fn generate_x_bogus_at(
    url: &str,
    user_agent: &str,
    timestamp_secs: u64,
) -> Result<String, AppError> {
    let parsed = Url::parse(url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Douyin URL for X-Bogus {url}: {error}"))
    })?;
    let url_path = match parsed.query() {
        Some(query) => format!("{}?{query}", parsed.path()),
        None => parsed.path().to_owned(),
    };

    let ua_rc4 = rc4_encrypt(&[0, 1, 12], user_agent.as_bytes());
    let md5_ua = md5_hex(encode_base64(&ua_rc4).as_bytes());
    let array1 = hex_to_bytes(&md5_ua)?;

    let array2 = md5_digest(&hex_to_bytes("d41d8cd98f00b204e9800998ecf8427e")?).to_vec();
    let url_encrypted = md5_encrypt(&url_path)?;

    let ct = 536_919_696u64;
    let mut data = vec![
        64,
        1,
        1,
        12,
        url_encrypted[14],
        url_encrypted[15],
        array2[14],
        array2[15],
        array1[14],
        array1[15],
        ((timestamp_secs >> 24) & 0xff) as u8,
        ((timestamp_secs >> 16) & 0xff) as u8,
        ((timestamp_secs >> 8) & 0xff) as u8,
        (timestamp_secs & 0xff) as u8,
        ((ct >> 24) & 0xff) as u8,
        ((ct >> 16) & 0xff) as u8,
        ((ct >> 8) & 0xff) as u8,
        (ct & 0xff) as u8,
    ];

    let checksum = data
        .iter()
        .copied()
        .reduce(|left, right| left ^ right)
        .unwrap_or(0);
    data.push(checksum);

    let mut even = Vec::new();
    let mut odd = Vec::new();
    for (index, byte) in data.into_iter().enumerate() {
        if index % 2 == 0 {
            even.push(byte);
        } else {
            odd.push(byte);
        }
    }

    even.extend(odd);
    let garbled = {
        let encrypted = rc4_encrypt(&[255], &even);
        let mut result = Vec::with_capacity(encrypted.len() + 2);
        result.push(2);
        result.push(255);
        result.extend(encrypted);
        result
    };

    Ok(encode_with_charset(&garbled))
}

/// Append an `X-Bogus` signature to a request URL.
pub fn build_signed_url_with_x_bogus(
    url: &str,
    user_agent: Option<&str>,
) -> Result<String, AppError> {
    let signature = generate_x_bogus(url, user_agent)?;
    let separator = if url.contains('?') { '&' } else { '?' };
    Ok(format!("{url}{separator}X-Bogus={signature}"))
}

fn md5_encrypt(url_path: &str) -> Result<Vec<u8>, AppError> {
    let first = md5_hex(url_path.as_bytes());
    let second = md5_digest(&hex_to_bytes(&first)?);
    Ok(second.to_vec())
}

fn hex_to_bytes(value: &str) -> Result<Vec<u8>, AppError> {
    if value.len() % 2 != 0 {
        return Err(AppError::InvalidRequestConfig(format!(
            "Invalid hexadecimal input length: {value}"
        )));
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    let chars = value.as_bytes();
    let mut index = 0usize;
    while index < chars.len() {
        let high = decode_hex(chars[index])?;
        let low = decode_hex(chars[index + 1])?;
        bytes.push((high << 4) | low);
        index += 2;
    }
    Ok(bytes)
}

fn decode_hex(value: u8) -> Result<u8, AppError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(AppError::InvalidRequestConfig(format!(
            "Invalid hexadecimal character: {}",
            value as char
        ))),
    }
}

fn encode_with_charset(bytes: &[u8]) -> String {
    let mut output = String::with_capacity((bytes.len() / 3) * 4);
    for chunk in bytes.chunks_exact(3) {
        let block = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
        output.push(BASE64_CHARSET[((block >> 18) & 0x3f) as usize] as char);
        output.push(BASE64_CHARSET[((block >> 12) & 0x3f) as usize] as char);
        output.push(BASE64_CHARSET[((block >> 6) & 0x3f) as usize] as char);
        output.push(BASE64_CHARSET[(block & 0x3f) as usize] as char);
    }
    output
}
