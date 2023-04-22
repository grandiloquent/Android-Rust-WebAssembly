use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::utils::file::get_file_list;
#[get("/files?<path>")]
pub fn files(path: String) -> RawJson<String> {
    RawJson(serde_json::to_string(&get_file_list(path, "/storage/emulated/0")).unwrap_or("".to_string()))
}