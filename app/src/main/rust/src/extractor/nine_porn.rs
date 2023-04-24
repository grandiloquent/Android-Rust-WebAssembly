use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::net::gen_ipv4;
use crate::utils::string::{parse_number, StringExt};
use std::error::Error;
use urlencoding::decode;

async fn fetch_nine_porn<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let mut client = reqwest::Client::builder().user_agent(config.user_agent);
    if let Some(proxy) = config.proxy {
        let proxy = reqwest::Proxy::http(proxy)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
    let ip4 = gen_ipv4();
    let mut client = client
        .get(url)
        .header("Accept-Language", "zh-CN,zh;q=0.9")
        .header("X-Forwarded-For", ip4.as_str())
        .header("X-Real-Ip", ip4.as_str());
    if let Some(v) = config.cookie {
        client = client.header("Cookie", v);
    }
    client.send().await?.text().await
}
pub async fn extract_nine_porn(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(
        Some("http://127.0.0.1:10809"),
        Some("CLIPSHARE=0av6iij67j68ml4v90tr2oj96k"),
    );
    let res = match fetch_nine_porn(&url, &config).await {
        Ok(res) => res,
        Err(err) => {
            // e
            log::error!("extract_nine_porn: {}", err);
            String::new()
        }
    };
    let mut file = res.substring_between("document.write(strencode2(\"", "\"");
    file = decode(&file).unwrap().to_string().substring_between("src='", "'");
    
    let uri = url.to_string();
    if is_detail {
        let title = res.substring_between("<title>", "\n").trim().to_string();
        let image = res.substring_between("poster=\"", "\"");
        let source_type = 2;
        let hidden = 0;
        let create_at = get_epoch_secs();
        let update_at = get_epoch_secs();
        Ok(Video {
            id: 0,
            uri,
            title,
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
            file,
            image: String::new(),
            source_type: 0,
            hidden: 0,
            create_at: 0,
            update_at: get_epoch_secs(),
        })
    }
}
