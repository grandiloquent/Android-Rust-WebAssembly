use std::sync::Arc;

use crate::log;
use crate::utils::{adjust_size, StringExt};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::MouseEvent;
use web_sys::{Document, Element, HtmlElement, HtmlVideoElement};
pub fn append_bottom(document: &Document) -> HtmlElement {
    let div = build_div(document);
    let body = document
        .body()
        .expect("document expect to have have a body");
    let s = div.style();
    let _ = s.set_property("display", "flex");
    let _ = s.set_property("flex-direction", "column");
    let _ = s.set_property("justify-content", "space-between");
    let _ = s.set_property("position", "fixed");
    let _ = s.set_property("left", "16px");
    let _ = s.set_property("right", "16px");
    let _ = s.set_property("bottom", "0px");
    let _ = s.set_property("z-index", "1");
    let _ = body.append_child(&div);
    let wrapper = build_div(document);
    {
        let s = wrapper.style();
        let _ = s.set_property("display", "flex");
        let _ = s.set_property("justify-content", "space-between");
        let _ = s.set_property("align-items", "center");
        let _ = s.set_property("width", "100%");
    }

    let subdiv = {
        let div = build_div(document);
        let s = div.style();
        let _ = s.set_property("display", "flex");
        let _ = s.set_property("align-items", "center");
        let _ = s.set_property("pointer-events", "none");
        let _ = s.set_property("overflow", "hidden");
        div
    };
    let _ = wrapper.append_child(&subdiv);

    let _ = div.append_child(&wrapper);
    let first = {
        let div = build_div(document);
        div.set_class_name("first");
        let s = div.style();
        let _ = s.set_property("font-size", "1.2rem");
        let _ = s.set_property("color", "#fff");
        div.set_text_content(Some("0:00"));
        div
    };
    let _ = subdiv.append_child(&first);
    let delimiter = {
        let div = build_div(document);
        let s = div.style();
        let _ = s.set_property("color", "#fff");
        let _ = s.set_property("opacity", ".7");
        let _ = s.set_property("margin", "0 4px");
        div.set_text_content(Some("/"));
        div
    };
    let _ = subdiv.append_child(&delimiter);
    let second = {
        let div = build_div(document);
        div.set_class_name("second");
        let s = div.style();
        let _ = s.set_property("color", "#fff");
        let _ = s.set_property("opacity", ".7");
        div.set_text_content(Some("0:00"));
        div
    };
    let _ = subdiv.append_child(&second);
    append_space(document, &wrapper);
    append_playback_speed(document, &wrapper);
    append_random(document, &wrapper);
    append_fullscreen(document, &wrapper);
    let progress_bar_line = build_progress_bar_line(document);
    let _ = div.append_child(&progress_bar_line);
    div
}
pub fn append_middle(document: &Document) -> HtmlElement {
    let div = build_div(document);
    let body = document
        .body()
        .expect("document expect to have have a body");
    let svg = build_svg(&document, "M6,4l12,8L6,20V4z");
    let button = build_div(document);
    button.set_class_name("play");
    let _ = button.append_child(&svg);
    let s = button.style();
    let _ = s.set_property("color", "#fff");
    let _ = s.set_property("fill", "currentColor");
    let _ = s.set_property("margin", "0 60px");
    let _ = s.set_property("height", "56px");
    let _ = s.set_property("width", "56px");
    let _ = div.append_child(&button);
    let s = div.style();
    let _ = s.set_property("display", "flex");
    let _ = s.set_property("align-items", "center");
    let _ = s.set_property("justify-content", "center");
    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("top", "50%");
    let _ = s.set_property("width", "100%");
    let _ = s.set_property("transform", "translateY(-50%)");
    let _ = s.set_property("z-index", "1");
    let _ = body.append_child(&div);
    div
}
fn build_div(document: &Document) -> HtmlElement {
    document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap()
}
pub fn build_progress_bar_background(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_background");
    let s = div.style();
    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("height", "3px");
    let _ = s.set_property("background-color", "#fff");
    let _ = s.set_property("opacity", ".3");
    let _ = s.set_property("width", "100%");
    div
}
pub fn build_progress_bar_line(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_line");
    let s = div.style();
    let _ = s.set_property("height", "43px");
    let _ = s.set_property("width", "100%");
    let _ = s.set_property("align-items", "center");
    let _ = s.set_property("display", "flex");
    let progress_bar_background = build_progress_bar_background(document);
    let _ = div.append_child(&progress_bar_background);
    let progress_bar_loaded = build_progress_bar_loaded(document);
    let _ = div.append_child(&progress_bar_loaded);
    let progress_bar_played = build_progress_bar_played(document);
    let _ = div.append_child(&progress_bar_played);
    let progress_bar_playhead_wrapper = build_progress_bar_playhead_wrapper(document);
    let _ = div.append_child(&progress_bar_playhead_wrapper);
    div
}
pub fn build_progress_bar_loaded(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_loaded");
    let s = div.style();
    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("height", "3px");
    let _ = s.set_property("background-color", "#fff");
    let _ = s.set_property("opacity", ".6");
    div
}
fn build_progress_bar_played(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_played");
    let s = div.style();
    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("height", "3px");
    let _ = s.set_property("background-color", "#f00");
    div
}
pub fn build_progress_bar_playhead_dot(document: &Document) -> HtmlElement {
    //let progress_bar_playhead_dot = build_progress_bar_playhead_dot(document);
    //let _ = div.append_child(&progress_bar_playhead_dot);
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_playhead_dot");
    let s = div.style();
    let _ = s.set_property("display", "block");
    let _ = s.set_property("height", "12px");
    let _ = s.set_property("width", "12px");
    let _ = s.set_property("border-radius", "50%");
    let _ = s.set_property("background-color", "#f00");
    div
}
pub fn build_progress_bar_playhead_wrapper(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("progress_bar_playhead_wrapper");
    let s = div.style();
    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("z-index", "0");
    let progress_bar_playhead_dot = build_progress_bar_playhead_dot(document);
    let _ = div.append_child(&progress_bar_playhead_dot);
    div
}
pub fn build_svg(document: &Document, d: &str) -> Element {
    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .expect("svg");
    let _ = svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg.set_attribute("viewBox", "0 0 24 24");
    let path = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        .expect("path");
    let _ = path.set_attribute("d", d);
    let _ = svg.append_child(&path);
    svg
}
pub fn get_video(document: &Document) -> Result<HtmlVideoElement, JsValue> {
    let body = match document.body() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("Could not find body"));
        }
    };
    let video = document
        .create_element("video")?
        .dyn_into::<HtmlVideoElement>()?;
    if let Err(_) = body.append_child(&video) {
        return Err(JsValue::from_str("Could not find body"));
    };
    // C:\Users\Administrator\.cargo\registry\src\github.com-1ecc6299db9ec823\web-sys-0.3.61\src\features
    Ok(video)
}
pub fn set_ondurationchange(second: Arc<HtmlElement>, video: Arc<HtmlVideoElement>) {
    let v = video.clone();
    let ondurationchange = Closure::wrap(Box::new(move || {
        let duration = v.duration() as i32;
        let seconds = duration % 60;
        let minutes = (duration / 60) % 60;
        let hours = (duration / 60) / 60;
        if hours > 0 {
            second.set_text_content(Some(
                format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds).as_str(),
            ));
        } else {
            second.set_text_content(Some(format!("{}:{:0>2}", minutes, seconds).as_str()));
        }
        adjust_size(&v);
    }) as Box<dyn FnMut()>);
    video.set_ondurationchange(Some(ondurationchange.as_ref().unchecked_ref()));
    ondurationchange.forget();
}
pub fn set_onpause(button: Arc<HtmlElement>, video: Arc<HtmlVideoElement>) {
    let button = button.clone();
    let onpause = Closure::wrap(Box::new(move || {
        let path = button.query_selector("path").unwrap().unwrap();
        let _ = path.set_attribute("d", "M6,4l12,8L6,20V4z");
    }) as Box<dyn FnMut()>);
    video.set_onpause(Some(onpause.as_ref().unchecked_ref()));
    onpause.forget();
}

