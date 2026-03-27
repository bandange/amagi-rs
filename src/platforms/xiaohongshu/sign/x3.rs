use crate::error::AppError;

use super::super::session::XiaohongshuSignState;
use super::codec::{decode_x3_base64, encode_x3_base64};
use super::types::XiaohongshuMnsv2Input;

pub(super) const X3_SIGNATURE_PREFIX: &str = "mns0301_";

const VERSION_BYTES: [u8; 4] = [121, 104, 96, 41];
const HEX_KEY: &str = "71a302257793271ddd273bcee3e4b98d9d7935e1da33f5765e2ea8afb6dc77a51a499d23b67c20660025860cbf13d4540d92497f58686c574e508f46e1956344f39139bf4faf22a3eef120b79258145b2feb5193b6478669961298e79bedca646e1a693a926154a5a7a1bd1cf0dedb742f917a747a1e388b234f2277516db7116035439730fa61e9822a0eca7bff72d8";
const HASH_IV: [u32; 4] = [1_831_565_813, 461_845_907, 2_246_822_507, 3_266_489_909];
const PAYLOAD_LENGTH: usize = 144;
const A1_LENGTH: usize = 52;
const APP_ID_LENGTH: usize = 10;
const TIMESTAMP_BYTES: usize = 8;
const A3_PREFIX: [u8; 4] = [2, 97, 51, 16];
const ENV_TABLE: [u8; 15] = [
    115, 248, 83, 102, 103, 201, 181, 131, 99, 94, 4, 68, 250, 132, 21,
];
const ENV_CHECKS_DEFAULT: [u8; 15] = [0, 1, 18, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0];

pub(super) fn build_x3_payload(
    sign_input: &XiaohongshuMnsv2Input,
    a1_value: &str,
    xsec_appid: &str,
    sign_state: XiaohongshuSignState,
    timestamp_ms: u64,
    random_word: u32,
) -> Result<Vec<u8>, AppError> {
    let seed_byte = (random_word & 0xff) as u8;
    let timestamp_bytes = int_to_le_bytes(timestamp_ms, TIMESTAMP_BYTES);
    let digest_prefix = decode_hex_prefix(&sign_input.md5_full, 8)?
        .into_iter()
        .map(|value| value ^ seed_byte)
        .collect::<Vec<_>>();
    let app_bytes = pad_utf8(xsec_appid, APP_ID_LENGTH);
    let a1_bytes = pad_utf8(a1_value, A1_LENGTH);
    let checksum = build_checksum_block(seed_byte);
    let a3 = build_a3_block(seed_byte, &timestamp_bytes, &sign_input.md5_path)?;

    let mut payload = Vec::with_capacity(PAYLOAD_LENGTH);
    payload.extend_from_slice(&VERSION_BYTES);
    payload.extend_from_slice(&(random_word as u64).to_le_bytes()[..4]);
    payload.extend_from_slice(&timestamp_bytes);
    payload.extend_from_slice(&int_to_le_bytes(
        sign_state.page_load_timestamp,
        TIMESTAMP_BYTES,
    ));
    payload.extend_from_slice(&(sign_state.sequence_value).to_le_bytes());
    payload.extend_from_slice(&(sign_state.window_props_length).to_le_bytes());
    payload.extend_from_slice(&(sign_state.uri_length).to_le_bytes());
    payload.extend_from_slice(&digest_prefix);
    payload.push(a1_bytes.len() as u8);
    payload.extend_from_slice(&a1_bytes);
    payload.push(app_bytes.len() as u8);
    payload.extend_from_slice(&app_bytes);
    payload.extend_from_slice(&checksum);
    payload.extend_from_slice(&a3);

    Ok(payload)
}

pub(super) fn sign_x3_payload(payload: &[u8]) -> Result<String, AppError> {
    let transformed = xor_with_hex_key(payload)?;
    Ok(format!(
        "{X3_SIGNATURE_PREFIX}{}",
        encode_x3_base64(&transformed)
    ))
}

pub(super) fn decode_x3_bytes(signature: &str) -> Result<Vec<u8>, AppError> {
    let encoded = signature
        .strip_prefix(X3_SIGNATURE_PREFIX)
        .ok_or_else(|| AppError::InvalidRequestConfig("invalid xiaohongshu x3 prefix".into()))?;
    let decoded = decode_x3_base64(encoded)?;
    xor_with_hex_key(&decoded)
}

