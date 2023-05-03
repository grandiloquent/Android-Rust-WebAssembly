use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_update_video(
    conn: &MutexGuard<Connection>,
    id: i32,
    uri: &str,
    title: &str,
    subtitle: &str,
    file: &str,
    image: &str,
    source_type: i32,
    hidden: i32,
    update_at: u64,
) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET title = ?,subtitle = ?,file = ?,image = ?,source_type = ?,hidden = ?,update_at = ? WHERE uri = ?",
        params![title,subtitle,file,image,source_type,hidden,update_at,uri],
        |_r| Ok(()),
    )
}
