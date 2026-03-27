use crate::error::AppError;

const XOR_CODE: u64 = 23_442_827_791_579;
const MASK_CODE: u64 = 2_251_799_813_685_247;
const MAX_AID: u64 = 1 << 51;
const BASE: u64 = 58;
const TABLE: &str = "FcwAPNKTMug3GV5Lj7EJnHpWsx4tb8haYeviqBz6rkCy12mUSDQX9RdoZf";

/// Convert a numeric `av` identifier into its `BV` representation.
#[doc(alias = "av2bv")]
pub fn av_to_bv(aid: u64) -> String {
    let mut bytes = ['B', 'V', '1', '0', '0', '0', '0', '0', '0', '0', '0', '0'];
    let mut index = bytes.len() - 1;
    let mut value = (MAX_AID | aid) ^ XOR_CODE;
    let table = TABLE.as_bytes();

    while value > 0 {
        bytes[index] = table[(value % BASE) as usize] as char;
        value /= BASE;
        index = index.saturating_sub(1);
    }

    bytes.swap(3, 9);
    bytes.swap(4, 7);
    bytes.iter().collect()
}

/// Convert a `BV` identifier back into its numeric `av` representation.
#[doc(alias = "bv2av")]
pub fn bv_to_av(bvid: &str) -> Result<u64, AppError> {
    let mut chars: Vec<char> = bvid.chars().collect();
    if chars.len() != 12 || chars[0] != 'B' || chars[1] != 'V' || chars[2] != '1' {
        return Err(AppError::InvalidRequestConfig(format!(
            "Invalid Bilibili BV id: {bvid}"
        )));
    }

    chars.swap(3, 9);
    chars.swap(4, 7);

    let mut value = 0u64;
    for ch in chars.into_iter().skip(3) {
        let index = TABLE.find(ch).ok_or_else(|| {
            AppError::InvalidRequestConfig(format!("Unknown BV character '{ch}' in {bvid}"))
        })?;
        value = value * BASE + index as u64;
    }

    Ok((value & MASK_CODE) ^ XOR_CODE)
}
