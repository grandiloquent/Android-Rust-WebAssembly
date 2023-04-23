use crate::data::video::{Video, VideoData};
use crate::server::Database;
use crate::utils::date::get_epoch_secs;
use crate::utils::string::StringExt;
use rocket::http::Status;
use rocket::State;
use rusqlite::{params, Connection};
use std::sync::Arc;
use std::sync::MutexGuard;
use rocket::serde::json::serde_json;

fn query(conn: &MutexGuard<Connection>, uri: &str) -> Result<(String, String, u64), rusqlite::Error> {
    conn.query_row(
        "SELECT title,file,update_at FROM video WHERE uri = ?",
        params![uri],
        |r| Ok((r.get(0).unwrap(), r.get(1).unwrap(), r.get(2).unwrap())),
    )
}

fn insert(conn: &MutexGuard<Connection>, video: &Video) -> Result<(), rusqlite::Error> {
    conn.query_row("INSERT INTO video (uri,title,file,image,source_type,hidden,create_at,update_at) VALUES (?,?,?,?,?,?,?,?)", params![
video.uri,
video.title,
video.file,
video.image,
video.source_type,
video.hidden,
video.create_at,
video.update_at
    ], |r| {
        Ok(())
    })
}

fn update(conn: &MutexGuard<Connection>, video: &Video) -> Result<(), rusqlite::Error> {
    conn.query_row(
        "UPDATE video SET title = ?,file = ?,image = ?,source_type = ?,hidden = ? WHERE uri = ?",
        params![video.title,
video.file,
video.image,
video.source_type,
video.hidden,
video.update_at,video.uri],
        |r| Ok(()),
    )
    /*
    conn.query_row(
        "UPDATE video SET file = ?,update_at = ? WHERE uri = ?",
        params![video.uri, video.file, video.update_at],
        |r| Ok(()),
    )
     */
}

#[get("/video/fetch?<url>")]
pub async fn parse(url: String, db: &State<Arc<Database>>) -> Result<String, Status> {
    if url.contains("xvideos.com") {
        let mut is_update = false;
        if let Ok(v) = query(&db.0.lock().unwrap(), &url) {
            let now = get_epoch_secs();
            if now - v.2 <= 3600 {
                return Ok(serde_json::to_string(&VideoData {
                    title: v.0,
                    file: v.1,
                }).unwrap());
            }
            is_update = true;
        }

        let video = match Video::xvideos(&url, true).await
        {
            Ok(res) => {
                res
            }
            Err(err) => {
                return Err(Status::InternalServerError);
            }
        };
        if is_update {
            update(&db.0.lock().unwrap(), &video);
        } else {
            insert(&db.0.lock().unwrap(), &video);
        }
        Ok(serde_json::to_string(&VideoData {
            title: video.title,
            file: video.file,
        }).unwrap())
    } else {
        Err(Status::NotFound)
    }
}
