use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_update_video_uri(conn: &MutexGuard<Connection>, id: u32, uri: &str) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET uri = ? WHERE id = ?",
        params![uri, id],
        |r| Ok(()),
    )
}