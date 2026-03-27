use crate::platforms::internal::random::{PseudoRandom, base36_u64, now_unix_ms};

use super::super::config::Config;

pub(crate) fn generate_b3_trace_id(random: &mut PseudoRandom, config: &Config) -> String {
    let hex_chars = config.hex_chars.as_bytes();
    let mut output = String::with_capacity(config.b3_trace_id_length);
    for _ in 0..config.b3_trace_id_length {
        output.push(hex_chars[random.next_mod(hex_chars.len() as u32) as usize] as char);
    }
    output
}

pub(crate) fn generate_xray_trace_id(
    random: &mut PseudoRandom,
    config: &Config,
    timestamp_ms: Option<u64>,
    seq: Option<u32>,
) -> String {
    let timestamp = timestamp_ms.unwrap_or_else(now_unix_ms);
    let sequence = seq.unwrap_or_else(|| random.next_mod(config.xray_trace_id_seq_max + 1));
    let combined = ((timestamp as u128) << config.xray_trace_id_timestamp_shift) | sequence as u128;
    let part1 = format!(
        "{combined:0width$x}",
        width = config.xray_trace_id_part1_length
    );

    let hex_chars = config.hex_chars.as_bytes();
    let mut part2 = String::with_capacity(config.xray_trace_id_part2_length);
    for _ in 0..config.xray_trace_id_part2_length {
        part2.push(hex_chars[random.next_mod(hex_chars.len() as u32) as usize] as char);
    }

    format!("{part1}{part2}")
}

pub(crate) fn generate_search_id(timestamp_ms: u64, random_value: u32) -> String {
    format!(
        "{}{}",
        (timestamp_ms as u128) << 64,
        base36_u64(random_value as u64)
    )
}
