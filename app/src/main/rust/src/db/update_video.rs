use rusqlite::{params, Connection};
use std::sync::MutexGuard;

pub fn execute_update_video(
    conn: &MutexGuard<Connection>,
    id: u32,
    uri: &str,
    title: &str,
    subtitle: &str,
    duration: u32,
    file: &str,
    image: &str,
    source_type: u32,
    views: u32,
    hidden: u32,
    create_at: u32,
    update_at: u32,
) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET uri = ?,title = ?,subtitle = ?,duration = ?,file = ?,image = ?,source_type = ?,views = ?,hidden = ?,create_at = ?,update_at = ? WHERE id = ?",
        params![uri,title,subtitle,duration,file,image,source_type,views,hidden,create_at,update_at, id],
        |_r| Ok(()),
    )
}
