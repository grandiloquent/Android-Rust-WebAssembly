use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::string::{StringExt};
use std::error::Error;

async fn fetch_jable<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let mut client = reqwest::Client::builder().user_agent(config.user_agent);
    if let Some(proxy) = config.proxy {
        let proxy = reqwest::Proxy::http(proxy)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
    let mut client = client.get(url);
    if let Some(v) = config.cookie {
        client = client.header("Cookie", v);
    }
    client = client.header("Referer", "https://jable.tv/")
    .header("Accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7");
    client.send().await?.text().await
}
pub async fn extract_jable(url: &str,  cookie: &str,is_detail: bool) -> Result<Video, Box<dyn Error>> {
    let config = Config::new(Some("http://127.0.0.1:10809"), Some(cookie));
    let res = match fetch_jable(&url, &config).await {
        Ok(res) => res,
        Err(err) => {
            // e
            log::error!("extract_jable: {}", err);
            String::new()
        }
    };
    // lf
    let file = res.substring_between(r#"var hlsUrl = '"#, r#"';"#);
    let uri = url.to_string();
    if is_detail {
        let title = res
            .substring_between(r#"<meta property="og:title" content=""#, r#"""#)
            .trim()
            .to_string();
        let image = res.substring_between(r#"<meta property="og:image" content=""#, r#"""#);
        let source_type = 7;
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
