use crate::error::AppError;

const CUSTOM_BASE64_ALPHABET: &[u8; 64] =
    b"ZmserbBoHQtNP+wOcza/LpngG8yJq42KWYj0DSfdikx3VT16IlUAFM97hECvuRX5";
const X3_BASE64_ALPHABET: &[u8; 64] =
    b"MfgqrsbcyzPQRStuvC7mn501HIJBo2DEFTKdeNOwxWXYZap89+/A4UVLhijkl63G";

pub(super) fn encode_custom_base64(bytes: &[u8]) -> String {
    encode_base64_with_alphabet(bytes, CUSTOM_BASE64_ALPHABET)
}

pub(super) fn decode_custom_base64(value: &str) -> Result<Vec<u8>, AppError> {
    decode_base64_with_alphabet(value, CUSTOM_BASE64_ALPHABET, "xiaohongshu base64")
}

pub(super) fn encode_x3_base64(bytes: &[u8]) -> String {
    encode_base64_with_alphabet(bytes, X3_BASE64_ALPHABET)
}

pub(super) fn decode_x3_base64(value: &str) -> Result<Vec<u8>, AppError> {
    decode_base64_with_alphabet(value, X3_BASE64_ALPHABET, "xiaohongshu x3 base64")
}

pub(super) fn crc32_js_int(input: &str) -> i32 {
    crc32_js_unsigned(input) as i32
}

fn encode_base64_with_alphabet(bytes: &[u8], alphabet: &[u8; 64]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(bytes.len().div_ceil(3) * 4);
    let mut index = 0;

    while index < bytes.len() {
        let remaining = bytes.len() - index;
        let b0 = bytes[index];
        let b1 = if remaining > 1 { bytes[index + 1] } else { 0 };
        let b2 = if remaining > 2 { bytes[index + 2] } else { 0 };
        let block = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        output.push(alphabet[((block >> 18) & 0x3f) as usize] as char);
        output.push(alphabet[((block >> 12) & 0x3f) as usize] as char);
        output.push(if remaining > 1 {
            alphabet[((block >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        output.push(if remaining > 2 {
            alphabet[(block & 0x3f) as usize] as char
        } else {
            '='
        });

        index += 3;
    }

    output
}

fn decode_base64_with_alphabet(
    value: &str,
    alphabet: &[u8; 64],
    label: &str,
) -> Result<Vec<u8>, AppError> {
    if value.is_empty() {
        return Ok(Vec::new());
    }
    if value.len() % 4 != 0 {
        return Err(AppError::InvalidRequestConfig(format!(
            "invalid {label} length: {value}"
        )));
    }

    let mut reverse = [u8::MAX; 256];
    for (index, byte) in alphabet.iter().enumerate() {
        reverse[*byte as usize] = index as u8;
    }

    let mut output = Vec::with_capacity((value.len() / 4) * 3);
    for chunk in value.as_bytes().chunks_exact(4) {
        let mut sextets = [0u8; 4];
        let mut padding = 0usize;

        for (index, byte) in chunk.iter().enumerate() {
            if *byte == b'=' {
                sextets[index] = 0;
                padding += 1;
                continue;
            }

            let mapped = reverse[*byte as usize];
            if mapped == u8::MAX {
                return Err(AppError::InvalidRequestConfig(format!(
                    "invalid {label} character `{}`",
                    *byte as char
                )));
            }
            sextets[index] = mapped;
        }

        let block = ((sextets[0] as u32) << 18)
            | ((sextets[1] as u32) << 12)
            | ((sextets[2] as u32) << 6)
            | (sextets[3] as u32);

        output.push(((block >> 16) & 0xff) as u8);
        if padding < 2 {
            output.push(((block >> 8) & 0xff) as u8);
        }
        if padding < 1 {
            output.push((block & 0xff) as u8);
        }
    }

    Ok(output)
}

fn crc32_js_unsigned(input: &str) -> u32 {
    let mut table = [0u32; 256];
    for (index, entry) in table.iter_mut().enumerate() {
        let mut value = index as u32;
        for _ in 0..8 {
            value = if value & 1 == 1 {
                (value >> 1) ^ 0xedb8_8320
            } else {
                value >> 1
            };
        }
        *entry = value;
    }

    let mut state = u32::MAX;
    for byte in input.as_bytes() {
        let lookup = ((state as u8) ^ *byte) as usize;
        state = table[lookup] ^ (state >> 8);
    }
    (!state) ^ 0xedb8_8320
}
