use crate::hidden_video::list_videos::execute_list_videos;
use crate::server::Database;
use crate::utils::string::StringExt;
use rocket::http::Status;
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

fn delete(conn: &MutexGuard<Connection>, id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM video WHERE id = ?", params![id])
}
fn hidden(conn: &MutexGuard<Connection>, id: i32) -> Result<usize, rusqlite::Error> {
    conn.execute("UPDATE video SET hidden = 1 WHERE id = ?", params![id])
}
#[get("/videos/list?<offset>&<limit>")]
pub fn list(
    offset: Option<u32>,
    limit: Option<u32>,
    hidden_video: &State<Arc<Database>>,
) -> Result<String, Status> {
    if let Ok(v) = execute_list_videos(
        &hidden_video.0.lock().unwrap(),
        offset.unwrap_or(0),
        limit.unwrap_or(20),
    ) {
        Ok(v)
    } else {
        Err(Status::NotFound)
    }
}
fn query_all(conn: &MutexGuard<Connection>) -> Result<(), rusqlite::Error> {
    let mut query = conn.prepare("SELECT id,uri FROM video")?;
    let mut rows = query.query(params![])?;

    while let Some(row) = rows.next()? {
        // id: row.get(0)?,
        // uri: row.get(1)?,
        let id: u32 = row.get(0)?;
        let mut uri: String = row.get(1)?;
        if uri.starts_with("http://91porn.com/view_video.php?") {
            uri = format!(
                "{}?viewkey={}",
                uri.substring_before("?"),
                uri.substring_between("viewkey=", "&")
            );
            update(conn, id, uri.as_str());
            log::error!("{} {}", id, uri);
        }
    }
    Ok(())
}
fn update(conn: &MutexGuard<Connection>, id: u32, uri: &str) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET uri = ? WHERE id = ?",
        params![uri, id],
        |r| Ok(()),
    )
}
#[get("/videos/delete?<id>")]
pub fn delete_video(id: i32, hidden_video: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(v) = delete(&hidden_video.0.lock().unwrap(), id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}
#[get("/videos/hidden?<id>")]
pub fn hidden_video(id: i32, hidden_video: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(v) = hidden(&hidden_video.0.lock().unwrap(), id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}
