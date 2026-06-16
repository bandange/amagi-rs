use super::types::{KuaishouHudrContext, KuaishouHudrResult};

const KUAISHOU_HUDR_PREFIX: &str = "HUDR_";
const KUAISHOU_HUDR_MASK_BYTE: u8 = 35;
const KUAISHOU_HUDR_BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const KUAISHOU_HUDR_CHACHA_KEY: [u32; 8] = [
    4_183_807_412,
    394_484_062,
    1_106_561_997,
    2_378_328_696,
    630_790_222,
    2_546_784_104,
    2_891_127_470,
    1_922_531_795,
];
const KUAISHOU_HUDR_CHACHA_NONCE: [u32; 3] = [2_215_853_858, 1_643_070_585, 1_849_059_804];

fn to_code_point_array(value: &str) -> Vec<u32> {
    value.chars().map(|character| character as u32).collect()
}

fn to_little_endian_bytes(value: u64, size: usize) -> Vec<u8> {
    if size >= 4 && value >= (1u64 << 32) {
        return vec![255; size];
    }

    (0..size)
        .map(|index| ((value >> (8 * index)) & 255) as u8)
        .collect()
}

fn encode_base64(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut index = 0usize;

    while index + 3 <= bytes.len() {
        let value = ((u32::from(bytes[index])) << 16)
            | ((u32::from(bytes[index + 1])) << 8)
            | u32::from(bytes[index + 2]);
        result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value >> 18) & 63) as usize] as char);
        result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value >> 12) & 63) as usize] as char);
        result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value >> 6) & 63) as usize] as char);
        result.push(KUAISHOU_HUDR_BASE64_ALPHABET[(value & 63) as usize] as char);
        index += 3;
    }

    match bytes.len() - index {
        1 => {
            let value = u32::from(bytes[index]);
            result.push(KUAISHOU_HUDR_BASE64_ALPHABET[(value >> 2) as usize] as char);
            result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value << 4) & 63) as usize] as char);
            result.push('=');
            result.push('=');
        }
        2 => {
            let value = ((u32::from(bytes[index])) << 8) | u32::from(bytes[index + 1]);
            result.push(KUAISHOU_HUDR_BASE64_ALPHABET[(value >> 10) as usize] as char);
            result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value >> 4) & 63) as usize] as char);
            result.push(KUAISHOU_HUDR_BASE64_ALPHABET[((value << 2) & 63) as usize] as char);
            result.push('=');
        }
        _ => {}
    }

    result
}

fn encode_base64_url(bytes: &[u8]) -> String {
    encode_base64(bytes)
        .replace('+', "-")
        .replace('/', "_")
        .replace('=', ".")
}

#[derive(Debug, Clone)]
struct KuaishouChaChaCipher {
    word_index: usize,
    state: [u32; 16],
    key: [u32; 8],
    nonce: [u32; 3],
}

impl KuaishouChaChaCipher {
    fn new(key: [u32; 8], nonce: [u32; 3]) -> Self {
        Self {
            word_index: 0,
            state: [0; 16],
            key,
            nonce,
        }
    }

    fn rotate_left(value: u32, shift: u32) -> u32 {
        value.rotate_left(shift)
    }

    fn quarter_round(target: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        target[a] = target[a].wrapping_add(target[b]);
        target[d] ^= target[a];
        target[d] = Self::rotate_left(target[d], 16);

        target[c] = target[c].wrapping_add(target[d]);
        target[b] ^= target[c];
        target[b] = Self::rotate_left(target[b], 12);

        target[a] = target[a].wrapping_add(target[b]);
        target[d] ^= target[a];
        target[d] = Self::rotate_left(target[d], 8);

        target[c] = target[c].wrapping_add(target[d]);
        target[b] ^= target[c];
        target[b] = Self::rotate_left(target[b], 7);
    }

