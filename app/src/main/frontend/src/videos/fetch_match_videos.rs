use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

use crate::utils::get_base_uri;

async fn search_videos(q: &str, offset: u32, limit: u32) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!(
        "{}/videos/search?q={}&offset={}&limit={}",
        get_base_uri(),
        q,
        offset,
        limit
    );
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
pub async fn fetch_match_videos(q: &str, offset: u32, limit: u32) -> Result<Value, JsValue> {
    let s = search_videos(q, offset, limit).await?;
    if let Some(v) = s.as_string() {
        if let Ok(value) = serde_json::from_str(v.as_str()) {
            return Ok(value);
        }
    }
    Err(JsValue::from_str(""))
}
