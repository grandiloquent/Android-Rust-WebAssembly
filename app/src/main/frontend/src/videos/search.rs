use serde_json::Value;
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;

use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

use crate::log;
use crate::utils::get_base_uri;

use super::dom::render_item;

pub fn initialize_search() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let share = document.query_selector(".topbar-header").unwrap().unwrap();
    let share = Rc::new(share);
    let _ = initialize_search_button(&document, share.clone());
    let _ = initialize_search_back(&document, share.clone());
    let _ = bind_input(&document, share.clone());
}

fn initialize_search_button(document: &Document, share: Rc<Element>) -> Result<(), JsValue> {
    let element = match document.query_selector(".topbar-menu-button")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };
    {
        let share = share.clone();
        let handler =
            Closure::wrap(
                Box::new(move || share.set_class_name("topbar-header search-on"))
                    as Box<dyn FnMut()>,
            );
        let _ =
            element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
        handler.forget();
    }
    Err("")?
}

fn initialize_search_back(document: &Document, share: Rc<Element>) -> Result<(), JsValue> {
    let element = match document.query_selector(".topbar-back-arrow")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };

    {
        let share = share.clone();
        let handler = Closure::wrap(
            Box::new(move || share.set_class_name("topbar-header")) as Box<dyn FnMut()>
        );
        let _ =
            element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
        handler.forget();
    }
    Err("")?
}

pub fn bind_input(document: &Document, share: Rc<Element>) -> Result<(), JsValue> {
    let element = query_input(document)?;

    {
        let share = share.clone();
        let ev = element.clone();
        let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
            share.set_class_name("topbar-header search-on");
            log(format!("{}", e.key()).as_str());
            if e.key() == "Enter" {
                let ev = ev.clone();
                spawn_local(async move {
                    let window = web_sys::window().expect("Couldn't get window");
                    let document = window.document().expect("Couldn't get document");
                    let obj: Value = serde_json::from_str(
                        search_videos(ev.value().as_str(), 0, 20)
                            .await
                            .unwrap()
                            .as_string()
                            .unwrap()
                            .as_str(),
                    )
                    .unwrap();
                    let array = obj.as_array().unwrap();
                    let parent = document
                        .query_selector(".media-items")
                        .unwrap()
                        .unwrap()
                        .dyn_into::<HtmlElement>()
                        .unwrap();
                    parent.set_inner_html("");
                    let bottom_sheet_container = Arc::new(
                        document
                            .query_selector(".bottom-sheet-container")
                            .unwrap()
                            .unwrap()
                            .dyn_into::<HtmlElement>()
                            .unwrap(),
                    );
                    array.iter().for_each(|x| {
                        render_item(
                            &document,
                            &parent,
                            bottom_sheet_container.clone(),
                            x["id"].as_i64().unwrap(),
                            x["image"].as_str().unwrap(),
                            x["title"].as_str().unwrap(),
                            x["uri"].as_str().unwrap(),
                        );
                    });
                });
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        let _ =
            element.add_event_listener_with_callback("keydown", handler.as_ref().dyn_ref().unwrap());
        handler.forget();
    }
    Err("")?
}
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
fn query_input(document: &Document) -> Result<Rc<HtmlInputElement>, JsValue> {
    let element = match document.query_selector(".searchbox-input")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };
    match element.dyn_into::<HtmlInputElement>() {
        Ok(v) => Ok(Rc::new(v)),
        Err(_) => return Err("")?,
    }
}
