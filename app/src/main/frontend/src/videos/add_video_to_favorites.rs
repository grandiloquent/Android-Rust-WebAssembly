use std::rc::Rc;

use crate::utils::{get_base_uri, hidden_element, query_element};

use wasm_bindgen::JsCast;
use wasm_bindgen::{prelude::Closure, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::Element;
use web_sys::{Request, RequestInit, Response};

fn get_data_id(bottom_sheet_container: Rc<Element>) -> Result<String, JsValue> {
    match bottom_sheet_container.get_attribute("data-id") {
        Some(id) => Ok(id),
        None => Err(JsValue::from_str("s")),
    }
}
pub fn execute_add_video_to_favorites(bottom_sheet_container: Rc<Element>) -> Closure<dyn FnMut()> {
    Closure::wrap(Box::new(move || {
        let _ = hidden_element(&bottom_sheet_container);
        let id = get_data_id(bottom_sheet_container.clone()).unwrap();
        spawn_local(async move {
            let _ = favorite_video(id.as_str()).await;
            query_element(format!(".media_item[data-id=\"{}\"]", id).as_str())
                .unwrap()
                .remove();
        });
    }) as Box<dyn FnMut()>)
}

async fn favorite_video(id: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/video/fav?id={}", get_base_uri(), id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
