mod elements;
mod listener;
mod utils;

use send_wrapper::SendWrapper;
use std::sync::Arc;
use utils::query_selector;

use elements::{
    append_bottom, append_middle, append_track, get_video, set_ondurationchange, set_onpause,
    set_onprogress, set_ontimeupdate, set_progress_click,
};
use listener::Listner;
use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlVideoElement};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(callback: &Closure<dyn FnMut()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(token: f64);

    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(callback: &Closure<dyn FnMut()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(token: f64);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

static HANLDER: once_cell::sync::OnceCell<SendWrapper<Closure<dyn FnMut()>>> = OnceCell::new();
macro_rules! set_handler {
    (($element:ident,$name:ident) -> $handler:expr) => {
        let handler = Closure::wrap(Box::new($handler) as Box<dyn FnMut()>);
        $element.$name(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
    };
}
static INSTANCE: once_cell::sync::OnceCell<SendWrapper<HtmlVideoElement>> = OnceCell::new();

#[wasm_bindgen]
pub fn start(src: &str) {
    let window = web_sys::window().expect("Couldn't get window");
    let document = window.document().expect("Couldn't get document");
    let middle = append_middle(&document);
    let middle = Arc::new(middle);
    let bottom = append_bottom(&document);
    let bottom = Arc::new(bottom);
    let button = query_selector(&middle, ".play");
    let first = query_selector(&bottom, ".first");
    let second = query_selector(&bottom, ".second");
    let progress_bar_loaded = query_selector(&bottom, ".progress_bar_loaded");
    let progress_bar_played = query_selector(&bottom, ".progress_bar_played");
    let progress_bar_playhead_wrapper = query_selector(&bottom, ".progress_bar_playhead_wrapper");
    let progress_bar_line = query_selector(&bottom, ".progress_bar_line");
    let video = get_video(&document).expect("Couldn't get video");
    let v = Arc::new(video.clone());

    {
        let v = v.clone();
        let handler = Closure::wrap(Box::new(move || {
            if v.paused() {
                let _ = v.play();
            } else {
                let _ = v.pause();
            }
        }) as Box<dyn FnMut()>);
        button.set_onclick(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
    }
    INSTANCE.get_or_init(|| SendWrapper::new(video.clone()));

    let onerror = Closure::wrap(Box::new(|| {
        log("onerror");
    }) as Box<dyn FnMut()>);
    video.set_onerror(Some(onerror.as_ref().unchecked_ref()));
    onerror.forget();

    set_onplay(button.clone(), v.clone());
    set_onpause(button.clone(), v.clone());
    set_ondurationchange(second.clone(), v.clone());
    set_ontimeupdate(
        first.clone(),
        progress_bar_played.clone(),
        progress_bar_playhead_wrapper.clone(),
        v.clone(),
    );
    set_onprogress(progress_bar_loaded.clone(), v.clone());
    set_progress_click(progress_bar_line, v.clone());

    on_video_click(middle.clone(), bottom.clone(), &video);

    HANLDER.get_or_init(|| {
        SendWrapper::new(Closure::wrap(Box::new(move || {
            let _ = middle.style().set_property("display", "none");
            let _ = bottom.style().set_property("display", "none");
        }) as Box<dyn FnMut()>))
    });

    video.set_src(src);
    append_track(&document, v.clone(), src);
    //video.remove_event_listener_with_callback("play", onplay.as_ref().unchecked_ref());
    //setTimeout()
}
fn set_onplay(button: Arc<HtmlElement>, video: Arc<HtmlVideoElement>) {
    let button = button.clone();
    let onplay = Closure::wrap(Box::new(move || {
        let window = web_sys::window().expect("Couldn't get window");
        window.clear_timeout();
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            HANLDER.get().unwrap().as_ref().unchecked_ref(),
            5000,
        );
        let path = button.query_selector("path").unwrap().unwrap();
        let _ = path.set_attribute("d", "M9,19H7V5H9ZM17,5H15V19h2Z");
    }) as Box<dyn FnMut()>);
    video.set_onplay(Some(onplay.as_ref().unchecked_ref()));
    onplay.forget();
}
#[wasm_bindgen]
pub fn play(src: &str) {
    match INSTANCE.get() {
        Some(v) => {
            v.set_src(src);
        }
        None => {}
    }
}
// wasm-pack build --target web

/*
https://github.com/rustwasm/wasm-bindgen

 */
// onclick
fn on_video_click(middle: Arc<HtmlElement>, bottom: Arc<HtmlElement>, element: &HtmlElement) {
    let handler = Closure::wrap(Box::new(move || {
        let _ = middle.style().set_property("display", "flex");
        let _ = bottom.style().set_property("display", "flex");
        let window = web_sys::window().expect("Couldn't get window");
        window.clear_timeout();
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            HANLDER.get().unwrap().as_ref().unchecked_ref(),
            5000,
        );
    }) as Box<dyn FnMut()>);
    element.set_onclick(handler.as_ref().dyn_ref());
    handler.forget();
}
