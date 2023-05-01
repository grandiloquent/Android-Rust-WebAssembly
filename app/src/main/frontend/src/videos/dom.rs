use std::{fmt::format, rc::Rc, sync::Arc};

use urlencoding::encode;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{Document, HtmlElement, Request, RequestInit, Response};

use crate::log;

use super::{
    elements::{create_bottom_tab_item, create_menu_item, open_page_with_id},
    utils::{bind_click_event, hidden_element},
};

pub fn render_item(
    document: &Document,
    parent: &HtmlElement,
    bottom_sheet_container: Arc<HtmlElement>,
    id: i64,
    src: &str,
    title: &str,
    uri: &str,
) {
    let href = format!("/video.html?url={}", encode(uri));
    let media_item = document.create_element("div").expect("div");
    let _ = media_item.set_attribute("class", "media_item");
    let _ = media_item.set_attribute("data-id", id.to_string().as_str());
    let _ = parent.append_child(&media_item);
    let media_item_thumbnail_container = document.create_element("a").expect("a");
    let _ = media_item_thumbnail_container.set_attribute("class", "media_item_thumbnail_container");
    let _ = media_item_thumbnail_container.set_attribute("href", &href);
    let _ = media_item.append_child(&media_item_thumbnail_container);
    let video_thumbnail_container_large = document.create_element("div").expect("div");
    let _ =
        video_thumbnail_container_large.set_attribute("class", "video_thumbnail_container_large");
    let _ = media_item_thumbnail_container.append_child(&video_thumbnail_container_large);
    let video_thumbnail_bg = document.create_element("div").expect("div");
    let _ = video_thumbnail_bg.set_attribute("class", "video_thumbnail_bg");
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_bg);
    let video_thumbnail_img = document.create_element("img").expect("img");
    let _ = video_thumbnail_img.set_attribute("class", "video_thumbnail_img");
    let _ = video_thumbnail_img.set_attribute("src", src);
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_img);
    let details = document.create_element("div").expect("div");
    let _ = details.set_attribute("class", "details");
    let _ = media_item.append_child(&details);
    let media_channel = document.create_element("div").expect("div");
    let _ = media_channel.set_attribute("class", "media_channel");
    let _ = details.append_child(&media_channel);
    let media_item_info = document.create_element("div").expect("div");
    let _ = media_item_info.set_attribute("class", "media_item_info");
    let _ = details.append_child(&media_item_info);
    let media_item_metadata = document.create_element("div").expect("div");
    let _ = media_item_metadata.set_attribute("class", "media_item_metadata");
    let _ = media_item_info.append_child(&media_item_metadata);
    let a = document.create_element("a").expect("a");
    let _ = a.set_attribute("class", "a");
    let _ = a.set_attribute("href", &href);
    let _ = media_item_metadata.append_child(&a);
    let media_item_headline = document.create_element("h3").expect("h3");
    let _ = media_item_headline.set_attribute("class", "media_item_headline");
    let _ = a.append_child(&media_item_headline);
    let _ = media_item_headline.set_text_content(Some(title));
    let badge_and_byline_renderer = document.create_element("div").expect("div");
    let _ = badge_and_byline_renderer.set_attribute("class", "badge_and_byline_renderer");
    let _ = a.append_child(&badge_and_byline_renderer);
    let bottom_sheet_renderer = document.create_element("div").expect("div");
    let _ = bottom_sheet_renderer.set_attribute("class", "bottom_sheet_renderer");
    let _ = media_item_info.append_child(&bottom_sheet_renderer);
    let button = document.create_element("button").expect("button");
    let _ = button.set_attribute("class", "button");
    let _ = bottom_sheet_renderer.append_child(&button);
    let c3_icon = document.create_element("div").expect("div");
    let _ = c3_icon.set_attribute("class", "c3_icon");
    let _ = button.append_child(&c3_icon);
    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .expect("svg");
    let _ = svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg.set_attribute("enable-background", "new 0 0 24 24");
    let _ = svg.set_attribute("height", "24");
    let _ = svg.set_attribute("viewBox", "0 0 24 24");
    let _ = svg.set_attribute("width", "24");
    let _ = c3_icon.append_child(&svg);
    let path = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        .expect("path");
    let _ = path.set_attribute("d","M12,16.5c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S11.17,16.5,12,16.5z M10.5,12 c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S10.5,11.17,10.5,12z M10.5,6c0,0.83,0.67,1.5,1.5,1.5 s1.5-0.67,1.5-1.5S12.83,4.5,12,4.5S10.5,5.17,10.5,6z");
    let _ = svg.append_child(&path);

    // click
    let bottom_sheet_container = Arc::new(bottom_sheet_container);

    {
        let bottom_sheet_container = bottom_sheet_container.clone();
        let handler = Closure::wrap(Box::new(move || {
            let _ = bottom_sheet_container.remove_attribute("style");
            let _ = bottom_sheet_container.set_attribute("data-id", id.to_string().as_str());
        }) as Box<dyn FnMut()>);
        button
            .dyn_into::<HtmlElement>()
            .unwrap()
            .set_onclick(handler.as_ref().dyn_ref());
        handler.forget();
    }
}

