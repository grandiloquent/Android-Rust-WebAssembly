
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub fn get_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
