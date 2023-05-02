use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::net::gen_ipv4;
use crate::utils::string::{ StringExt};
use rocket::serde::json::{serde_json, Value};
use std::error::Error;

async fn fetch_twitter<'a>(id: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let mut client = reqwest::Client::builder().user_agent(config.user_agent);
    if let Some(proxy) = config.proxy {
        let proxy = reqwest::Proxy::http(proxy)?;
        client = client.proxy(proxy);
    }
    let ip4 = gen_ipv4();

    let res = client
        .build()?
        .post("https://twittervideodownloaderpro.com/twittervideodownloadv2/index.php")
        .header("X-Forwarded-For", ip4.as_str())
        .header("X-Real-Ip", ip4.as_str())
        .form(&[("id", id)])
        .send()
        .await?;
    // e
    log::error!("extract_twitter: {}", res.status());
    res.text().await
}
pub async fn extract_twitter(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(Some("http://127.0.0.1:10809"), None);
    let res = match fetch_twitter(url.to_string().substring_after_last("/").as_str(), &config).await
    {
        Ok(res) => res,
        Err(err) => {
            log::error!("extract_twitter error: {}", err);
            return Err("")?;
        }
    };
    if res.len() <= 3 {
        return Err("")?;
    }
    log::error!("extract_twitter: {}", res.len());
    let obj: Value = match serde_json::from_str(&res) {
        Ok(v) => v,
        Err(err) => {
            return Err("")?;
        }
    };
    let videos = match obj["videos"].as_array() {
        Some(v) => v,
        None => {
            return Err("")?;
        }
    };
    let mut videos = videos.iter().map(|v| v.clone()).collect::<Vec<Value>>();
    // e
    log::error!("extract_twitter: {}", "1");
    videos.sort_by(|a, b| {
        return b["size"].as_u64().partial_cmp(&a["size"].as_u64()).unwrap();
    });
    let video = videos[0].clone();
    let file = video["url"].as_str().unwrap().to_string();
    let uri = url.to_string();
    if is_detail {
        let title = video["text"].as_str().unwrap().to_string();
        let image = video["thumb"].as_str().unwrap().to_string();
        let source_type = 3;
        let hidden = 0;
        let create_at = get_epoch_secs();
        let update_at = get_epoch_secs();
        Ok(Video {
            id: 0,
            uri,
            title,
            subtitle: String::new(),
            file: file,
            image,
            source_type,
            hidden,
            create_at,
            update_at,
        })
    } else {
        Ok(Video {
            id: 0,
            uri,
            title: String::new(),
            subtitle: String::new(),
            file,
            image: String::new(),
            source_type: 0,
            hidden: 0,
            create_at: 0,
            update_at: get_epoch_secs(),
        })
    }
}
