use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::Element;
use web_sys::KeyboardEvent;

use wasm_bindgen_futures::spawn_local;

use crate::log;

use super::fetch_match_videos::fetch_match_videos;
use super::query_input::query_input;
use super::render_video_list::render_video_list;

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
                    let obj = fetch_match_videos(ev.value().as_str(), 0, 20)
                        .await
                        .unwrap();
                    let obj = obj.as_array().unwrap();
                    let _ = render_video_list(obj);
                    ()
                });
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        let _ = element
            .add_event_listener_with_callback("keydown", handler.as_ref().dyn_ref().unwrap());
        handler.forget();
    }
    Err("")?
}
