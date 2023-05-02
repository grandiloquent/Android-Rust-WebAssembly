use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::net::gen_ipv4;
use crate::utils::string::{StringExt};
use std::error::Error;

async fn fetch_ma_hua<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let mut client = reqwest::Client::builder().user_agent(config.user_agent);
    if let Some(proxy) = config.proxy {
        let proxy = reqwest::Proxy::http(proxy)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
    let ip4 = gen_ipv4();
    let mut client = client.get(url);
    if let Some(v) = config.cookie {
        client = client.header("Cookie", v);
    }
    client.send().await?.text().await
}
pub async fn extract_ma_hua(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(Some("http://127.0.0.1:10809"), Some("PHPSESSID=ug5qdi2pd3kcc53m5a6b2jqm9i; kt_ips=43.163.192.239; kt_tcookie=1; kt_is_visited=1; kt_qparams=id%3D14294%26dir%3D300022"));
    let res = match fetch_ma_hua(&url, &config).await {
        Ok(res) => res,
        Err(err) => {
            // e
            log::error!("extract_ma_hua: {}", err);
            String::new()
        }
    };
    let file =format!("https://www.mahua11.com/get_file/{}",res.substring_between("<a href=\"https://www.mahua11.com/get_file/", "\"")); 
    let uri = url.to_string();
    if is_detail {
        let title = res
            .substring_between("<meta property=\"og:title\" content=\"", "\"/>")
            .trim()
            .to_string();
        let image = res.substring_between("<meta property=\"og:image\" content=\"", "\"/>");
        let source_type = 4;
        let hidden = 0;
        let create_at = get_epoch_secs();
        let update_at = get_epoch_secs();
        Ok(Video {
            id: 0,
            uri,
            title,
            subtitle: String::new(),
            file,
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
