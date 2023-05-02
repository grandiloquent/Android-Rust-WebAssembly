use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_delete_video(conn: &MutexGuard<Connection>, id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM video WHERE id = ?", params![id])
}