use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::net::gen_ipv4;
use crate::utils::string::{parse_number, StringExt};
use regex::Regex;
use std::error::Error;
use urlencoding::decode;

async fn fetch_five_two_ck<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
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
pub async fn extract_five_two_ck(url: &str, is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(None, Some(r#"Hm_lvt_9c69de51657cb6e2da4f620629691e94=1683028155; Hm_lpvt_9c69de51657cb6e2da4f620629691e94=1683028277"#));
    let res = match fetch_five_two_ck(&url, &config).await {
        Ok(res) => res,
        Err(err) => {
            // e
            log::error!("extract_five_two_ck: {}", err);
            String::new()
        }
    };
    let file = res.substring_between(r#""link_pre":"","url":""#, r#"""#)
    .replace(r#"\"#, "");
    let uri = format_address(url);
    if is_detail {
        let title = res
            .substring_between(r#"<meta name="keywords" content=""#, "在线收看,")
            .trim()
            .to_string();
        let image = res
            .substring_between(r#"<a class="stui-vodlist__thumb lazyload" href=""#, "<span")
            .substring_between(r#"data-original=""#, r#"""#);
        let source_type = 5;
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

fn format_address(s: &str) -> String {
    let regex = Regex::new("/vodplay/[0-9-]+.html").unwrap();
    match regex.find(s) {
        Some(s) => s.as_str().to_string(),
        None => String::new(),
    }
}
