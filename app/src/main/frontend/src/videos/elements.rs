use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

use crate::utils::get_base_uri;

pub async fn open_page_with_id(id: &str) -> Result<(), JsValue> {
    // base_uri
    let base_uri = get_base_uri();
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/video/url?id={}", base_uri, id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    let window = web_sys::window().expect("global window does not exists");
    let _ = window.open_with_url_and_target(json.as_string().unwrap().as_str(), "_blank");
    Ok(())
}
