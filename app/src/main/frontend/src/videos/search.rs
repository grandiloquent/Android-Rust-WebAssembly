use std::sync::Arc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::HtmlElement;

pub fn initialize_search() {
    // querySelector
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let share = document
        .query_selector(".topbar-header")
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    let share = Arc::new(share);
    let _ = initialize_search_button(&document, share.clone());
    let _ = initialize_search_back(&document, share.clone());
}

fn initialize_search_button(document: &Document, share: Arc<HtmlElement>) -> Result<(), JsValue> {
    // initialize_search_button(&document, share.clone());
    let element = match document.query_selector(".topbar-menu-button")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };
    let element = match element.dyn_into::<HtmlElement>() {
        Ok(element) => element,
        Err(err) => {
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
        element.set_onclick(handler.as_ref().dyn_ref());
        handler.forget();
    }
    Err("")?
}

// clickfn
fn initialize_search_back(document: &Document, share: Arc<HtmlElement>) -> Result<(), JsValue> {
    //  let _ = initialize_search_back(&document, share.clone());
    let element = match document.query_selector(".topbar-back-arrow")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };
    let element = match element.dyn_into::<HtmlElement>() {
        Ok(element) => element,
        Err(err) => {
            return Err("")?;
        }
    };
    {
        let share = share.clone();
        let handler = Closure::wrap(
            Box::new(move || share.set_class_name("topbar-header")) as Box<dyn FnMut()>
        );
        element.set_onclick(handler.as_ref().dyn_ref());
        handler.forget();
    }
    Err("")?
}
