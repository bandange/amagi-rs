mod defaults;

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub(crate) session_window_props_init_min: u32,
    pub(crate) session_window_props_init_max: u32,
    pub(crate) session_sequence_init_min: u32,
    pub(crate) session_sequence_init_max: u32,
    pub(crate) session_sequence_step_min: u32,
    pub(crate) session_sequence_step_max: u32,
    pub(crate) session_window_props_step_min: u32,
    pub(crate) session_window_props_step_max: u32,
    pub(crate) hex_chars: &'static str,
    pub(crate) xray_trace_id_seq_max: u32,
    pub(crate) xray_trace_id_timestamp_shift: u32,
    pub(crate) xray_trace_id_part1_length: usize,
    pub(crate) xray_trace_id_part2_length: usize,
    pub(crate) b3_trace_id_length: usize,
}
