use super::{
    hudr::derive_kuaishou_hudr_body,
    primitives::{
        bytes_to_lower_hex, compute_kuaishou_lrc_hex, derive_kuaishou_b2sa, derive_kuaishou_cts,
        hex_to_signed_bytes, to_little_endian_hex, transform_kuaishou_he_hex, xor_byte_arrays,
    },
    types::{
        KuaishouHeContext, KuaishouHeResult, KuaishouHudrContext, KuaishouPureSignContext,
        KuaishouPureSignResult,
    },
};

const KUAISHOU_HE_HEADER_HEX: &str = "4B54";
const KUAISHOU_HE_VERSION_HEX: &str = "cda9";
const KUAISHOU_HE_STARTUP_MARKER_HEX: &str = "ab";
const KUAISHOU_HE_FIXED_BODY_HEX: &str = "0100000001";
const KUAISHOU_HE_INPUT_XOR_MASK: [u8; 4] = [45, 211, 69, 192];
const KUAISHOU_HE_COUNTER_XOR_MASK: u32 = 3_131_873_467;
const KUAISHOU_HE_TIME_XOR_MASK: u64 = 3_360_347_992;
const KUAISHOU_HE_TAIL_HEX: &str = "9b563eda7b563e";
const KUAISHOU_HE_RANDOM_MAX: u64 = 281_474_976_710_655;

/// Derive the hash field embedded inside `$HE_`.
pub fn derive_kuaishou_he_hash_field_hex(sign_input: &str, hudr_body: &str) -> String {
    let hash_input = format!("{sign_input}HUDR_{hudr_body}");
    let digest_hex = bytes_to_lower_hex(&derive_kuaishou_cts(&derive_kuaishou_b2sa(&hash_input)));
    let digest_prefix = &digest_hex[..8];

    bytes_to_lower_hex(&xor_byte_arrays(
        &hex_to_signed_bytes(digest_prefix),
        &KUAISHOU_HE_INPUT_XOR_MASK,
    ))
}

/// Derive the final `$HE_` payload.
pub fn derive_kuaishou_he_hex(context: &KuaishouHeContext) -> KuaishouHeResult {
    let random48 = (context.random_value * KUAISHOU_HE_RANDOM_MAX as f64).floor() as u64;
    let hash_field_hex = derive_kuaishou_he_hash_field_hex(&context.sign_input, &context.hudr_body);
    let time_xor = context.timestamp ^ KUAISHOU_HE_TIME_XOR_MASK;
    let pre_hex = [
        KUAISHOU_HE_HEADER_HEX.to_owned(),
        KUAISHOU_HE_VERSION_HEX.to_owned(),
        KUAISHOU_HE_STARTUP_MARKER_HEX.to_owned(),
        to_little_endian_hex(context.startup_random, 6),
        to_little_endian_hex(random48, 6),
        KUAISHOU_HE_FIXED_BODY_HEX.to_owned(),
        to_little_endian_hex(u64::from(context.count ^ KUAISHOU_HE_COUNTER_XOR_MASK), 4),
        hash_field_hex.clone(),
        to_little_endian_hex(time_xor, 6),
        KUAISHOU_HE_TAIL_HEX.to_owned(),
        compute_kuaishou_lrc_hex(KUAISHOU_HE_TAIL_HEX),
    ]
    .join("");
    let final_hex = transform_kuaishou_he_hex(&pre_hex, &compute_kuaishou_lrc_hex(&pre_hex));

    KuaishouHeResult {
        final_hex,
        hash_field_hex,
        pre_hex,
    }
}

/// Derive the full pure Kuaishou signature.
pub fn derive_kuaishou_pure_signature(context: &KuaishouPureSignContext) -> KuaishouPureSignResult {
    let hudr = derive_kuaishou_hudr_body(&KuaishouHudrContext {
        count: context.count,
        script_count: context.script_count,
        secs: context.secs.clone(),
    });
    let he = derive_kuaishou_he_hex(&KuaishouHeContext {
        count: context.count,
        hudr_body: hudr.body.clone(),
        random_value: context.random_value,
        sign_input: context.sign_input.clone(),
        startup_random: context.startup_random,
        timestamp: context.timestamp,
    });
    let sign_result = format!("{}$HE_{}", hudr.full, he.final_hex);

    KuaishouPureSignResult {
        hudr,
        he,
        sign_result,
    }
}