pub fn set_onprogress(progress_bar_loaded: Arc<HtmlElement>, video: Arc<HtmlVideoElement>) {
    let v = video.clone();
    let onprogress = Closure::wrap(Box::new(move || {
        let duration = v.duration();
        let time_ranges = v.buffered();
        if time_ranges.length() > 0 {
            let end = time_ranges.end(0).unwrap_or(0.0);
            let s = progress_bar_loaded.style();
            let _ = s.set_property("width", format!("{}%", end / duration * 100.0).as_str());
        }
    }) as Box<dyn FnMut()>);
    video.set_onprogress(Some(onprogress.as_ref().unchecked_ref()));
    onprogress.forget();
}
pub fn set_ontimeupdate(
    first: Arc<HtmlElement>,
    progress_bar_played: Arc<HtmlElement>,
    progress_bar_playhead_wrapper: Arc<HtmlElement>,
    video: Arc<HtmlVideoElement>,
) {
    let v = video.clone();
    let ontimeupdate = Closure::wrap(Box::new(move || {
        let t = v.current_time() as i32;
        let seconds = t % 60;
        let minutes = (t / 60) % 60;
        let hours = (t / 60) / 60;
        if hours > 0 {
            first.set_text_content(Some(
                format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds).as_str(),
            ));
        } else {
            first.set_text_content(Some(format!("{}:{:0>2}", minutes, seconds).as_str()));
        }
        let current_time = v.current_time();
        let s = progress_bar_played.style();
        let _ = s.set_property(
            "width",
            format!("{}%", current_time / v.duration() * 100.0).as_str(),
        );
        let s = progress_bar_playhead_wrapper.style();
        let _ = s.set_property(
            "margin-left",
            format!("{}%", current_time / v.duration() * 100.0).as_str(),
        );
    }) as Box<dyn FnMut()>);
    video.set_ontimeupdate(Some(ontimeupdate.as_ref().unchecked_ref()));
    ontimeupdate.forget();
}
pub fn set_progress_click(element: Arc<HtmlElement>, video: Arc<HtmlVideoElement>) {
    let e = element.clone();
    let handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        let offset_x = event.offset_x() as f64;
        let width = e.get_bounding_client_rect().width();
        video.set_current_time(offset_x / width * video.duration());
    }) as Box<dyn FnMut(_)>);
    element.set_onclick(handler.as_ref().dyn_ref());
    handler.forget();
}
pub fn append_track(document: &Document, video: Arc<HtmlVideoElement>, src: &str) {
    let src = format!(
        "{}{}",
        src.to_string()
            .replace("/api/file", "/subtitle")
            .substring_before_last("."),
        ".srt"
    );
    log(src.as_str());

    let element = document.create_element("track").expect("track");
    let _ = element.set_attribute("default", "");
    let _ = element.set_attribute("kind", "captions");
// src.as_str()
    let _ = element.set_attribute("src", "/pkg/1.vtt");
    let _ = video.append_child(&element);
}

