use crate::data::video::{Video, VideoData};
use crate::db::query_cookie::execute_query_cookie;
use crate::db::update_video::execute_update_video;
use crate::db::update_video_file::execute_update_video_file;
use crate::db::update_video_views::execute_update_video_views;
use crate::server::Database;
use crate::utils::date::get_epoch_secs;
use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::State;
use rusqlite::{params, Connection};
use std::sync::Arc;
use std::sync::MutexGuard;

fn query(
    conn: &MutexGuard<Connection>,
    uri: &str,
) -> Result<(String, Option<String>, String, u64), rusqlite::Error> {
    conn.query_row(
        "SELECT title,subtitle,file,update_at FROM video WHERE uri = ?",
        params![uri],
        |r| {
            Ok((
                r.get(0).unwrap(),
                r.get(1).unwrap(),
                r.get(2).unwrap(),
                r.get(3).unwrap(),
            ))
        },
    )
}
fn insert(conn: &MutexGuard<Connection>, video: &Video) -> Result<(), rusqlite::Error> {
    conn.query_row("INSERT INTO video (uri,title,subtitle,file,image,source_type,hidden,create_at,update_at) VALUES (?,?,?,?,?,?,?,?,?)", params![
video.uri,
video.title,
video.subtitle,
video.file,
video.image,
video.source_type,
video.hidden,
video.create_at,
video.update_at
    ], |_r| {
        Ok(())
    })
}

fn read_from_database(
    url: &str,
    db: &MutexGuard<Connection>,
) -> Result<String, Box<dyn std::error::Error>> {
    let v = query(db, &url)?;
    if url.contains("xvideos.com") || url.contains("mahua11.com") {
        let now = get_epoch_secs();
        if now - v.3 <= 3600 {
            return Ok(serde_json::to_string(&VideoData {
                title: v.0,
                subtitle: v.1,
                file: v.2,
            })
            .unwrap());
        }
        return Ok(String::new());
    } else if url.contains("91porn.com")
        || url.contains("eroticmv.com")
        || url.starts_with("/vodplay/")
    {
        return Ok(serde_json::to_string(&VideoData {
            title: v.0,
            subtitle: v.1,
            file: v.2,
        })
        .unwrap());
    }
    Err("")?
}
async fn create_video(
    url: &str,
    is_detail: bool,
    cookie: &str,
) -> Result<Video, Box<dyn std::error::Error>> {
    if url.contains("xvideos.com") {
        Video::xvideos(&url, is_detail).await
    } else if url.contains("91porn.com") {
        Video::nine_porn(&url, is_detail).await
    } else if url.contains("eroticmv.com") {
        Video::erotic_mv(&url, is_detail).await
    } else if url.contains("mahua11.com") {
        Video::ma_hua(&url, is_detail).await
    } else if url.contains("/vodplay/") {
        Video::five_two_ck(&url, cookie, is_detail).await
    } else {
        Err("")?
    }
}

#[get("/video/fetch?<url>")]
pub async fn parse(url: String, db: &State<Arc<Database>>) -> Result<String, Status> {
    let _ = execute_update_video_views(&db.0.lock().unwrap(), url.as_str());
    let mut is_update = false;
    if let Ok(v) = read_from_database(&url, &db.0.lock().unwrap()) {
        if v.is_empty() {
            is_update = true;
        } else {
            return Ok(v);
        }
    }

    let cookie = if url.contains("/vodplay/") {
        execute_query_cookie(&db.0.lock().unwrap())
    } else {
        String::new()
    };
    match create_video(&url, !is_update, cookie.as_str()).await {
        Ok(video) => {
            if is_update {
                match execute_update_video_file(
                    &db.0.lock().unwrap(),
                    video.uri.as_str(),
                    video.file.as_str(),
                ) {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!("{}", err);
                    }
                };
            } else {
                if video.title.is_empty() || video.image.is_empty() {
                    return Err(Status::NoContent);
                }
                if let Err(err) = insert(&db.0.lock().unwrap(), &video) {
                    log::error!("{}", err.to_string());
                    return Err(Status::Conflict);
                }
            }
            return Ok(serde_json::to_string(&VideoData {
                title: video.title,
                subtitle: Some(video.subtitle),
                file: video.file,
            })
            .unwrap());
        }
        Err(_err) => {
            log::error!("{}", _err.to_string());
            return Err(Status::InternalServerError);
        }
    };
}
#[get("/video/get?<url>")]
pub async fn get(url: String) -> Status {
    // log::error!("get: {}", url);
    // let video = Video::five_two_ck(url.as_str(), true).await.unwrap();
    // log::error!("id = {}\nuri = {}\ntitle = {}\nfile = {}\nimage = {}\nsource_type = {}\nhidden = {}\ncreate_at = {}\nupdate_at = {}\nid = {}",video.id,video.uri,video.title,video.file,video.image,video.source_type,video.hidden,video.create_at,video.update_at,video.id);
    Status::Ok
}
#[get("/video/url?<id>")]
pub fn get_url(id: u32, db: &State<Arc<Database>>) -> Result<String, Status> {
    match db
        .0
        .lock()
        .unwrap()
        .query_row(
            "select uri from video where id = ?",
            params![id],
            |row| match row.get(0) {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            },
        ) {
        Ok(value) => Ok(value),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/video/update?<url>")]
pub async fn update(url: String, db: &State<Arc<Database>>) -> Result<String, Status> {
    let cookie = if url.contains("/vodplay/") {
        execute_query_cookie(&db.0.lock().unwrap())
    } else {
        String::new()
    };
    match create_video(&url, true, cookie.as_str()).await {
        Ok(video) => {
            let _ = execute_update_video(
                &db.0.lock().unwrap(),
                video.id,
                &video.uri,
                &video.title,
                &video.subtitle,
                &video.file,
                &video.image,
                video.source_type,
                video.hidden,
                get_epoch_secs(),
            );
            Ok(String::from("Success"))
        }
        Err(_err) => {
            return Err(Status::InternalServerError);
        }
    }
}
#[get("/video/fav?<id>")]
pub fn update_fav(id: u32, db: &State<Arc<Database>>) -> Result<String, Status> {
    match db.0.lock().unwrap().execute(
        "update video set source_type = 10 where id = ?",
        params![id],
    ) {
        Ok(value) => Ok(value.to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/video/duration?<duration>&<url>")]
pub fn update_duration(
    duration: u32,
    url: String,
    db: &State<Arc<Database>>,
) -> Result<String, Status> {
    match db.0.lock().unwrap().execute(
        "update video set duration = ? where uri = ?",
        params![duration, url.as_str()],
    ) {
        Ok(value) => Ok(value.to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}
