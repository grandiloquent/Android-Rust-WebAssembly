use std::sync::Arc;

use serde::__private::doc;
use serde_json::Value;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{HtmlElement, Request, RequestInit, Response};

use crate::log;

use super::dom::render_item;

async fn load_videos(base_uri: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/videos/list", base_uri);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}

pub fn render() {
    spawn_local(async move {
        let videos = load_videos("http://192.168.8.55:3000").await;
        let videos = videos.unwrap().as_string().unwrap();
        let obj: Value = serde_json::from_str(&videos).unwrap();
        let window = web_sys::window().expect("Couldn't get window");
        let document = window.document().expect("Couldn't get document");
        let array = obj.as_array().unwrap();
        let parent = document
            .query_selector(".media-items")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let bottom_sheet_container = Arc::new(document
            .query_selector(".bottom-sheet-container")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap());
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

