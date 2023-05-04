use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_update_video_duration(conn: &MutexGuard<Connection>, uri: &str, duration: u32) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET duration = ? WHERE uri = ?",
        params![duration,uri ],
        |_r| Ok(()),
    )
}