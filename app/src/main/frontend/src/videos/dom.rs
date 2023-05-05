use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Element, Request, RequestInit, Response};

use crate::utils::{get_base_uri, hidden_element, query_element};

use super::{add_video_to_favorites::execute_add_video_to_favorites, elements::open_page_with_id};
fn bind_action_open(bottom_sheet_container: Rc<Element>) {
    let element = query_element(".menu-item-open").unwrap();
    let handler = Closure::wrap(Box::new(move || {
        let _ = hidden_element(&bottom_sheet_container);
        let id = match bottom_sheet_container.get_attribute("data-id") {
            Some(id) => id,
            None => {
                return;
            }
        };
        spawn_local(async move {
            let _ = open_page_with_id(id.as_str()).await;
        });
    }) as Box<dyn FnMut()>);
    let _ = element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
}
pub fn initialize_bottom_sheet() {
    let bottom_sheet_container = Rc::new(query_element(".bottom-sheet-container").unwrap());
    let _ = bind_action_open(bottom_sheet_container.clone());
    let _ = bind_action_fav(bottom_sheet_container.clone());
    let _ = bind_action_delete(bottom_sheet_container.clone());
    let _ = bind_action_overlay(bottom_sheet_container.clone());
}

async fn delete_video(id: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/videos/hidden?id={}", get_base_uri(), id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
fn bind_action_fav(bottom_sheet_container: Rc<Element>) {
    let element = query_element(".menu-item-fav").unwrap();
    let handler = execute_add_video_to_favorites(bottom_sheet_container);
    let _ = element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
}
fn bind_action_delete(bottom_sheet_container: Rc<Element>) {
    let element = query_element(".menu-item-delete").unwrap();
    let handler = Closure::wrap(Box::new(move || {
        let _ = hidden_element(&bottom_sheet_container);
        let id = match bottom_sheet_container.get_attribute("data-id") {
            Some(id) => id,
            None => {
                return;
            }
        };
        spawn_local(async move {
            let _ = delete_video(id.as_str()).await;
            query_element(format!(".media_item[data-id=\"{}\"]", id).as_str())
                .unwrap()
                .remove();
            //let _ = window.location().reload();
        });
    }) as Box<dyn FnMut()>);
    let _ = element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
}
fn bind_action_overlay(bottom_sheet_container: Rc<Element>) {
    let element = query_element(".bottom-sheet-overlay").unwrap();
    let handler = Closure::wrap(Box::new(move || {
        let _ = hidden_element(&bottom_sheet_container);
    }) as Box<dyn FnMut()>);
    let _ = element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
}
