use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SEED_COUNTER: AtomicU64 = AtomicU64::new(0x9e37_79b9_7f4a_7c15);

#[derive(Debug, Clone)]
pub(crate) struct PseudoRandom {
    state: u64,
}

impl PseudoRandom {
    pub(crate) fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x4d59_5df4_d0f3_3173
            } else {
                seed
            },
        }
    }

    pub(crate) fn from_system() -> Self {
        let now = now_unix_nanos();
        let counter = SEED_COUNTER.fetch_add(0x9e37_79b9, Ordering::Relaxed);
        Self::new(now ^ counter)
    }

    pub(crate) fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub(crate) fn next_mod(&mut self, modulus: u32) -> u32 {
        if modulus == 0 {
            0
        } else {
            self.next_u32() % modulus
        }
    }

    pub(crate) fn fill_bytes(&mut self, bytes: &mut [u8]) {
        for byte in bytes {
            *byte = (self.next_u32() & 0xff) as u8;
        }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
}

pub(crate) fn now_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

pub(crate) fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

pub(crate) fn base36_u64(mut value: u64) -> String {
    if value == 0 {
        return "0".to_owned();
    }

    let mut digits = Vec::new();
    while value > 0 {
        let remainder = (value % 36) as u8;
        let ch = match remainder {
            0..=9 => (b'0' + remainder) as char,
            _ => (b'a' + (remainder - 10)) as char,
        };
        digits.push(ch);
        value /= 36;
    }
    digits.iter().rev().collect()
}

fn now_unix_nanos() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos() as u64)
        .unwrap_or_default()
}
