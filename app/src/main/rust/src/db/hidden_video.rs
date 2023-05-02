use rusqlite::{params, Connection};
use std::error::Error;
use std::sync::MutexGuard;

pub fn execute_hidden_video(conn: &MutexGuard<Connection>, id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("UPDATE video SET hidden = 1 WHERE id = ?", params![id])
}