const AES_RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[rustfmt::skip]
const AES_S_BOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

fn encode_base64(bytes: &[u8]) -> String {
    let mut result = String::new();
    let mut index = 0usize;

    while index + 3 <= bytes.len() {
        let value = ((u32::from(bytes[index])) << 16)
            | ((u32::from(bytes[index + 1])) << 8)
            | u32::from(bytes[index + 2]);
        result.push(BASE64_ALPHABET[((value >> 18) & 63) as usize] as char);
        result.push(BASE64_ALPHABET[((value >> 12) & 63) as usize] as char);
        result.push(BASE64_ALPHABET[((value >> 6) & 63) as usize] as char);
        result.push(BASE64_ALPHABET[(value & 63) as usize] as char);
        index += 3;
    }

    match bytes.len() - index {
        1 => {
            let value = u32::from(bytes[index]);
            result.push(BASE64_ALPHABET[(value >> 2) as usize] as char);
            result.push(BASE64_ALPHABET[((value << 4) & 63) as usize] as char);
            result.push('=');
            result.push('=');
        }
        2 => {
            let value = ((u32::from(bytes[index])) << 8) | u32::from(bytes[index + 1]);
            result.push(BASE64_ALPHABET[(value >> 10) as usize] as char);
            result.push(BASE64_ALPHABET[((value >> 4) & 63) as usize] as char);
            result.push(BASE64_ALPHABET[((value << 2) & 63) as usize] as char);
            result.push('=');
        }
        _ => {}
    }

    result
}

fn pkcs7_pad(value: &[u8], block_size: usize) -> Vec<u8> {
    let padding = block_size - (value.len() % block_size);
    let mut result = value.to_vec();
    result.extend(std::iter::repeat_n(padding as u8, padding));
    result
}

fn rot_word(word: [u8; 4]) -> [u8; 4] {
    [word[1], word[2], word[3], word[0]]
}

fn sub_word(word: [u8; 4]) -> [u8; 4] {
    [
        AES_S_BOX[word[0] as usize],
        AES_S_BOX[word[1] as usize],
        AES_S_BOX[word[2] as usize],
        AES_S_BOX[word[3] as usize],
    ]
}

fn expand_aes128_key(key: &[u8; 16]) -> [u8; 176] {
    let mut expanded = [0u8; 176];
    expanded[..16].copy_from_slice(key);
    let mut bytes_generated = 16usize;
    let mut rcon_index = 0usize;

    while bytes_generated < expanded.len() {
        let mut temp = [
            expanded[bytes_generated - 4],
            expanded[bytes_generated - 3],
            expanded[bytes_generated - 2],
            expanded[bytes_generated - 1],
        ];

        if bytes_generated.is_multiple_of(16) {
            temp = sub_word(rot_word(temp));
            temp[0] ^= AES_RCON[rcon_index];
            rcon_index += 1;
        }

        for byte in temp {
            expanded[bytes_generated] = expanded[bytes_generated - 16] ^ byte;
            bytes_generated += 1;
        }
    }

    expanded
}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for index in 0..16 {
        state[index] ^= round_key[index];
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for byte in state.iter_mut() {
        *byte = AES_S_BOX[*byte as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let original = *state;
    state[0] = original[0];
    state[4] = original[4];
    state[8] = original[8];
    state[12] = original[12];

    state[1] = original[5];
    state[5] = original[9];
    state[9] = original[13];
    state[13] = original[1];

    state[2] = original[10];
    state[6] = original[14];
    state[10] = original[2];
    state[14] = original[6];

    state[3] = original[15];
    state[7] = original[3];
    state[11] = original[7];
    state[15] = original[11];
}

fn xtime(value: u8) -> u8 {
    if value & 0x80 != 0 {
        (value << 1) ^ 0x1b
    } else {
        value << 1
    }
}

fn mix_single_column(column: &mut [u8; 4]) {
    let original = *column;
    let xor_all = original[0] ^ original[1] ^ original[2] ^ original[3];

    column[0] ^= xor_all ^ xtime(original[0] ^ original[1]);
    column[1] ^= xor_all ^ xtime(original[1] ^ original[2]);
    column[2] ^= xor_all ^ xtime(original[2] ^ original[3]);
    column[3] ^= xor_all ^ xtime(original[3] ^ original[0]);
}

fn mix_columns(state: &mut [u8; 16]) {
    for column_index in 0..4 {
        let start = column_index * 4;
        let mut column = [
            state[start],
            state[start + 1],
            state[start + 2],
            state[start + 3],
        ];
        mix_single_column(&mut column);
        state[start..start + 4].copy_from_slice(&column);
    }
}

fn aes128_encrypt_block(block: &[u8; 16], expanded_key: &[u8; 176]) -> [u8; 16] {
    let mut state = *block;

    add_round_key(&mut state, &expanded_key[..16]);

    for round in 1..10 {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &expanded_key[round * 16..(round + 1) * 16]);
    }

    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &expanded_key[160..176]);

    state
}

fn aes128_cbc_encrypt_pkcs7(plaintext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    let expanded_key = expand_aes128_key(key);
    let padded = pkcs7_pad(plaintext, 16);
    let mut previous = *iv;
    let mut result = Vec::with_capacity(padded.len());

    for chunk in padded.chunks_exact(16) {
        let mut block = [0u8; 16];

        for index in 0..16 {
            block[index] = chunk[index] ^ previous[index];
        }

        let encrypted = aes128_encrypt_block(&block, &expanded_key);
        result.extend_from_slice(&encrypted);
        previous = encrypted;
    }

    result
}

pub(super) fn encrypt_kuaishou_anonymous_kww_seed(seed: &str, key: &[u8; 16]) -> String {
    let encrypted = aes128_cbc_encrypt_pkcs7(seed.as_bytes(), key, key);
    encode_base64(&encrypted)
}
