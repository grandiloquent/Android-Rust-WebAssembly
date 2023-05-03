use crate::db::delete_video::execute_delete_video;
use crate::db::hidden_video::execute_hidden_video;
use crate::db::list_videos::execute_list_videos;
use crate::db::search_videos::execute_search_videos;
use crate::server::Database;
use rocket::http::Status;
use rocket::State;
use std::sync::Arc;

#[get("/videos/list?<offset>&<limit>&<q>&<t>")]
pub fn list(
    offset: Option<u32>,
    limit: Option<u32>,
    q: Option<String>,
    t: Option<u32>,
    db: &State<Arc<Database>>,
) -> Result<String, Status> {
    if let Some(v) = q {
        if let Ok(v) = execute_search_videos(
            &db.0.lock().unwrap(),
            &v,
            offset.unwrap_or(0),
            limit.unwrap_or(20),
        ) {
            Ok(v)
        } else {
            Err(Status::NotFound)
        }
    } else {
        if let Ok(v) = execute_list_videos(
            &db.0.lock().unwrap(),
            offset.unwrap_or(0),
            limit.unwrap_or(20),
        ) {
            Ok(v)
        } else {
            Err(Status::NotFound)
        }
    }
}
#[get("/videos/search?<q>&<offset>&<limit>")]
pub fn search(
    q: String,
    offset: Option<u32>,
    limit: Option<u32>,
    db: &State<Arc<Database>>,
) -> Result<String, Status> {
    if let Ok(v) = execute_search_videos(
        &db.0.lock().unwrap(),
        &q,
        offset.unwrap_or(0),
        limit.unwrap_or(20),
    ) {
        Ok(v)
    } else {
        Err(Status::NotFound)
    }
}
#[get("/videos/delete?<id>")]
pub fn delete_video(id: i32, db: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(_v) = execute_delete_video(&db.0.lock().unwrap(), id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}
#[get("/videos/hidden?<id>")]
pub fn hidden_video(id: i32, db: &State<Arc<Database>>) -> Result<String, Status> {
    if let Ok(_v) = execute_hidden_video(&db.0.lock().unwrap(), id) {
        Ok("Success".to_string())
    } else {
        Err(Status::NotFound)
    }
}
