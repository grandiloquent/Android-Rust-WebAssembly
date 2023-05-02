use rocket::serde::json::serde_json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rusqlite::{params, Connection};
use std::error::Error;
use std::sync::MutexGuard;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    pub id: i32,
    pub uri: String,
    pub title: String,
    pub image: String,
    pub source_type: i32,
    pub update_at: u64,
}
fn list_videos(
    conn: &MutexGuard<Connection>,
    offset: u32,
    limit: u32,
) -> Result<Vec<Video>, rusqlite::Error> {
    let mut query = conn.prepare("SELECT id,uri,title,image,source_type,update_at FROM video WHERE hidden = 0 ORDER BY update_at DESC LIMIT ? OFFSET ?")?;
    let mut rows = query.query(params![limit, offset])?;
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

pub fn execute_list_videos(
    conn: &MutexGuard<Connection>,
    offset: u32,
    limit: u32,
) -> Result<String, Box<dyn Error>> {
    let v = list_videos(conn, offset, limit)?;
    match serde_json::to_string(&v) {
        Ok(v) => Ok(v),
        Err(_) => Err("")?,
    }
}
