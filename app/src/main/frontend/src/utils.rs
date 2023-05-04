use std::rc::Rc;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, HtmlElement, HtmlVideoElement, Request, RequestInit, Response, Url};

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
        let h1 = window.inner_height().unwrap().as_f64().unwrap();
        let h2 = window.outer_height().unwrap().as_f64().unwrap();
        if h1 < h2 {
            h1
        } else {
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

pub fn get_base_uri() -> String {
    if web_sys::window().unwrap().location().host().unwrap() == "127.0.0.1:5500" {
        "http://192.168.0.109:3000".to_string()
    } else {
        String::new()
    }
}
pub fn query_element(selectors: &str) -> Result<Element, JsValue> {
    let window = match web_sys::window() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("window"));
        }
    };
    let document = match window.document() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("document"));
        }
    };
    let element = document.query_selector(selectors)?;
    match element {
        Some(v) => Ok(v),
        None => {
            return Err(JsValue::from_str("element"));
        }
    }
}
pub fn create_wrapper_element() -> Result<Element, JsValue> {
    let window = match web_sys::window() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("window"));
        }
    };
    let document = match window.document() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("document"));
        }
    };
    document.create_element("div")
}

pub fn hidden_element(element: &Element) -> Result<(), JsValue> {
    element.set_attribute("style", "display:none")
}
pub async fn get_string(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}
pub fn get_base_url() -> Result<Url, JsValue> {
    let window = match web_sys::window() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("window"));
        }
    };
    let location = window.location();
    let host = match location.host() {
        Ok(v) => v,
        Err(_) => {
            return Err(JsValue::from_str("host"));
        }
    };
    if host == "127.0.0.1:5500" {
        Url::new("http://192.168.0.109:3000")
    } else {
        Url::new(format!("{}{}", location.protocol().unwrap(), host).as_str())
    }
}
pub fn get_search_params(name: &str) -> Result<String, JsValue> {
    let window = match web_sys::window() {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("window"));
        }
    };
    let href = window.location().href()?;
    let url = Url::new(href.as_str())?;
    let value = match url.search_params().get(name) {
        Some(v) => v,
        None => {
            return Err(JsValue::from_str("name"));
        }
    };
    Ok(value)
}
pub fn format_duration(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;
    if hours > 0 {
        format!("{}:{:0>2}:{:0>2}", hours, minutes, seconds)
    } else {
        format!("{}:{:0>2}", minutes, seconds)
    }
}
