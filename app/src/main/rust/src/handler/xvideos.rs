use reqwest::Proxy;
use rocket::http::Status;
pub async fn fetch(url: &str) -> reqwest::Result<String> {
    let proxy = reqwest::Proxy::http("http://127.0.0.1:10809")?;
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3 Mobile/15E148 Safari/604.1")
        .proxy(proxy)
        .build()?;
    client.get(url).send().await?.text().await
}
#[get("/xvideos?<url>")]
pub async fn parse(url: String) -> Status {
    log::error!("{}",url);
    let res = fetch(&url);
    log::error!("{}",url);
    match res.await {
        Ok(res) => {
            log::error!("{}", res);
        }
        Err(err) => {
            log::error!("{}",err);
        }
    }
    Status::Ok
}