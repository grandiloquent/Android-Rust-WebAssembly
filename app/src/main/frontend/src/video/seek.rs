use std::rc::Rc;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Document, Element, HtmlVideoElement, MouseEvent};

use super::utils::{fast_forward, schedule_hidden};

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
pub fn set_progress_click(element: Rc<Element>, video: Rc<HtmlVideoElement>) {
    let width = element.get_bounding_client_rect().width();
    let handler = Closure::wrap(Box::new(move |event: MouseEvent| {
        schedule_hidden();
        let offset_x = event.offset_x() as f64;
        video.set_current_time(offset_x / width * video.duration());
    }) as Box<dyn FnMut(_)>);
    let _ = element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap());
    handler.forget();
    
}
