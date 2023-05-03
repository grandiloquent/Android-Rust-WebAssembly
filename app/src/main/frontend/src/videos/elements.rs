use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, Request, RequestInit, Response};

use crate::utils::get_base_uri;


pub fn create_menu_item(
    document: &Document,
    bottom_sheet_content: &Element,
    text_content: &str,
    handler: Closure<dyn FnMut()>,
) -> Result<(), JsValue> {
    let menu_item = document.create_element("div")?;
    let _ = menu_item.set_attribute("class", "menu-item");
    let _ = bottom_sheet_content.append_child(&menu_item);
    let menu_item_button = document.create_element("button").expect("button");
    let _ = menu_item_button.set_attribute("class", "menu-item-button");
    let _ = menu_item_button.set_text_content(Some(text_content));
    let _ = menu_item.append_child(&menu_item_button);
    menu_item.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap())?;
    handler.forget();
    Ok(())
}
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
