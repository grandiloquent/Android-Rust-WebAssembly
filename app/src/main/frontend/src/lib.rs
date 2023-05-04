mod elements;
mod listener;
mod utils;
mod video;
mod videos;

use send_wrapper::SendWrapper;
use std::{rc::Rc, sync::atomic::AtomicI32};
use utils::query_selector;
use video::{
    handler::{bind_onplay, bind_video},
    seek::{bind_fast_forward, set_progress_click},
};
use videos::{data::render, dom::initialize_bottom_sheet, search::initialize_search};

use elements::{
    append_bottom, append_middle, append_track, get_video, set_ondurationchange, set_onpause,
    set_onprogress, set_ontimeupdate,
};
use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

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

pub static HANLDER: once_cell::sync::OnceCell<SendWrapper<Closure<dyn FnMut()>>> = OnceCell::new();
pub static HANDLE: AtomicI32 = AtomicI32::new(0);

// macro_rules! set_handler {
//     (($element:ident,$name:ident) -> $handler:expr) => {
//         let handler = Closure::wrap(Box::new($handler) as Box<dyn FnMut()>);
//         $element.$name(Some(handler.as_ref().unchecked_ref()));
//         handler.forget();
//     };
// }
static INSTANCE: once_cell::sync::OnceCell<SendWrapper<HtmlVideoElement>> = OnceCell::new();

#[wasm_bindgen]
pub fn start(src: &str) {
    let window = web_sys::window().expect("Couldn't get window");
    let document = window.document().expect("Couldn't get document");
    let middle = append_middle(&document);
    let middle = Rc::new(middle);
    let bottom = append_bottom(&document);
    let bottom = Rc::new(bottom);
    let button = query_selector(&middle, ".play");
    let first = query_selector(&bottom, ".first");
    let second = query_selector(&bottom, ".second");
    let progress_bar_loaded = query_selector(&bottom, ".progress_bar_loaded");
    let progress_bar_played = query_selector(&bottom, ".progress_bar_played");
    let progress_bar_playhead_wrapper = query_selector(&bottom, ".progress_bar_playhead_wrapper");

    let video = get_video(&document).expect("Couldn't get video");
    let v = Rc::new(video.clone());

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

    bind_onplay(button.clone(), v.clone());
    set_onpause(button.clone(), v.clone());
    set_ondurationchange(second.clone(), v.clone());
    set_ontimeupdate(
        first.clone(),
        progress_bar_played.clone(),
        progress_bar_playhead_wrapper.clone(),
        v.clone(),
    );
    set_onprogress(progress_bar_loaded.clone(), v.clone());
    set_progress_click(
        Rc::new(
            bottom
                .query_selector(".progress_bar_line")
                .unwrap()
                .unwrap(),
        ),
        v.clone(),
    );

    bind_video(middle.clone(), bottom.clone(), &video);

    HANLDER.get_or_init(|| {
        SendWrapper::new(Closure::wrap(Box::new(move || {
            let _ = middle.style().set_property("display", "none");
            let _ = bottom.style().set_property("display", "none");
        }) as Box<dyn FnMut()>))
    });

    video.set_src(src);
    //append_track(&document, v.clone(), src);

    let _ = bind_fast_forward(&document, v.clone());
    //video.remove_event_listener_with_callback("play", onplay.as_ref().unchecked_ref());
    //setTimeout()
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
#[wasm_bindgen]
pub fn render_videos() {
    initialize_search();
    initialize_bottom_sheet();
    render();
}

// wasm-pack build --target web --out-dir C:\Users\psycho\Desktop\file\Plane\app\src\main\assets\pkg

/*
https://github.com/rustwasm/wasm-bindgen

 */
// onclick
