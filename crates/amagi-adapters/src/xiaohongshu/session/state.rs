/// Immutable session state carried into one Xiaohongshu signing operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XiaohongshuSignState {
    /// Page load timestamp in milliseconds.
    pub page_load_timestamp: u64,
    /// Monotonic sequence counter.
    pub sequence_value: u32,
    /// Simulated browser window property length.
    pub window_props_length: u32,
    /// URI length for the current request.
    pub uri_length: u32,
}
