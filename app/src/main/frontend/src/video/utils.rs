use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;

use crate::{HANDLE, HANLDER};

pub fn schedule_hidden() {
    let window = web_sys::window().expect("Couldn't get window");
    window.clear_timeout_with_handle(HANDLE.load(std::sync::atomic::Ordering::SeqCst));
    let handle = window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            HANLDER.get().unwrap().as_ref().unchecked_ref(),
            10000,
        )
        .unwrap();
    HANDLE.swap(handle, std::sync::atomic::Ordering::AcqRel);
}
pub fn fast_forward(video: &HtmlVideoElement, value: f64) {
    video.set_current_time(video.current_time() + value);
}
