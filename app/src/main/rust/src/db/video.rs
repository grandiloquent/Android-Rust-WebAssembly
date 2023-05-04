
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Video {
    pub id: u32,
    pub uri: String,
    pub title: String,
    pub image: String,
    pub duration: Option<u32>,
    pub source_type: u32,
    pub update_at: u64,
}