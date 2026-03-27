const KUAISHOU_BLAKE2S_IV: [u32; 8] = [
    2_837_534_710,
    2_845_986_804,
    2_436_420_605,
    706_843_635,
    719_254_516,
    2_557_931_286,
    2_596_197_199,
    2_432_949_778,
];

const KUAISHOU_BLAKE2S_SIGMA: [[usize; 16]; 10] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
    [14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
    [11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
    [7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
    [9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
    [2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
    [12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
    [13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
    [6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
    [10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
];

const KUAISHOU_CTS_STATE_VECTOR: [i8; 48] = [
    98, 0, 0, -128, 49, 117, -71, -3, -32, -84, 104, 36, -33, -101, 87, 19, 32, 0, 0, 64, 2, 0, 0,
    16, -1, -1, -1, 127, -1, -1, -1, 63, 0, 0, 0, -16, 0, 0, 0, -64, 0, 0, 0, -128, -1, -1, -1, 15,
];

fn rotate_right(value: u32, shift: u32) -> u32 {
    value.rotate_right(shift)
}

fn to_utf8_i8_array(value: &str) -> Vec<i8> {
    value.as_bytes().iter().map(|byte| *byte as i8).collect()
}

fn to_hex32(value: u32) -> String {
    format!("{value:08x}")
}

fn blake2s_quarter_round(
    state: &mut [u32; 16],
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    x: u32,
    y: u32,
) {
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(x);
    state[d] = rotate_right(state[d] ^ state[a], 16);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = rotate_right(state[b] ^ state[c], 12);
    state[a] = state[a].wrapping_add(state[b]).wrapping_add(y);
    state[d] = rotate_right(state[d] ^ state[a], 8);
    state[c] = state[c].wrapping_add(state[d]);
    state[b] = rotate_right(state[b] ^ state[c], 7);
}

fn blake2s_compress(
    hash: &mut [u32; 8],
    words: &[u32],
    offset: usize,
    counter: u32,
    length: usize,
    is_last_block: bool,
) {
    let mut work = [0u32; 16];
    let mut block = [0u32; 16];

    for index in 0..8 {
        work[index] = hash[index];
        work[index + 8] = KUAISHOU_BLAKE2S_IV[index];
    }

    work[12] ^= counter;

    if is_last_block {
        work[14] ^= u32::MAX;
    }

    for index in 0..length {
        block[index % 16] ^= words[offset + index];
    }

    for sigma in KUAISHOU_BLAKE2S_SIGMA {
        blake2s_quarter_round(&mut work, 0, 4, 8, 12, block[sigma[0]], block[sigma[1]]);
        blake2s_quarter_round(&mut work, 1, 5, 9, 13, block[sigma[2]], block[sigma[3]]);
        blake2s_quarter_round(&mut work, 2, 6, 10, 14, block[sigma[4]], block[sigma[5]]);
        blake2s_quarter_round(&mut work, 3, 7, 11, 15, block[sigma[6]], block[sigma[7]]);
        blake2s_quarter_round(&mut work, 0, 5, 10, 15, block[sigma[8]], block[sigma[9]]);
        blake2s_quarter_round(&mut work, 1, 6, 11, 12, block[sigma[10]], block[sigma[11]]);
        blake2s_quarter_round(&mut work, 2, 7, 8, 13, block[sigma[12]], block[sigma[13]]);
        blake2s_quarter_round(&mut work, 3, 4, 9, 14, block[sigma[14]], block[sigma[15]]);
    }

    for index in 0..8 {
        hash[index] ^= work[index] ^ work[index + 8];
    }
}

fn derive_b2has_words(value: &str) -> Vec<u32> {
    let utf8 = to_utf8_i8_array(value);
    let padding = match utf8.len() % 4 {
        0 => 0,
        remainder => 4 - remainder,
    };
    let mut padded = vec![0u8; utf8.len() + padding];

    for (index, byte) in utf8.iter().enumerate() {
        padded[index] = *byte as u8;
    }

    let mut words = Vec::with_capacity(padded.len() / 4);

    for chunk in padded.chunks_exact(4) {
        words.push(i32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]) as u32);
    }

    words
}

fn derive_b2has_hash(words: &[u32]) -> [u32; 8] {
    let mut hash = KUAISHOU_BLAKE2S_IV;
    hash[0] ^= 16_842_784;

    let mut offset = 0usize;
    let mut length = words.len();
    let mut counter = 0u32;

    while length > 64 {
        length -= 64;
        counter = counter.wrapping_add(64);
        blake2s_compress(&mut hash, words, offset, counter, 64, false);
        offset += 64;
    }

    blake2s_compress(
        &mut hash,
        words,
        offset,
        counter.wrapping_add(length as u32),
        length,
        true,
    );

    hash
}

/// Derive the Kuaishou page `b2has` value.
pub fn derive_kuaishou_b2has(value: &str) -> String {
    derive_b2has_hash(&derive_b2has_words(value))
        .into_iter()
        .map(to_hex32)
        .collect()
}

/// Convert `b2has` into the signed byte array used by the page.
pub fn derive_kuaishou_b2sa(value: &str) -> Vec<i8> {
    to_utf8_i8_array(&derive_kuaishou_b2has(value))
}

#[derive(Debug, Clone, Copy)]
struct KuaishouCtsState {
    e: i32,
    b: i32,
    c: i32,
    d: i32,
    f: i32,
    h: i32,
    l: i32,
    m: i32,
    p: i32,
    s: i32,
    u: i32,
    y: i32,
}

fn read_i32_le(bytes: &[i8]) -> i32 {
    i32::from_le_bytes([
        bytes[0] as u8,
        bytes[1] as u8,
        bytes[2] as u8,
        bytes[3] as u8,
    ])
}

fn create_kuaishou_cts_state() -> KuaishouCtsState {
    KuaishouCtsState {
        s: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[12..16]),
        u: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[8..12]),
        c: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[4..8]),
        l: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[0..4]),
        p: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[16..20]),
        f: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[20..24]),
        d: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[24..28]),
        y: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[28..32]),
        h: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[44..48]),
        e: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[40..44]),
        m: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[36..40]),
        b: read_i32_le(&KUAISHOU_CTS_STATE_VECTOR[32..36]),
    }
}

