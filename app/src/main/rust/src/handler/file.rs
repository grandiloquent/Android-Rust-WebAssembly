use std::path::{Path};
use rocket::get;
use crate::seek_stream::SeekStream;
#[get("/file?<path>")]
pub fn file<'a>(path: String) -> std::io::Result<SeekStream<'a>> {
    let p = Path::new(path.as_str());
    SeekStream::from_path(p)
}