    fn refill_block(&self) -> [u32; 16] {
        let mut working = self.state;

        for _ in (0..20).step_by(2) {
            Self::quarter_round(&mut working, 0, 4, 8, 12);
            Self::quarter_round(&mut working, 1, 5, 9, 13);
            Self::quarter_round(&mut working, 2, 6, 10, 14);
            Self::quarter_round(&mut working, 3, 7, 11, 15);
            Self::quarter_round(&mut working, 0, 5, 10, 15);
            Self::quarter_round(&mut working, 1, 6, 11, 12);
            Self::quarter_round(&mut working, 2, 7, 8, 13);
            Self::quarter_round(&mut working, 3, 4, 9, 14);
        }

        let mut mixed_state = [0u32; 16];

        for index in 0..16 {
            mixed_state[index] = working[index].wrapping_add(self.state[index]);
        }

        mixed_state
    }

    fn encrypt(&mut self, input: &[u8]) -> Vec<u8> {
        self.word_index = 0;
        self.state[0] = 394_484_062;
        self.state[1] = 2_378_328_696;
        self.state[2] = 630_790_222;
        self.state[3] = 1_922_531_795;

        for index in 0..8 {
            self.state[index + 4] = self.key[index];
        }

        self.state[12] = 1;
        self.state[13] = self.nonce[0];
        self.state[14] = self.nonce[1];
        self.state[15] = self.nonce[2];

        let mut mixed_state = self.refill_block();
        let mut output = vec![0u8; input.len()];

        for (index, value) in input.iter().enumerate() {
            if self.word_index == 64 {
                self.state[12] = self.state[12].wrapping_add(1);
                mixed_state = self.refill_block();
                self.word_index = 0;
            }

            let word = mixed_state[self.word_index >> 2];
            let keystream_byte = ((word >> ((self.word_index & 3) << 3)) & 255) as u8;
            self.word_index += 1;
            output[index] = value ^ keystream_byte;
        }

        output
    }
}

/// Build the `infoCache` field embedded in `HUDR_`.
pub fn build_kuaishou_hudr_info_cache(script_count: Option<u32>) -> Vec<u8> {
    let count = u64::from(script_count.unwrap_or(0));
    [vec![68, 0], to_little_endian_bytes(count, 4)].concat()
}

/// Build the raw Kuaishou `HUDR_` payload.
pub fn build_kuaishou_hudr_payload(context: &KuaishouHudrContext) -> Vec<u8> {
    let stack_tail = context
        .secs
        .as_ref()
        .and_then(|state| state.s.as_deref())
        .unwrap_or_default();
    let secs_count = context.secs.as_ref().and_then(|state| state.c).unwrap_or(0);
    let stack_tail_length = stack_tail.encode_utf16().count() as u64;
    let mut payload = vec![45, 61, 0, 2];

    payload.extend(build_kuaishou_hudr_info_cache(context.script_count));
    payload.extend([112, 0]);
    payload.extend(to_little_endian_bytes(u64::from(context.count), 4));
    payload.extend([114, 1]);
    payload.extend(to_little_endian_bytes(stack_tail_length, 2));
    payload.extend(
        to_code_point_array(stack_tail)
            .into_iter()
            .map(|value| value as u8),
    );
    payload.extend([115, 0]);
    payload.extend(to_little_endian_bytes(u64::from(secs_count), 4));

    payload
}

/// Apply the page mask used before `HUDR_` encryption.
pub fn mask_kuaishou_hudr_payload(payload: &[u8]) -> Vec<u8> {
    payload
        .iter()
        .map(|value| KUAISHOU_HUDR_MASK_BYTE ^ value)
        .collect()
}

/// Derive the complete `HUDR_` result from the provided context.
pub fn derive_kuaishou_hudr_body(context: &KuaishouHudrContext) -> KuaishouHudrResult {
    let masked_payload = mask_kuaishou_hudr_payload(&build_kuaishou_hudr_payload(context));
    let mut cipher =
        KuaishouChaChaCipher::new(KUAISHOU_HUDR_CHACHA_KEY, KUAISHOU_HUDR_CHACHA_NONCE);
    let encrypted = cipher.encrypt(&masked_payload);
    let body = encode_base64_url(&encrypted);

    KuaishouHudrResult {
        body: body.clone(),
        full: format!("{KUAISHOU_HUDR_PREFIX}{body}"),
        info_cache: build_kuaishou_hudr_info_cache(context.script_count),
        masked_payload,
        next_count: context.count.wrapping_add(1),
    }
}
