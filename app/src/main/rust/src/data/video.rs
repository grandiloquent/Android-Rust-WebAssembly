use std::error::Error;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use crate::extractor::nine_porn::extract_nine_porn;
use crate::extractor::xvideos::extract_xvideos;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoData {
    pub title: String,
    pub file: String,
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


impl Video {
    pub async fn xvideos(url: &str, is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_xvideos(url, is_detail).await
    }
    pub async fn nine_porn(url: &str, is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_nine_porn(url, is_detail).await
    }
}
