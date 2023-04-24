use crate::server::Database;
use crate::utils::string::StringExt;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::State;
use rusqlite::{params, Connection};
use std::sync::Arc;
use std::sync::MutexGuard;
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Video {
    pub id: i32,
    pub uri: String,
    pub title: String,
    pub image: String,
    pub source_type: i32,
    pub update_at: u64,
}
fn query(conn: &MutexGuard<Connection>) -> Result<Vec<Video>, rusqlite::Error> {
    let mut query = conn.prepare("SELECT id,uri,title,image,source_type,update_at FROM video WHERE hidden = 0 ORDER BY update_at DESC ")?;
    let mut rows = query.query([])?;
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
fn delete(conn: &MutexGuard<Connection>,id: i32) -> Result<usize, rusqlite::Error>{
    conn.execute("DELETE FROM video WHERE id = ?", params![id])
}
fn hidden(conn: &MutexGuard<Connection>,id: i32) -> Result<usize, rusqlite::Error>{
    conn.execute("UPDATE video SET hidden = 1 WHERE id = ?", params![id])
}
#[get("/videos/list")]
pub fn list(db: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(v) = query(&db.0.lock().unwrap()) {
        Ok(serde_json::to_string(&v).unwrap_or(String::new()))
    } else {
        Err(Status::NotFound)
    }
}
#[get("/videos/delete?<id>")]
pub fn delete_video(id: i32, db: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(v) = delete(&db.0.lock().unwrap(),id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}
#[get("/videos/hidden?<id>")]
pub fn hidden_video(id: i32, db: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(v) = hidden(&db.0.lock().unwrap(),id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}