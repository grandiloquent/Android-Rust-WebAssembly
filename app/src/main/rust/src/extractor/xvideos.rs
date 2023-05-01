use std::error::Error;
use crate::data::video::Video;
use crate::utils::date::get_epoch_secs;
use crate::utils::string::{parse_number, StringExt};

async fn fetch_xvideos(url: &str) -> reqwest::Result<String> {
    let proxy = reqwest::Proxy::http("http://127.0.0.1:10809")?;
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1")
        .proxy(proxy)
        .build()?;
    client.get(url).send().await?.text().await
}

fn find_xvideos(s: &str) -> String {
    let mut lines = s
        .lines()
        .skip(1)
        .filter(|f| f.contains(".m3u8"))
        .collect::<Vec<&str>>();
    lines.sort_by(|a, b| parse_number(b).partial_cmp(&parse_number(a)).unwrap());
    return lines[0].to_string();
}

pub async fn extract_xvideos(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let res = fetch_xvideos(&url).await?;
    let mut file = res.substring_between("setVideoHLS('", "'");
    let hls = fetch_xvideos(&file).await?;
    file = format!("{}/{}", file.substring_before_last("/"), find_xvideos(&hls));
    let uri = url.to_string();
    if is_detail {
        let title = res.substring_between("<title>", " - XVIDEOS.COM</title>");
        let image = res.substring_between("setThumbUrl169('", "'");
        let source_type = 1;
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