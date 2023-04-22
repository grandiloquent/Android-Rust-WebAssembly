use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::utils::file::get_file_list;
#[get("/files?<path..>")]
pub fn files(path: Option<String>) -> RawJson<String> {
    match path {
        Some(v)=>{
            RawJson(serde_json::to_string(&get_file_list(v)).unwrap_or("".to_string()))
        }
        None=>{
            RawJson(serde_json::to_string(&get_file_list( "/storage/emulated/0".to_string())).unwrap_or("".to_string()))
        }
    }
}