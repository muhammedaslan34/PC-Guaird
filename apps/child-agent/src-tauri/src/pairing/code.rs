#![allow(dead_code)]

use rand::Rng;
use std::time::{Duration, SystemTime};

// Excludes I, O, 0, 1 to avoid visual confusion when reading aloud or typing
const PAIRING_CODE_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
const PAIRING_CODE_LEN: usize = 6;
const PAIRING_CODE_TTL_SECS: u64 = 300; // 5 minutes

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairingCode {
    pub code: String,
    pub expires_at: SystemTime,
}

impl PairingCode {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let code: String = (0..PAIRING_CODE_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..PAIRING_CODE_CHARS.len());
                PAIRING_CODE_CHARS[idx] as char
            })
            .collect();
        Self {
            code,
            expires_at: SystemTime::now() + Duration::from_secs(PAIRING_CODE_TTL_SECS),
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() >= self.expires_at
    }
}
