use rusqlite::{params, Connection};
use std::sync::MutexGuard;
use crate::utils::date::get_epoch_secs;

pub fn execute_update_video_views(
    conn: &MutexGuard<Connection>,
    uri: &str,
) -> Result<usize, rusqlite::Error> {
    let r: (u32, Option<u32>) = conn.query_row(
        "select id,views from video WHERE uri = ?",
        params![uri],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    conn.execute(
        "UPDATE video SET views = ?,update_at = ? WHERE id = ?",
        params![r.1.unwrap_or(0) + 1, r.0,get_epoch_secs()],
    )
}
