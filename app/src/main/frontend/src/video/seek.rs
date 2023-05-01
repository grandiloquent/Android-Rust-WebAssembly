use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, HtmlVideoElement};


use super::utils::{schedule_hidden, fast_forward};


pub fn bind_fast_forward(document: &Document, video: Rc<HtmlVideoElement>) -> Result<(), JsValue> {
    let playback_speed = document.query_selector(".playback_speed")?.unwrap();
    let handler = Closure::wrap(Box::new(move || {
        schedule_hidden();
        fast_forward(&video, 10f64);
    }) as Box<dyn FnMut()>);
    let _ = playback_speed
        .add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
    Ok(())
}
