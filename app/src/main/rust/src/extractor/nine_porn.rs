use std::error::Error;
use crate::data::video::Video;
use crate::utils::date::get_epoch_secs;
use crate::utils::string::{parse_number, StringExt};
use crate::extractor::config::Config;
use crate::utils::net::gen_ipv4;

async fn fetch_nine_porn<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let proxy = reqwest::Proxy::http("http://127.0.0.1:10809")?;
    let client = reqwest::Client::builder()
        .user_agent(config.user_agent)
        .proxy(proxy)
        .build()?;
    let ip4 = gen_ipv4();

    let mut client = client.get(url)
        .header("X-Forwarded-For", ip4.as_str())
        .header("X-Real-Ip", ip4.as_str());
    if let Some(v) = config.cookie {
        client = client.header("Cookie", v);
    }
    client.send().await?.text().await
}

fn find_nine_porn(s: &str) -> String {
    let mut lines = s
        .lines()
        .skip(1)
        .filter(|f| f.contains(".m3u8"))
        .collect::<Vec<&str>>();
    lines.sort_by(|a, b| parse_number(b).partial_cmp(&parse_number(a)).unwrap());
    return lines[0].to_string();
}

pub async fn extract_nine_porn(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(None);
    let res = fetch_nine_porn(&url, &config).await?;
    // let mut file = res.substring_between("setVideoHLS('", "'");
    // let hls = fetch_nine_porn(&file).await?;
    // file = format!("{}/{}", file.substring_before_last("/"), find_nine_porn(&hls));
    // let uri = url.to_string();
    // if is_detail {
    //     let title = res.substring_between("<title>", " - NINE PORN.COM</title>");
    //     let image = res.substring_between("setThumbUrl169('", "'");
    //     let source_type = 1;
    //     let hidden = 0;
    //     let create_at = get_epoch_secs();
    //     let update_at = get_epoch_secs();
    //     Ok(Video {
    //         id: 0,
    //         uri,
    //         title,
    //         file: file,
    //         image,
    //         source_type,
    //         hidden,
    //         create_at,
    //         update_at,
    //     })
    // } else {
    Ok(Video {
        id: 0,
        uri: String::new(),
        title: String::new(),
        file: String::new(),
        image: String::new(),
        source_type: 0,
        hidden: 0,
        create_at: 0,
        update_at: get_epoch_secs(),
    })
    // }
}