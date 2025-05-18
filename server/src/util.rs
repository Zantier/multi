use std::time::{SystemTime, UNIX_EPOCH};

/// Time since unix epoch (ms)
pub fn get_now() -> i32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i32
}
