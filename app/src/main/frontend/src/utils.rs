use std::{rc::Rc};

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlVideoElement};

use crate::log;

pub fn adjust_size(video: &HtmlVideoElement) {
    let vh = video.video_height();
    let vw = video.video_width();

    let window: web_sys::Window = web_sys::window().unwrap();
    let w = {
        let w1 = window.inner_width().unwrap().as_f64().unwrap();
        let w2 = window.outer_width().unwrap().as_f64().unwrap();
        if w1 < w2 {
            w1
        } else {
            w2
        }
    };
    let h = {
        let h1=window.inner_height().unwrap().as_f64().unwrap();
        let h2=window.outer_height().unwrap().as_f64().unwrap();
        if h1<h2{
            h1
        }else {
            h2
        }
    };
    log(format!("video: {}x{}\nwindow: {}x{}", vw, vh, w, h).as_str());

    let s = video.style();

    let a1 = w / (vw as f64);
    let a2 = h / (vh as f64);
    let aspect_ratio = if a1 < a2 { a1 } else { a2 };

    let height = (vh as f64) * aspect_ratio;
    let width = (vw as f64) * aspect_ratio;

    let _ = s.set_property("width", format!("{}px", w).as_str());
    let _ = s.set_property("position", "fixed");
    let _ = s.set_property("height", format!("{}px", height).as_str());
    let _ = s.set_property("width", format!("{}px", width).as_str());
    let _ = s.set_property("top", format!("{}px", (h - height) / 2.0).as_str());
    let _ = s.set_property("left", format!("{}px", (w - width) / 2.0).as_str());
}

pub fn query_selector(parent: &HtmlElement, selector: &str) -> Rc<HtmlElement> {
    let progress_bar_loaded = parent
        .query_selector(selector)
        .unwrap()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    Rc::new(progress_bar_loaded)
}
pub trait StringExt {
    /// Returns the string before the search string.
    fn substring_before(&self, search: &str) -> String;
    /// Returns the string after the search string.
    fn substring_after(&self, search: &str) -> String;

    /// Returns the string before the last match of the search string.
    fn substring_before_last(&self, search: &str) -> String;

    /// Returns the string after the last match of the search string.
    fn substring_after_last(&self, search: &str) -> String;

    /// Returns the string between the start and end bookend strings.
    fn substring_between(&self, start: &str, end: &str) -> String;
}

impl StringExt for String {
    fn substring_before(&self, search: &str) -> String {
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string(),
        };
        answer
    }

    fn substring_before_last(&self, search: &str) -> String {
        let i_pos = self.rfind(search);
        let answer = match i_pos {
            None => String::from(self),
            Some(val) => self[..val].to_string(),
        };
        answer
    }

    fn substring_after(&self, search: &str) -> String {
        let i_pos = self.find(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string(),
        };
        answer
    }

    fn substring_after_last(&self, search: &str) -> String {
        let i_pos = self.rfind(search);
        let answer = match i_pos {
            None => String::new(),
            Some(val) => self[(val + search.len())..].to_string(),
        };
        answer
    }

    fn substring_between(&self, start: &str, end: &str) -> String {
        let i_start_pos = self.find(start);
        let answer = match i_start_pos {
            None => String::new(),
            Some(val) => {
                let rest = self[(val + start.len())..].to_string();
                let i_end_pos = rest.find(end);
                match i_end_pos {
                    None => String::new(),
                    Some(val2) => rest[0..val2].to_string(),
                }
            }
        };
        answer
    }
}
