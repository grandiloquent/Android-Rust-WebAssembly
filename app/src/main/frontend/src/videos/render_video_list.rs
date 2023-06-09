use serde_json::Value;
use urlencoding::encode;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

use crate::utils::{create_wrapper_element, format_duration, query_element};

use super::data::HANLDER;

pub fn render_video_list(item: &Vec<Value>) -> Result<(), JsValue> {
    let parent = query_element(".media-items")?;

    let s=item.iter().map(|x|{
        let duration=match x["duration"].as_u64() {
            Some(v)=>format!(r#"<div class="video-thumbnail-overlay">{}</div>"#,format_duration(v)),
            None=>String::new(),
        };
        format!(r#"<div class="media_item" data-id="{0}"><a class="media_item_thumbnail_container"
        href="/video.html?url={3}" target="_blank">
        <div class="video_thumbnail_container_large">
            <div class="video_thumbnail_bg"></div><img class="video_thumbnail_img"
                src="{1}">
        </div>
        {4}
    </a>
    <div class="details">
        <div class="media_channel"></div>
        <div class="media_item_info">
            <div class="media_item_metadata"><a class="a" href="/video.html?url={3}" target="_blank">
                    <h3 class="media_item_headline">{2}</h3>
                    <div class="badge_and_byline_renderer"></div>
                </a></div>
            <div class="bottom_sheet_renderer"><button class="button" data-id="{0}">
                    <div class="c3_icon"><svg xmlns="http://www.w3.org/2000/svg"
                            enable-background="new 0 0 24 24" height="24" viewBox="0 0 24 24" width="24">
                            <path
                                d="M12,16.5c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S11.17,16.5,12,16.5z M10.5,12 c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S10.5,11.17,10.5,12z M10.5,6c0,0.83,0.67,1.5,1.5,1.5 s1.5-0.67,1.5-1.5S12.83,4.5,12,4.5S10.5,5.17,10.5,6z">
                            </path>
                        </svg></div>
                </button></div>
        </div>
    </div>
</div>"#,
        x["id"].as_i64().unwrap(),
        x["image"].as_str().unwrap(),
        x["title"].as_str().unwrap(),
        encode(x["uri"].as_str().unwrap()),duration
    )
    }).collect::<Vec<String>>()
    .join("");

    let wrapper = create_wrapper_element()?;
    wrapper.set_inner_html(s.as_str());

    let _ = parent.append_child(&wrapper);
    let _ = bind_video_menu(&wrapper);
    Ok(())
}

fn bind_video_menu(wrapper: &Element) -> Result<(), JsValue> {
    let nodes = wrapper.query_selector_all(".button")?;
    let mut count = 0;
    loop {
        let n = match nodes.get(count) {
            Some(v) => v,
            None => {
                return Err(JsValue::from_str("count"));
            }
        };
        let _ = n.add_event_listener_with_callback(
            "click",
            HANLDER.get().unwrap().as_ref().unchecked_ref(),
        );
        count = count + 1;
        if count >= nodes.length() {
            break;
        }
    }
    Err("")?
}
