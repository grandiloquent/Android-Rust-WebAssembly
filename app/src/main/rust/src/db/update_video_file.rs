use crate::utils::date::get_epoch_secs;
use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_update_video_file(
    conn: &MutexGuard<Connection>,
    uri: &str,
    file: &str,
) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET file = ?,update_at = ? WHERE uri = ?",
        params![uri, file, get_epoch_secs()],
        |_r| Ok(()),
    )
}
