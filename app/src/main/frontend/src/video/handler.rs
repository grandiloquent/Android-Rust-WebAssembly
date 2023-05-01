use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{HtmlElement, HtmlVideoElement};

use super::utils::schedule_hidden;

pub fn bind_video(middle: Rc<HtmlElement>, bottom: Rc<HtmlElement>, element: &HtmlElement) {
    let handler = Closure::wrap(Box::new(move || {
        let _ = middle.style().set_property("display", "flex");
        let _ = bottom.style().set_property("display", "flex");
        schedule_hidden();
    }) as Box<dyn FnMut()>);
    element.set_onclick(handler.as_ref().dyn_ref());
    handler.forget();
}
pub fn bind_onplay(button: Rc<HtmlElement>, video: Rc<HtmlVideoElement>) {
    let button = button.clone();
    let onplay = Closure::wrap(Box::new(move || {
        schedule_hidden();
        let path = button.query_selector("path").unwrap().unwrap();
        let _ = path.set_attribute("d", "M9,19H7V5H9ZM17,5H15V19h2Z");
    }) as Box<dyn FnMut()>);
    video.set_onplay(Some(onplay.as_ref().unchecked_ref()));
    onplay.forget();
}
