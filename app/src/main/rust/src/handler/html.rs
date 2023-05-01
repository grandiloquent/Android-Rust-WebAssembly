use std::path::PathBuf;
use std::sync::Arc;
use rocket::State;
use crate::data::asset::Asset;
use crate::data::cache::Cache;
use crate::seek_stream::mimetypes::extension_to_mime;
use crate::utils::string::StringExt;
#[get("/<b..>")]
pub fn file(b: PathBuf, cache: &State<Arc<Cache>>) -> Asset {
    match cache.get(b.to_str().unwrap_or("")) {
        None => {
            Asset::default()
        }
        Some(data) => {
            log::error!("{}",b.to_str().unwrap_or(""));
            Asset {
                data,
                content_type: extension_to_mime(b.to_str().unwrap_or("").to_string().substring_after_last(".").as_str()),
            }
        }
    }
}