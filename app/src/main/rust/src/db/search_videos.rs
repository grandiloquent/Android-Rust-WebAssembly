use rocket::serde::json::serde_json;
use rusqlite::{params, Connection};
use std::error::Error;
use std::sync::MutexGuard;
use crate::db::video::Video;

fn search_videos(conn: &MutexGuard<Connection>) -> Result<Vec<Video>, rusqlite::Error> {
    let mut query = conn.prepare(
        "SELECT id,uri,title,image,source_type,update_at FROM video ORDER BY update_at DESC",
    )?;
    let mut rows = query.query(params![])?;
    let mut v = Vec::<Video>::new();
    while let Some(row) = rows.next()? {
        v.push(Video {
            id: row.get(0)?,
            uri: row.get(1)?,
            title: row.get(2)?,
            image: row.get(3)?,
            source_type: row.get(4)?,
            update_at: row.get(5)?,
        });
    }
    Ok(v)
}

pub fn execute_search_videos(
    conn: &MutexGuard<Connection>,
    q: &str,
    offset: u32,
    limit: u32,
) -> Result<String, Box<dyn Error>> {
    let v = search_videos(conn)?;
    let v = v
        .iter()
        .filter(|video| video.uri.contains(q) || video.title.contains(q))
        .collect::<Vec<&Video>>();
    match serde_json::to_string(&v) {
        Ok(v) => Ok(v),
        Err(_) => Err("")?,
    }
}
