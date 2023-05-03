
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    pub id: i32,
    pub uri: String,
    pub title: String,
    pub image: String,
    pub source_type: i32,
    pub update_at: u64,
}