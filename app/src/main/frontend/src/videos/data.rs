use std::{sync::{atomic::{AtomicU16, Ordering}, Arc}};

use serde_json::Value;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, Request, RequestInit, Response,
};

use crate::log;

use super::dom::render_item;

static mut OFFSET: AtomicU16 = AtomicU16::new(0);

pub fn render() {
    // log

    // spawn_local(async move {
    //     let window = web_sys::window().expect("Couldn't get window");
    //     let document = window.document().expect("Couldn't get document");
    //     let obj: Value = load_video_list(
    //         unsafe {
    //             OFFSET
    //                 .fetch_add(20, Ordering::SeqCst)
    //                 .into()
    //         },
    //         20,
    //     )
    //     .await
    //     .unwrap();
    //     let array = obj.as_array().unwrap();
    //     let parent = document
    //         .query_selector(".media-items")
    //         .unwrap()
    //         .unwrap()
    //         .dyn_into::<HtmlElement>()
    //         .unwrap();
    //     let bottom_sheet_container = Arc::new(
    //         document
    //             .query_selector(".bottom-sheet-container")
    //             .unwrap()
    //             .unwrap()
    //             .dyn_into::<HtmlElement>()
    //             .unwrap(),
    //     );
    //     array.iter().for_each(|x| {
    //         render_item(
    //             &document,
    //             &parent,
    //             bottom_sheet_container.clone(),
    //             x["id"].as_i64().unwrap(),
    //             x["image"].as_str().unwrap(),
    //             x["title"].as_str().unwrap(),
    //             x["uri"].as_str().unwrap(),
    //         );
    //     });
    // });
    {
        let window = web_sys::window().expect("Couldn't get window");
        let document = window.document().expect("Couldn't get document");
        let load_more = document.query_selector(".load-more").unwrap().unwrap();
            log(format!(
                "{}",
               "load"
            )
            .as_str());
        let handler = Closure::<dyn Fn(Vec<IntersectionObserverEntry>)>::new(
            move |entries: Vec<IntersectionObserverEntry>| {
                // Get the right entry, and make sure it's currently intersecting (this callback
                // will fire if it starts or stops intersecting)
                for entry in entries.iter() {
                    if entry.is_intersecting() {
                        spawn_local(async move {
                            let window = web_sys::window().expect("Couldn't get window");
                            let document = window.document().expect("Couldn't get document");
                            let obj: Value = load_video_list(unsafe {
                                OFFSET
                                    .fetch_add(20, Ordering::SeqCst)
                                    .into()
                            }, 20).await.unwrap();
                            let array = obj.as_array().unwrap();
                            let parent = document
                                .query_selector(".media-items")
                                .unwrap()
                                .unwrap()
                                .dyn_into::<HtmlElement>()
                                .unwrap();
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
                }
            },
        );
        let observer = IntersectionObserver::new(handler.as_ref().dyn_ref().unwrap()).unwrap();
        observer.observe(&load_more);
        handler.forget();
    }
}
async fn load_video_list(offset: u32, limit: u32) -> Result<Value, JsValue> {
    let base_uri = get_base_address();
    let videos = match load_videos(base_uri, offset, limit).await?.as_string() {
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
fn get_base_address() -> &'static str {
    if web_sys::window().unwrap().location().host().unwrap() == "127.0.0.1:5500" {
        "http://192.168.0.109:3000"
    } else {
        ""
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