fn build_checksum_block(seed_byte: u8) -> [u8; 16] {
    let mut block = [0u8; 16];
    block[0] = 1;
    block[1] = seed_byte ^ ENV_TABLE[0];

    for index in 1..ENV_TABLE.len() {
        block[index + 1] = ENV_TABLE[index] ^ ENV_CHECKS_DEFAULT[index];
    }

    block
}

fn build_a3_block(
    seed_byte: u8,
    timestamp_bytes: &[u8],
    md5_path: &str,
) -> Result<Vec<u8>, AppError> {
    let mut md5_path_bytes = decode_hex_prefix(md5_path, 16)?;
    let mut hash_input = Vec::with_capacity(timestamp_bytes.len() + md5_path_bytes.len());
    hash_input.extend_from_slice(timestamp_bytes);
    hash_input.append(&mut md5_path_bytes);

    let hash = custom_hash_v2(&hash_input);
    let mut block = Vec::with_capacity(20);
    block.extend_from_slice(&A3_PREFIX);
    block.extend(hash.into_iter().map(|value| value ^ seed_byte));
    Ok(block)
}

fn decode_hex_prefix(value: &str, byte_len: usize) -> Result<Vec<u8>, AppError> {
    let expected_hex_len = byte_len * 2;
    if value.len() < expected_hex_len {
        return Err(AppError::InvalidRequestConfig(format!(
            "invalid xiaohongshu hex input length `{value}`"
        )));
    }

    let mut decoded = Vec::with_capacity(byte_len);
    for index in (0..expected_hex_len).step_by(2) {
        let part = &value[index..index + 2];
        let byte = u8::from_str_radix(part, 16).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid xiaohongshu hex byte `{part}`: {error}"
            ))
        })?;
        decoded.push(byte);
    }

    Ok(decoded)
}

fn pad_utf8(value: &str, len: usize) -> Vec<u8> {
    let mut padded = vec![0u8; len];
    let bytes = value.as_bytes();
    let copy_len = bytes.len().min(len);
    padded[..copy_len].copy_from_slice(&bytes[..copy_len]);
    padded
}

fn xor_with_hex_key(input: &[u8]) -> Result<Vec<u8>, AppError> {
    let key = decode_hex_prefix(HEX_KEY, HEX_KEY.len() / 2)?;
    let mut output = Vec::with_capacity(input.len());

    for (index, byte) in input.iter().copied().enumerate() {
        output.push(if index < key.len() {
            byte ^ key[index]
        } else {
            byte
        });
    }

    Ok(output)
}

fn int_to_le_bytes(value: u64, byte_len: usize) -> Vec<u8> {
    (0..byte_len)
        .map(|offset| ((value >> (offset * 8)) & 0xff) as u8)
        .collect()
}

fn custom_hash_v2(input: &[u8]) -> Vec<u8> {
    let mut s0 = HASH_IV[0];
    let mut s1 = HASH_IV[1];
    let mut s2 = HASH_IV[2];
    let mut s3 = HASH_IV[3];
    let length = input.len() as u32;

    s0 ^= length;
    s1 ^= length << 8;
    s2 ^= length << 16;
    s3 ^= length << 24;

    for chunk in input.chunks_exact(8) {
        let v0 = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        let v1 = u32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);

        s0 = rotate_left((s0.wrapping_add(v0)) ^ s2, 7);
        s1 = rotate_left((v0 ^ s1).wrapping_add(s3), 11);
        s2 = rotate_left((s2.wrapping_add(v1)) ^ s0, 13);
        s3 = rotate_left((s3 ^ v1).wrapping_add(s1), 17);
    }

    let t0 = s0 ^ length;
    let t1 = s1 ^ t0;
    let t2 = s2.wrapping_add(t1);
    let t3 = s3 ^ t2;
    let rot_t0 = rotate_left(t0, 9);
    let rot_t1 = rotate_left(t1, 13);
    let rot_t2 = rotate_left(t2, 17);
    let rot_t3 = rotate_left(t3, 19);

    let s0 = rot_t0.wrapping_add(rot_t2);
    let s1 = rot_t1 ^ rot_t3;
    let s2 = rot_t2.wrapping_add(s0);
    let s3 = rot_t3 ^ s1;

    let mut output = Vec::with_capacity(16);
    output.extend_from_slice(&s0.to_le_bytes());
    output.extend_from_slice(&s1.to_le_bytes());
    output.extend_from_slice(&s2.to_le_bytes());
    output.extend_from_slice(&s3.to_le_bytes());
    output
}

fn rotate_left(value: u32, shift: u32) -> u32 {
    value.rotate_left(shift)
}
