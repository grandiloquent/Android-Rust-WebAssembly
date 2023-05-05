use crate::extractor::erotic_mv::extract_erotic_mv;
use crate::extractor::five_two_ck::extract_five_two_ck;
use crate::extractor::ma_hua::extract_ma_hua;
use crate::extractor::nine_porn::extract_nine_porn;
use crate::extractor::twitter::extract_twitter;
use crate::extractor::xvideos::extract_xvideos;
use crate::extractor::jable::extract_jable;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VideoData {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    pub file: String,
}

pub struct Video {
    pub id: i32,
    pub uri: String,
    pub title: String,
    pub subtitle: String,
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
    pub async fn erotic_mv(url: &str, is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_erotic_mv(url, is_detail).await
    }
    pub async fn ma_hua(url: &str,cookie: &str ,is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_ma_hua(url,cookie, is_detail).await
    }
    pub async fn twitter(url: &str, is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_twitter(url, is_detail).await
    }
    pub async fn jable(url: &str, cookie: &str,is_detail: bool) -> Result<Self, Box<dyn Error>> {
        extract_jable(url, cookie, is_detail).await
    }
    pub async fn five_two_ck(
        url: &str,
        cookie: &str,
        is_detail: bool,
    ) -> Result<Self, Box<dyn Error>> {
        extract_five_two_ck(url, cookie,is_detail).await
    }
    pub fn new() -> Self {
        Video {
            id: 0,
            uri: String::new(),
            title: String::new(),
            subtitle: String::new(),
            file: String::new(),
            image: String::new(),
            source_type: 0,
            hidden: 0,
            create_at: 0,
            update_at: 0,
        }
    }
}
