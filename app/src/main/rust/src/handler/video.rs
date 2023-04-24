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
use serde::de::Unexpected::Str;

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
    /*    conn.query_row(
            "UPDATE video SET title = ?,file = ?,image = ?,source_type = ?,hidden = ?, update_at = ? WHERE uri = ?",
            params![video.title,
    video.file,
    video.image,
    video.source_type,
    video.hidden,
    video.update_at,video.uri],
            |r| Ok(()),
        )*/

    conn.query_row(
        "UPDATE video SET file = ?,update_at = ? WHERE uri = ?",
        params![video.uri, video.file, video.update_at],
        |r| Ok(()),
    )
}

fn read_from_database(url: &str, db: &MutexGuard<Connection>) -> Result<String, Box<dyn std::error::Error>> {
    let v = query(db, &url)?;
    if url.contains("xvideos.com") {
        let now = get_epoch_secs();
        if now - v.2 <= 3600 {
            return Ok(serde_json::to_string(&VideoData {
                title: v.0,
                file: v.1,
            }).unwrap());
        }
        return Ok(String::new());
    }
    Err("")?
}

async fn create_video(url: &str, is_detail: bool) -> Result<Video, Box<dyn std::error::Error>> {
    if url.contains("xvideos.com") {
        Video::xvideos(&url, is_detail).await
    } else {
        Err("")?
    }
}

#[get("/video/fetch?<url>")]
pub async fn parse(url: String, db: &State<Arc<Database>>) -> Result<String, Status> {
    let mut is_update = false;
    if let Ok(v) = read_from_database(&url, &db.0.lock().unwrap()) {
        if v.is_empty() {
            is_update = true;
        } else {
            return Ok(v);
        }
    }
    match create_video(&url, !is_update).await
    {
        Ok(video) => {
            if is_update {
                match update(&db.0.lock().unwrap(), &video) {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("{}",err);
                    }
                };
            } else {
                insert(&db.0.lock().unwrap(), &video);
            }
            return Ok(serde_json::to_string(&VideoData {
                title: video.title,
                file: video.file,
            }).unwrap());
        }
        Err(err) => {
            return Err(Status::InternalServerError);
        }
    };
    //log::error!("id = {}\nuri = {}\ntitle = {}\nfile = {}\nimage = {}\nsource_type = {}\nhidden = {}\ncreate_at = {}\nupdate_at = {}\nid = {}",video.id,video.uri,video.title,video.file,video.image,video.source_type,video.hidden,video.create_at,video.update_at,video.id);
}
