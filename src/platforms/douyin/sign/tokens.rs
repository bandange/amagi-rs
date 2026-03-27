use crate::platforms::internal::random::{PseudoRandom, base36_u64, now_unix_ms};

const ALPHANUMERIC: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const VERIFY_FP_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Default desktop browser user-agent used by the original Douyin signer.
pub const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36";

/// Generate a browser-style Douyin `msToken`.
#[doc(alias = "Mstoken")]
pub fn generate_ms_token(length: usize) -> String {
    let mut random = PseudoRandom::from_system();
    let mut entropy = vec![0u8; length.max(1)];
    random.fill_bytes(&mut entropy);
    generate_ms_token_from_entropy(length, &entropy)
}

/// Generate a deterministic Douyin `msToken` from caller-supplied entropy.
pub fn generate_ms_token_from_entropy(length: usize, entropy: &[u8]) -> String {
    if length == 0 {
        return String::new();
    }

    let source = if entropy.is_empty() {
        &[0u8][..]
    } else {
        entropy
    };

    (0..length)
        .map(|index| {
            let byte = source[index % source.len()];
            ALPHANUMERIC[(byte as usize) % ALPHANUMERIC.len()] as char
        })
        .collect()
}

/// Generate a browser-style Douyin `verifyFp`.
#[doc(alias = "VerifyFpManager")]
pub fn generate_verify_fp() -> String {
    let mut random = PseudoRandom::from_system();
    let mut entropy = [0u8; 36];
    random.fill_bytes(&mut entropy);
    generate_verify_fp_with_entropy(now_unix_ms(), &entropy)
}

/// Generate a deterministic Douyin `verifyFp` from a fixed timestamp and entropy.
pub fn generate_verify_fp_with_entropy(timestamp_ms: u64, entropy: &[u8]) -> String {
    let source = if entropy.is_empty() {
        &[0u8][..]
    } else {
        entropy
    };
    let mut chars = ['\0'; 36];

    chars[8] = '_';
    chars[13] = '_';
    chars[18] = '_';
    chars[23] = '_';
    chars[14] = '4';

    let mut entropy_index = 0usize;
    for (index, slot) in chars.iter_mut().enumerate() {
        if *slot != '\0' {
            continue;
        }

        let byte = source[entropy_index % source.len()] as usize;
        entropy_index += 1;
        let random_index = byte % VERIFY_FP_CHARS.len();
        let effective_index = if index == 19 {
            (random_index & 3) | 8
        } else {
            random_index
        };
        *slot = VERIFY_FP_CHARS[effective_index] as char;
    }

    format!(
        "verify_{}_{}",
        base36_u64(timestamp_ms),
        chars.iter().collect::<String>()
    )
}
