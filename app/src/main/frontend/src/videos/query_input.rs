use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Document;
use web_sys::HtmlInputElement;

pub fn query_input(document: &Document) -> Result<Rc<HtmlInputElement>, JsValue> {
    let element = match document.query_selector(".searchbox-input")? {
        Some(element) => element,
        None => {
            return Err("")?;
        }
    };
    match element.dyn_into::<HtmlInputElement>() {
        Ok(v) => Ok(Rc::new(v)),
        Err(_) => return Err("")?,
    }
}