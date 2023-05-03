use std::rc::Rc;
use urlencoding::encode;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::Element;
use web_sys::KeyboardEvent;

use web_sys::Url;


use super::query_input::query_input;

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

            if e.key() == "Enter" {
                let l = web_sys::window().unwrap().location();
                let url = Url::new(l.href().unwrap().as_str()).unwrap();
                url.search_params().set("q", &encode(ev.value().as_str()));
                let _ = l.set_href(url.to_string().as_string().unwrap().as_str());
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        let _ = element
            .add_event_listener_with_callback("keydown", handler.as_ref().dyn_ref().unwrap());
        handler.forget();
    }
    Err("")?
}
