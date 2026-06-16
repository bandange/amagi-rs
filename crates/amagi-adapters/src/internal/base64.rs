const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub(crate) fn encode_base64(bytes: &[u8]) -> String {
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

        output.push(BASE64_TABLE[((block >> 18) & 0x3f) as usize] as char);
        output.push(BASE64_TABLE[((block >> 12) & 0x3f) as usize] as char);
        output.push(if remaining > 1 {
            BASE64_TABLE[((block >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        output.push(if remaining > 2 {
            BASE64_TABLE[(block & 0x3f) as usize] as char
        } else {
            '='
        });

        index += 3;
    }

    output
}
