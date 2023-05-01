use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, Request, RequestInit, Response};

use crate::log;

// "M4,20h14v1H3V6h1V20z M21,3v15H6V3H21z M17,10.5L11,7v7L17,10.5z"
//
pub fn create_bottom_tab_item(
    document: &Document,
    bar_item_renderer: &Element,
    class: &str,
    d: &str,
    name: &str,
) -> Result<(), JsValue> {
    let bar_item_tab = document.create_element("div")?;
    let _ = bar_item_tab.set_attribute("class", format!("bar-item-tab {}", class).as_str());
    let _ = bar_item_renderer.append_child(&bar_item_tab);
    let c3_icon = document.create_element("div")?;
    let _ = c3_icon.set_attribute("class", "c3-icon");
    let _ = bar_item_tab.append_child(&c3_icon);
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?;
    let _ = svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg.set_attribute("enable-background", "new 0 0 24 24");
    let _ = svg.set_attribute("height", "24");
    let _ = svg.set_attribute("viewBox", "0 0 24 24");
    let _ = svg.set_attribute("width", "24");
    let _ = c3_icon.append_child(&svg);
    let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path")?;
    let _ = path.set_attribute("d", d);
    let _ = svg.append_child(&path);
    let pivot_bar_item_title = document.create_element("div")?;
    let _ = pivot_bar_item_title.set_attribute("class", "pivot-bar-item-title");
    let _ = bar_item_tab.append_child(&pivot_bar_item_title);
    let _ = pivot_bar_item_title.set_text_content(Some(name));
    Ok(())
}
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
    let base_uri = if web_sys::window().unwrap().location().host().unwrap() == "127.0.0.1:5500" {
        "http://192.168.0.109:3000"
    } else {
        ""
    };
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
