use crate::data::asset::Asset;
use crate::data::cache::Cache;
use crate::seek_stream::mimetypes::extension_to_mime;
use crate::utils::string::StringExt;
use rocket::State;
use std::path::PathBuf;
use std::sync::Arc;
use rocket::http::Status;
#[get("/favicon.ico")]
pub fn fav() ->  Status {
    Status::NotFound
}
#[get("/<b..>")]
pub fn file(b: PathBuf, cache: &State<Arc<Cache>>) -> Asset {
    match cache.get(b.to_str().unwrap_or("")) {
        None => Asset::default(),
        Some(data) => Asset {
            data,
            content_type: extension_to_mime(
                b.to_str()
                    .unwrap_or("")
                    .to_string()
                    .substring_after_last(".")
                    .as_str(),
            ),
        },
    }
}
