use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now() -> i32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i32
}
