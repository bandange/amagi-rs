use std::{
    backtrace::Backtrace,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

use super::types::{KuaishouPureRuntimeState, KuaishouSecsState};

const KUAISHOU_DEFAULT_CAT_VERSION: &str = "2";
const KUAISHOU_DEFAULT_COUNT: u32 = 100;
const KUAISHOU_SECS_STACK_LIMIT: usize = 100;

static PURE_RUNTIME_STATE: OnceLock<Mutex<KuaishouPureRuntimeState>> = OnceLock::new();

fn current_timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn capture_kuaishou_encode_stack() -> String {
    format!("{:?}", Backtrace::force_capture())
}

fn runtime_state_cell() -> &'static Mutex<KuaishouPureRuntimeState> {
    PURE_RUNTIME_STATE.get_or_init(|| {
        Mutex::new(KuaishouPureRuntimeState {
            cat_version: KUAISHOU_DEFAULT_CAT_VERSION.to_owned(),
            count: KUAISHOU_DEFAULT_COUNT,
            startup_random: current_timestamp_millis(),
        })
    })
}

/// Return the trailing stack fragment used by `SECS.s`.
pub fn derive_kuaishou_secs_stack_tail(stack: Option<&str>) -> String {
    let source = stack
        .map(str::to_owned)
        .unwrap_or_else(capture_kuaishou_encode_stack);
    let tail: Vec<char> = source
        .chars()
        .rev()
        .take(KUAISHOU_SECS_STACK_LIMIT)
        .collect();

    tail.into_iter().rev().collect()
}

/// Build the pure `SECS` state used by the Rust sign runtime.
pub fn derive_kuaishou_secs_state(count: u32, stack: Option<&str>) -> KuaishouSecsState {
    KuaishouSecsState {
        c: Some(count),
        s: Some(derive_kuaishou_secs_stack_tail(stack)),
    }
}

/// Return a snapshot of the process-wide pure sign runtime state.
pub fn get_kuaishou_pure_runtime_state() -> KuaishouPureRuntimeState {
    runtime_state_cell()
        .lock()
        .expect("kuaishou runtime state lock should not be poisoned")
        .clone()
}

pub(crate) fn reserve_kuaishou_runtime_state() -> KuaishouPureRuntimeState {
    let mut state = runtime_state_cell()
        .lock()
        .expect("kuaishou runtime state lock should not be poisoned");
    let snapshot = state.clone();
    state.count = state.count.wrapping_add(1);
    snapshot
}