fn seed_kuaishou_cts_state(state: &mut KuaishouCtsState, seed: &str) {
    let chars: Vec<i8> = seed
        .chars()
        .map(|character| character as u32 as u8 as i8)
        .collect();

    for index in 0..4 {
        let value = i32::from(chars[index + 4]);
        state.s = state.s.wrapping_shl(8) | value;
        state.u = state.u.wrapping_shl(8) | value;
        state.c = state.c.wrapping_shl(8) | value;
    }

    if state.s == 0 {
        state.s = 324_508_639;
    }

    if state.u == 0 {
        state.u = 610_839_776;
    }

    if state.c == 0 {
        state.c = 4_256_789_809u32 as i32;
    }
}

fn derive_kuaishou_cts_byte(state: &mut KuaishouCtsState, value: i8) -> i8 {
    let mut result = 0i32;
    let mut right_bit = state.u & 1;
    let mut left_bit = state.c & 1;

    for _ in 0..8 {
        if state.s & 1 != 0 {
            state.s = (state.s ^ (state.l >> 1)) | state.e;

            if state.u & 1 != 0 {
                state.u = (state.u ^ (state.p >> 1)) | state.m;
                right_bit = 1;
            } else {
                state.u = (state.u >> 1) & state.y;
                right_bit = 0;
            }
        } else {
            state.s = (state.s >> 1) & state.d;

            if state.c & 1 != 0 {
                state.c = (state.c ^ (state.f >> 1)) | state.b;
                left_bit = 1;
            } else {
                state.c = (state.c >> 1) & state.h;
                left_bit = 0;
            }
        }

        let mixed = result.wrapping_shl(1) | (right_bit ^ left_bit);
        result = if mixed > 127 {
            mixed - 256
        } else if mixed < -128 {
            mixed + 256
        } else {
            mixed
        };
    }

    ((i32::from(value)) ^ (result + 3)) as i8
}

/// Derive the Kuaishou `cts` byte stream transform.
pub fn derive_kuaishou_cts(input: &[i8]) -> Vec<i8> {
    let mut state = create_kuaishou_cts_state();
    seed_kuaishou_cts_state(&mut state, "Vuz4fCHxn1CO");

    input
        .iter()
        .map(|byte| derive_kuaishou_cts_byte(&mut state, *byte))
        .collect()
}

/// Encode bytes as lowercase hexadecimal.
pub fn bytes_to_lower_hex<T>(bytes: &[T]) -> String
where
    T: Copy + Into<i32>,
{
    let mut result = String::with_capacity(bytes.len() * 2);

    for byte in bytes {
        let value = ((*byte).into() & 255) as u8;
        result.push_str(&format!("{value:02x}"));
    }

    result
}

/// Decode a hexadecimal string into signed bytes.
pub fn hex_to_signed_bytes(value: &str) -> Vec<i8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|chunk| {
            let text = std::str::from_utf8(chunk).expect("hex bytes should be valid utf-8");
            u8::from_str_radix(text, 16).expect("hex bytes should be valid") as i8
        })
        .collect()
}

/// XOR the left byte array with the right byte array in a repeating cycle.
pub fn xor_byte_arrays<L, R>(left: &[L], right: &[R]) -> Vec<i8>
where
    L: Copy + Into<i32>,
    R: Copy + Into<i32>,
{
    let mut result = Vec::with_capacity(left.len());

    for (index, value) in left.iter().enumerate() {
        let mask = right[index % right.len()].into() & 255;
        result.push(((*value).into() ^ mask) as i8);
    }

    result
}

/// Encode a value as fixed-size little-endian hexadecimal.
pub fn to_little_endian_hex(value: u64, size: usize) -> String {
    let mut result = String::with_capacity(size * 2);

    for index in 0..size {
        let byte = ((value >> (8 * index)) & 255) as u8;
        result.push_str(&format!("{byte:02x}"));
    }

    result
}

/// Compute the page LRC checksum used by `$HE_`.
pub fn compute_kuaishou_lrc_hex(source_hex: &str) -> String {
    let sum = hex_to_signed_bytes(source_hex)
        .into_iter()
        .map(|value| i32::from(value) & 255)
        .sum::<i32>();

    format!("{:02x}", ((-sum) & 255) as u8)
}

/// Apply the final Kuaishou `$HE_` XOR transform.
pub fn transform_kuaishou_he_hex(prefix_hex: &str, checksum_hex: &str) -> String {
    let input = hex_to_signed_bytes(&format!("{prefix_hex}{checksum_hex}"));

    if input.is_empty() {
        return String::new();
    }

    let xor_key = *input.last().expect("input should contain checksum");
    let mut output = vec![0i8; input.len()];

    for index in 0..input.len() - 1 {
        output[index] = (i32::from(input[index]) ^ i32::from(xor_key)) as i8;
    }

    output[input.len() - 1] = xor_key;
    bytes_to_lower_hex(&output)
}
