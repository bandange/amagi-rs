use crate::platforms::internal::random::{PseudoRandom, now_unix_ms};

use super::super::config::Config;
use super::state::XiaohongshuSignState;

/// Stateful session manager that simulates stable Xiaohongshu browser activity.
#[derive(Debug, Clone)]
pub struct XiaohongshuSession {
    config: Config,
    random: PseudoRandom,
    page_load_timestamp: u64,
    sequence_value: u32,
    window_props_length: u32,
}

impl Default for XiaohongshuSession {
    fn default() -> Self {
        Self::new()
    }
}

impl XiaohongshuSession {
    /// Create a new session with runtime randomness.
    pub fn new() -> Self {
        let config = Config::default();
        let mut random = PseudoRandom::from_system();
        let sequence_value = random_range(
            &mut random,
            config.session_sequence_init_min,
            config.session_sequence_init_max,
        );
        let window_props_length = random_range(
            &mut random,
            config.session_window_props_init_min,
            config.session_window_props_init_max,
        );

        Self {
            config,
            random,
            page_load_timestamp: now_unix_ms(),
            sequence_value,
            window_props_length,
        }
    }

    /// Create a deterministic session for tests and repeatable signing.
    pub fn with_seed(seed: u64, page_load_timestamp: u64) -> Self {
        let config = Config::default();
        let mut random = PseudoRandom::new(seed);
        let sequence_value = random_range(
            &mut random,
            config.session_sequence_init_min,
            config.session_sequence_init_max,
        );
        let window_props_length = random_range(
            &mut random,
            config.session_window_props_init_min,
            config.session_window_props_init_max,
        );

        Self {
            config,
            random,
            page_load_timestamp,
            sequence_value,
            window_props_length,
        }
    }

    /// Return the next signing state for a request URI.
    pub fn current_state(&mut self, uri: &str) -> XiaohongshuSignState {
        self.sequence_value += random_range(
            &mut self.random,
            self.config.session_sequence_step_min,
            self.config.session_sequence_step_max,
        );
        self.window_props_length += random_range(
            &mut self.random,
            self.config.session_window_props_step_min,
            self.config.session_window_props_step_max,
        );

        XiaohongshuSignState {
            page_load_timestamp: self.page_load_timestamp,
            sequence_value: self.sequence_value,
            window_props_length: self.window_props_length,
            uri_length: uri.len() as u32,
        }
    }
}

fn random_range(random: &mut PseudoRandom, min: u32, max: u32) -> u32 {
    if max <= min {
        min
    } else {
        min + random.next_mod(max - min + 1)
    }
}
