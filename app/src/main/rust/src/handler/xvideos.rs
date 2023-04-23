use std::sync::{Arc};
use rocket::http::Status;
use rocket::State;
use crate::server::Database;
use crate::utils::string::StringExt;
use rusqlite::{Connection, params};
use std::sync::MutexGuard;
use crate::utils::date::{get_epoch_secs};
async fn fetch(url: &str) -> reqwest::Result<String> {
    let proxy = reqwest::Proxy::http("http://127.0.0.1:10809")?;
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1")
        .proxy(proxy)
        .build()?;
    client.get(url).send().await?.text().await
}
pub struct Video {
    pub id: i32,
    pub uri: String,
    pub title: String,
    pub file: String,
    pub image: String,
    pub source_type: i32,
    pub hidden: i32,
    pub create_at: u64,
    pub update_at: u64,
}
fn query(conn: &MutexGuard<Connection>, uri: &str) -> Result<(String, u64), rusqlite::Error> {
    conn.query_row("SELECT file,update_at FROM video WHERE uri = ?", params![uri], |r| {
        Ok((r.get(0).unwrap(), r.get(1).unwrap()))
    })
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
    conn.query_row("UPDATE video SET file = ?,update_at = ? WHERE uri = ?", params![
video.uri,
video.file,
video.update_at
    ], |r| {
        Ok(())
    })
}
#[get("/xvideos?<url>")]
pub async fn parse(url: String, db: &State<Arc<Database>>) -> Result<String, Status> {
    let mut is_update = false;
    if let Ok(v) = query(&db.0.lock().unwrap(), &url)
    {
        let now = get_epoch_secs();
        if now - v.1 <= 3600 {
            return Ok(v.0);
        }
        is_update = true;
    }
    let res = fetch(&url);
    let video = match res.await {
        Ok(res) => {
            let file = res.substring_between("setVideoUrlHigh('", "'");
            let uri = url;
            if is_update {
                Video {
                    id: 0,
                    uri,
                    title: String::new(),
                    file,
                    image: String::new(),
                    source_type: 0,
                    hidden: 0,
                    create_at: 0,
                    update_at: get_epoch_secs(),
                }
            } else {
                let title = res.substring_between("<title>", " - XVIDEOS.COM</title>");
                let image = res.substring_between("setThumbUrl169('", "'");
                let source_type = 1;
                let hidden = 0;
                let create_at = get_epoch_secs();
                let update_at = get_epoch_secs();
                Video {
                    id: 0,
                    uri,
                    title,
                    file: file.clone(),
                    image,
                    source_type,
                    hidden,
                    create_at,
                    update_at,
                }
            }
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
    Ok(video.file)
}