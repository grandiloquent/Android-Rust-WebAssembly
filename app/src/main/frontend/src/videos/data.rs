use std::sync::atomic::{AtomicU16, Ordering};

use once_cell::sync::OnceCell;
use send_wrapper::SendWrapper;
use serde_json::Value;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    Element, Event,  IntersectionObserver, IntersectionObserverEntry, Request,
    RequestInit, Response,
};

use crate::{
    utils::{get_base_uri, query_element},
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
    let base_uri = get_base_uri();
    let videos = match load_videos(base_uri.as_str(), offset, limit)
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

async fn load_videos(base_uri: &str, offset: u32, limit: u32) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/videos/list?offset={}&limit={}", base_uri, offset, limit);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
