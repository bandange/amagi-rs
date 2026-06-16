use super::Config;

impl Default for Config {
    fn default() -> Self {
        Self {
            session_window_props_init_min: 1000,
            session_window_props_init_max: 2000,
            session_sequence_init_min: 15,
            session_sequence_init_max: 17,
            session_sequence_step_min: 0,
            session_sequence_step_max: 1,
            session_window_props_step_min: 1,
            session_window_props_step_max: 10,
            hex_chars: "abcdef0123456789",
            xray_trace_id_seq_max: 8_388_607,
            xray_trace_id_timestamp_shift: 23,
            xray_trace_id_part1_length: 16,
            xray_trace_id_part2_length: 16,
            b3_trace_id_length: 16,
        }
    }
}