fn append_fullscreen(document: &Document, wrapper: &HtmlElement) {
    let fullscreen = {
        let div = build_div(document);
        div.set_class_name("fullscreen button");
        let svg = build_svg(
            document,
            "M7,11H6V6h5v1H7V11z M18,6h-5v1h4v4h1V6z M18,13h-1v4h-4v1h5V13z M11,17H7v-4H6v5h5V17z",
        );
        let _ = div.append_child(&svg);
        div
    };
    let _ = wrapper.append_child(&fullscreen);
}
fn append_playback_speed(document: &Document, wrapper: &HtmlElement) {
    let playback_speed = {
        let div = build_div(document);
        div.set_class_name("playback_speed button");
        let svg = build_svg(
            document,
            "M10,8v8l6-4L10,8L10,8z M6.3,5L5.7,4.2C7.2,3,9,2.2,11,2l0.1,1C9.3,3.2,7.7,3.9,6.3,5z            M5,6.3L4.2,5.7C3,7.2,2.2,9,2,11 l1,.1C3.2,9.3,3.9,7.7,5,6.3z            M5,17.7c-1.1-1.4-1.8-3.1-2-4.8L2,13c0.2,2,1,3.8,2.2,5.4L5,17.7z            M11.1,21c-1.8-0.2-3.4-0.9-4.8-2 l-0.6,.8C7.2,21,9,21.8,11,22L11.1,21z            M22,12c0-5.2-3.9-9.4-9-10l-0.1,1c4.6,.5,8.1,4.3,8.1,9s-3.5,8.5-8.1,9l0.1,1 C18.2,21.5,22,17.2,22,12z",
        );
        let _ = div.append_child(&svg);
        div
    };
    let _ = wrapper.append_child(&playback_speed);
}
fn append_random(document: &Document, wrapper: &HtmlElement) {
    let random = {
        let div = build_div(document);
        div.set_class_name("random button");
        let svg = build_svg(
            document,
            "M15,17h6v1h-6V17z M11,17H3v1h8v2h1v-2v-1v-2h-1V17z M14,8h1V6V5V3h-1v2H3v1h11V8z            M18,5v1h3V5H18z M6,14h1v-2v-1V9H6v2H3v1 h3V14z M10,12h11v-1H10V12z",
        );
        let _ = div.append_child(&svg);
        div
    };
    let _ = wrapper.append_child(&random);
}
fn append_space(document: &Document, wrapper: &HtmlElement) {
    let space = {
        let div = build_div(document);
        let s = div.style();
        let _ = s.set_property("flex-grow", "1");
        div
    };
    let _ = wrapper.append_child(&space);
}
