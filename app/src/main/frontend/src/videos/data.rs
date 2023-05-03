use std::sync::atomic::{AtomicU16, Ordering};

use once_cell::sync::OnceCell;
use send_wrapper::SendWrapper;
use serde_json::Value;
use urlencoding::encode;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    Element, Event, IntersectionObserver, IntersectionObserverEntry, Request, RequestInit,
    Response, Url,
};

use crate::{
    log,
    utils::{get_base_url, get_string, query_element},
};

use super::render_video_list::render_video_list;

static mut OFFSET: AtomicU16 = AtomicU16::new(0);
pub static HANLDER: once_cell::sync::OnceCell<SendWrapper<Closure<dyn FnMut(Event)>>> =
    OnceCell::new();

pub fn render() {
    HANLDER.get_or_init(|| {
        SendWrapper::new(Closure::wrap(Box::new(move |e: Event| {
            let current_target = match e.current_target() {
                Some(v) => v,
                None => {
                    return;
                }
            };
            let element = match current_target.dyn_into::<Element>() {
                Ok(v) => v,
                Err(_) => {
                    return;
                }
            };
            let bottom_sheet_container = query_element(".bottom-sheet-container").unwrap();
            let _ = bottom_sheet_container.set_attribute(
                "data-id",
                element.get_attribute("data-id").unwrap().as_str(),
            );
            let _ = bottom_sheet_container.set_attribute("style", "display: flex");
        }) as Box<dyn FnMut(Event)>))
    });
    {
        let window = web_sys::window().expect("Couldn't get window");
        let document = window.document().expect("Couldn't get document");
        let load_more = document.query_selector(".load-more").unwrap().unwrap();
        let handler = Closure::<dyn Fn(Vec<IntersectionObserverEntry>)>::new(
            move |entries: Vec<IntersectionObserverEntry>| {
                // Get the right entry, and make sure it's currently intersecting (this callback
                // will fire if it starts or stops intersecting)
                for entry in entries.iter() {
                    if entry.is_intersecting() {
                        spawn_local(async move {
                            let obj: Value = load_video_list(
                                unsafe { OFFSET.fetch_add(20, Ordering::SeqCst).into() },
                                20,
                            )
                            .await
                            .unwrap();
                            let obj = obj.as_array().unwrap();
                            let _ = render_video_list(obj);
                            ()
                        });
                    }
                }
            },
        );
        let observer = IntersectionObserver::new(handler.as_ref().dyn_ref().unwrap()).unwrap();
        observer.observe(&load_more);
        handler.forget();
    }
}
async fn load_video_list(offset: u32, limit: u32) -> Result<Value, JsValue> {
    let base_url = get_base_url().unwrap();
    base_url.set_pathname("/videos/list");
    base_url
        .search_params()
        .set("offset", offset.to_string().as_str());
    base_url
        .search_params()
        .set("limit", limit.to_string().as_str());
    let url = Url::new(
        web_sys::window()
            .unwrap()
            .location()
            .href()
            .unwrap()
            .as_str(),
    )
    .unwrap();

    if let Some(q) = url.search_params().get("q") {
        base_url
            .search_params()
            .set("q", encode(q.as_str()).to_string().as_str());
    }
    if let Some(q) = url.search_params().get("t") {
        base_url
            .search_params()
            .set("t", encode(q.as_str()).to_string().as_str());
    }
    let videos = match get_string(base_url.to_string().as_string().unwrap().as_str())
        .await?
        .as_string()
    {
        Some(v) => v,
        None => {
            return Err("")?;
        }
    };
    match serde_json::from_str(&videos) {
        Ok(v) => Ok(v),
        Err(err) => {
            return Err(err.to_string())?;
        }
    }
}