use crate::error::AppError;
use crate::platforms::internal::random::PseudoRandom;
use crate::platforms::internal::{md5::md5_hex, rc4::rc4_encrypt};
use crate::platforms::xiaohongshu::OrderedJson;

use super::codec::encode_custom_base64;

const B1_SECRET_KEY: &[u8] = b"xhswebmplfbt";
const URL_SAFE_BYTES: &[u8] = b"!*'()~_-";

pub(super) fn build_b1_value(random: &mut PseudoRandom, now_ms: u64) -> Result<String, AppError> {
    let payload = build_b1_payload(random, now_ms)?;
    let payload_json = payload.to_json_string()?;
    let encrypted = rc4_encrypt(B1_SECRET_KEY, payload_json.as_bytes());
    let encoded = custom_url_encode_latin1(&encrypted);
    let byte_array = percent_encoded_bytes(&encoded)?;
    let json = OrderedJson::Array(
        byte_array
            .into_iter()
            .map(|value| OrderedJson::uint(value.into()))
            .collect(),
    )
    .to_json_string()?;

    Ok(encode_custom_base64(json.as_bytes()))
}

fn build_b1_payload(random: &mut PseudoRandom, now_ms: u64) -> Result<OrderedJson, AppError> {
    let mut entropy = [0u8; 32];
    random.fill_bytes(&mut entropy);
    let x52 = md5_hex(&entropy);
    let x36 = (random.next_mod(20) + 1).to_string();

    Ok(OrderedJson::object(vec![
        ("x33", OrderedJson::string("0")),
        ("x34", OrderedJson::string("0")),
        ("x35", OrderedJson::string("0")),
        ("x36", OrderedJson::string(x36)),
        (
            "x37",
            OrderedJson::string("0|0|0|0|0|0|0|0|0|1|0|0|0|0|0|0|0|0|1|0|0|0|0|0"),
        ),
        (
            "x38",
            OrderedJson::string(
                "0|0|1|0|1|0|0|0|0|0|1|0|1|0|1|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0|0",
            ),
        ),
        ("x39", OrderedJson::uint(0)),
        ("x42", OrderedJson::string("3.4.4")),
        ("x43", OrderedJson::string("742cc32c")),
        ("x44", OrderedJson::string(now_ms.to_string())),
        (
            "x45",
            OrderedJson::string("__SEC_CAV__1-1-1-1-1|__SEC_WSA__|"),
        ),
        ("x46", OrderedJson::string("false")),
        ("x48", OrderedJson::string("")),
        ("x49", OrderedJson::string("{list:[],type:}")),
        ("x50", OrderedJson::string("")),
        ("x51", OrderedJson::string("")),
        ("x52", OrderedJson::string(x52)),
        ("x82", OrderedJson::string("_0x17a2|_0x1954")),
    ]))
}

fn custom_url_encode_latin1(bytes: &[u8]) -> String {
    let mut output = String::new();

    for &byte in bytes {
        if byte.is_ascii_alphanumeric() || URL_SAFE_BYTES.contains(&byte) {
            output.push(byte as char);
        } else {
            output.push('%');
            output.push_str(&format!("{byte:02X}"));
        }
    }

    output
}

fn percent_encoded_bytes(value: &str) -> Result<Vec<u8>, AppError> {
    let mut bytes = Vec::new();

    for chunk in value.split('%').skip(1) {
        if chunk.len() < 2 {
            return Err(AppError::InvalidRequestConfig(format!(
                "invalid xiaohongshu b1 percent chunk `{chunk}`"
            )));
        }

        let byte = u8::from_str_radix(&chunk[..2], 16).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid xiaohongshu b1 percent byte `{}`: {error}",
                &chunk[..2]
            ))
        })?;
        bytes.push(byte);
        bytes.extend_from_slice(&chunk.as_bytes()[2..]);
    }

    Ok(bytes)
}
