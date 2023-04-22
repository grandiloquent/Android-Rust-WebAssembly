use std::path::PathBuf;
use std::sync::Arc;
use rocket::State;
use crate::data::cache::Cache;
#[get("/<b..>")]
pub fn file<'a>(b: PathBuf, cache: &State<Arc<Cache>>) -> Asset {
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