pub fn build_bottom_bar() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let bar_renderer = document.create_element("div").expect("div");
    let _ = bar_renderer.set_attribute("class", "bar-renderer");
    let body = document
        .body()
        .expect("document expect to have have a body");
    let _ = body.append_child(&bar_renderer);
    let bar_item_renderer = document.create_element("div").expect("div");
    let _ = bar_item_renderer.set_attribute("class", "bar-item-renderer");
    let _ = bar_renderer.append_child(&bar_item_renderer);

    let _ = create_bottom_tab_item(
        &document,
        &bar_item_renderer,
        "tab-item-home",
        "M4,21V10.08l8-6.96l8,6.96V21h-6v-6h-4v6H4z",
        "主页",
    );
    let _ = create_bottom_tab_item(
        &document,
        &bar_item_renderer,
        "tab-item-add",
        "M18.984 12.984h-6v6h-1.969v-6h-6v-1.969h6v-6h1.969v6h6v1.969z",
        "添加",
    );
}

pub fn build_bottom_sheet() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let bottom_sheet_container = document.create_element("div").expect("div");
    let _ = bottom_sheet_container.set_attribute("class", "bottom-sheet-container");
    let _ = bottom_sheet_container.set_attribute("style", "display: none");

    let body = document
        .body()
        .expect("document expect to have have a body");
    let _ = body.append_child(&bottom_sheet_container);
    let bottom_sheet_overlay = document.create_element("div").expect("div");
    let _ = bottom_sheet_overlay.set_attribute("class", "bottom-sheet-overlay");
    let _ = bottom_sheet_container.append_child(&bottom_sheet_overlay);
    let hidden_button = document.create_element("button").expect("button");
    let _ = hidden_button.set_attribute("class", "hidden-button");
    let _ = bottom_sheet_overlay.append_child(&hidden_button);
    let bottom_sheet_layout = document.create_element("div").expect("div");
    let _ = bottom_sheet_layout.set_attribute("class", "bottom-sheet-layout");
    let _ = bottom_sheet_container.append_child(&bottom_sheet_layout);
    let bottom_sheet_renderer_container = document.create_element("div").expect("div");
    let _ =
        bottom_sheet_renderer_container.set_attribute("class", "bottom-sheet-renderer-container");
    let _ = bottom_sheet_layout.append_child(&bottom_sheet_renderer_container);
    let bottom_sheet_layout_header_wrapper = document.create_element("div").expect("div");
    let _ = bottom_sheet_layout_header_wrapper
        .set_attribute("class", " bottom-sheet-layout-header-wrapper");
    let _ = bottom_sheet_renderer_container.append_child(&bottom_sheet_layout_header_wrapper);
    let bottom_sheet_drag_line = document.create_element("div").expect("div");
    let _ = bottom_sheet_drag_line.set_attribute("class", "bottom-sheet-drag-line");
    let _ = bottom_sheet_layout_header_wrapper.append_child(&bottom_sheet_drag_line);
    let bottom_sheet_layout_header = document.create_element("div").expect("div");
    let _ = bottom_sheet_layout_header.set_attribute("class", "bottom-sheet-layout-header");
    let _ = bottom_sheet_layout_header_wrapper.append_child(&bottom_sheet_layout_header);
    let bottom_sheet_layout_header_title_wrapper = document.create_element("div").expect("div");
    let _ = bottom_sheet_layout_header_title_wrapper
        .set_attribute("class", "bottom-sheet-layout-header-title-wrapper");
    let _ = bottom_sheet_layout_header.append_child(&bottom_sheet_layout_header_title_wrapper);
    let bottom_sheet_layout_content_wrapper = document.create_element("div").expect("div");
    let _ = bottom_sheet_layout_content_wrapper
        .set_attribute("class", "bottom-sheet-layout-content-wrapper");
    let _ = bottom_sheet_renderer_container.append_child(&bottom_sheet_layout_content_wrapper);
    let bottom_sheet_content = document.create_element("div").expect("div");
    let _ = bottom_sheet_content.set_attribute("class", "bottom-sheet-content");
    let _ = bottom_sheet_layout_content_wrapper.append_child(&bottom_sheet_content);
    let bottom_sheet_container = Rc::new(bottom_sheet_container);
    {
        let bottom_sheet_container = bottom_sheet_container.clone();
        let _ = create_menu_item(
            &document,
            &bottom_sheet_content,
            "打开",
            Closure::wrap(Box::new(move || {
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
            }) as Box<dyn FnMut()>),
        );
    }
    {
        let bottom_sheet_container = bottom_sheet_container.clone();
        let _ = create_menu_item(
            &document,
            &bottom_sheet_content,
            "删除",
            Closure::wrap(Box::new(move || {
                let _ = hidden_element(&bottom_sheet_container);
                let id = match bottom_sheet_container.get_attribute("data-id") {
                    Some(id) => id,
                    None => {
                        return;
                    }
                };
                spawn_local(async move {
                    let _ = delete_video("", id.as_str()).await;
                    let window = web_sys::window().expect("global window does not exists");
                    let _ = window.location().reload();
                });
            }) as Box<dyn FnMut()>),
        );
    }
    {
        let bottom_sheet_container = bottom_sheet_container.clone();
        let handler = Closure::wrap(Box::new(move || {
            let _ = hidden_element(&bottom_sheet_container);
        }) as Box<dyn FnMut()>);
        let _ = bind_click_event(&bottom_sheet_overlay, handler);
    }
}

async fn delete_video(base_uri: &str, id: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("{}/videos/hidden?id={}", base_uri, id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
