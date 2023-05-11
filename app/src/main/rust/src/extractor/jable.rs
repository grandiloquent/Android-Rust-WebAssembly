use crate::data::video::Video;
use crate::extractor::config::Config;
use crate::utils::date::get_epoch_secs;
use crate::utils::net::gen_ipv4;
use crate::utils::string::StringExt;
use std::error::Error;

async fn fetch_jable<'a>(url: &str, config: &Config<'a>) -> reqwest::Result<String> {
    let mut client = reqwest::Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.69 Safari/537.36");
    if let Some(proxy) = config.proxy {
        let proxy = reqwest::Proxy::http(proxy)?;
        client = client.proxy(proxy);
    }
    let client = client.build()?;
    let mut client = client.get(url);
    // https://httpdump.app/
    if let Some(v) = config.cookie {
        client = client.header("cookie", v);
    }
    let ip4 = gen_ipv4();
    client = client
    .header("accept","text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
.header("accept-encoding","gzip, deflate, br")
.header("accept-language","en")
.header("cache-control","no-cache")
.header("pragma","no-cache")
.header("sec-ch-ua","\"Google Chrome\";v=\"95\", \"Chromium\";v=\"95\", \";Not A Brand\";v=\"99\"")
.header("sec-ch-ua-mobile","?0")
.header("sec-ch-ua-platform","\"Windows\"")
.header("sec-fetch-dest","document")
.header("sec-fetch-mode","navigate")
.header("sec-fetch-site","none")
.header("sec-fetch-user","?1")
.header("upgrade-insecure-requests","1");
// .header("x-forwarded-for", ip4.as_str())
//     .header("x-real-ip", ip4.as_str())
    
    let res = client.send().await?;
    res.headers().iter().for_each(|h| {
        log::error!(
            "{}: {}",
            h.0.as_str(),
            String::from_utf8(h.1.as_bytes().to_vec()).unwrap()
        );
    });
    res.text().await
}
pub async fn extract_jable(
    url: &str,
    cookie: &str,
    is_detail: bool,
) -> Result<Video, Box<dyn Error>> {
    log::error!("{}\n{}", url, cookie);
    let config = Config::new(Some("http://127.0.0.1:10809"), None);
    let res = match fetch_jable(&url, &config).await {
        Ok(res) => res,
        Err(err) => {
            // e
            log::error!("extract_jable: {}", err);
            String::new()
        }
    };
    log::error!("{}", res);
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
