use wasm_bindgen::{prelude::Closure, JsValue, JsCast};
use web_sys::Element;

pub fn hidden_element(element: &Element) -> Result<(), JsValue> {
    element.set_attribute("style", "display:none")
}
pub fn get_data_attribute_value(element: &Element) -> Result<(), JsValue> {
    Err("")?
}

pub fn bind_click_event(element: &Element, handler: Closure<dyn FnMut()>) -> Result<(), JsValue> {
    element.add_event_listener_with_callback("click", handler.as_ref().dyn_ref().unwrap())?;
    handler.forget();
    Ok(())
}
