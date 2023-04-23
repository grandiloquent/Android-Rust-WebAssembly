use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

fn build_media_item(document: &Document) -> HtmlElement {
    //let media_item = build_media_item(document);
    //let _ = div.append_child(&media_item);

    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_item");
    div
}

fn build_media_item_thumbnail_container(document: &Document,uri:&str) -> HtmlElement {
    //let media_item_thumbnail_container = build_media_item_thumbnail_container(document);
    //let _ = div.append_child(&media_item_thumbnail_container);

    let div = document
        .create_element("a")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_item_thumbnail_container");
    div.set_attribute("href", format!("/video.html?uri={}",uri).as_str());
    let s = div.style();
    let _ = s.set_property("color", "currentColor");
    let _ = s.set_property("text-decoration", "none");
    let _ = s.set_property("display", "block");
    let _ = s.set_property("position", "relative");
    div
}

fn build_video_thumbnail_container_large(document: &Document) -> HtmlElement {
    //let video_thumbnail_container_large = build_video_thumbnail_container_large(document);
    //let _ = div.append_child(&video_thumbnail_container_large);

    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("video_thumbnail_container_large");
    let s = div.style();

    let _ = s.set_property("position", "relative");
    let _ = s.set_property("padding-top", "56.25%");
    div
}

fn build_video_thumbnail_bg(document: &Document) -> HtmlElement {
    //let video_thumbnail_bg = build_video_thumbnail_bg(document);
    //let _ = div.append_child(&video_thumbnail_bg);

    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("video_thumbnail_bg");
    let s = div.style();

    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("top", "0");
    let _ = s.set_property("bottom", "0");
    let _ = s.set_property("left", "0");
    let _ = s.set_property("right", "0");
    let _ = s.set_property("width", "100%");
    let _ = s.set_property("min-height", "100%");
    let _ = s.set_property("margin", "auto");
    let _ = s.set_property("background-color", "rgba(0,0,0,0.1)");
    div
}

fn build_video_thumbnail_img(document: &Document, src: &str) -> HtmlElement {
    //let video_thumbnail_img = build_video_thumbnail_img(document);
    //let _ = div.append_child(&video_thumbnail_img);

    let div = document
        .create_element("img")
        .expect("img")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("video_thumbnail_img");
    div.set_attribute("src", src);
    let s = div.style();

    let _ = s.set_property("position", "absolute");
    let _ = s.set_property("top", "0");
    let _ = s.set_property("bottom", "0");
    let _ = s.set_property("left", "0");
    let _ = s.set_property("right", "0");
    let _ = s.set_property("margin", "auto");
    let _ = s.set_property("display", "inline-block");
    let _ = s.set_property("min-height", "1px");
    let _ = s.set_property("min-width", "1px");
    let _ = s.set_property("visibility", "inherit");
    let _ = s.set_property("height", "100%");
    let _ = s.set_property("width", "100%");
    let _ = s.set_property("object-fit", "cover");
    div
}

fn build_details(document: &Document) -> HtmlElement {
    //let details = build_details(document);
    //let _ = div.append_child(&details);

    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("details");
    let s = div.style();

    let _ = s.set_property("display", "flex");
    let _ = s.set_property("-webkit-box-flex", "1");
    let _ = s.set_property("flex-grow", "1");
    let _ = s.set_property("min-width", "0");
    let _ = s.set_property("margin-bottom", "16px");
    let _ = s.set_property("margin-left", "12px");
    let _ = s.set_property("margin-top", "4px");
    div
}

fn build_media_channel(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_channel");
    let s = div.style();

    let _ = s.set_property("word-wrap", "break-word");
    let _ = s.set_property("color", "#0f0f0f");
    let _ = s.set_property("-webkit-text-size-adjust", "100%");
    let _ = s.set_property("font-size", "1.2rem");
    let _ = s.set_property("flex-shrink", "0");
    let _ = s.set_property("margin-top", "8px");
    div
}

fn build_media_item_info(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_item_info");
    let s = div.style();

    let _ = s.set_property("display", "flex");
    let _ = s.set_property("-webkit-box-flex", "1");
    let _ = s.set_property("flex-grow", "1");
    let _ = s.set_property("min-width", "0");
    let _ = s.set_property("-webkit-box-align", "start");
    let _ = s.set_property("align-items", "flex-start");
    let _ = s.set_property("margin-left", "12px");
    div
}

fn build_media_item_metadata(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_item_metadata");
    let s = div.style();

    let _ = s.set_property("display", "flex");
    let _ = s.set_property("-webkit-box-flex", "1");
    let _ = s.set_property("flex-grow", "1");
    let _ = s.set_property("-webkit-box-orient", "vertical");
    let _ = s.set_property("-webkit-box-direction", "normal");
    let _ = s.set_property("flex-direction", "column");
    let _ = s.set_property("min-width", "0");
    let _ = s.set_property("margin-top", "8px");
    div
}

fn build_a(document: &Document,uri:&str) -> HtmlElement {
    let div = document
        .create_element("a")
        .expect("a")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("a");
    div.set_attribute("href", format!("/video.html?uri={}",uri).as_str());
  
    let s = div.style();
    let _ = s.set_property("color", "currentColor");
    let _ = s.set_property("text-decoration", "none");
    div
}

fn build_badge_and_byline_renderer(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("badge_and_byline_renderer");
    let s = div.style();

    let _ = s.set_property("-webkit-box-orient", "vertical");
    let _ = s.set_property("display", "-webkit-box");
    let _ = s.set_property("-webkit-line-clamp", "2");
    let _ = s.set_property("max-height", "3em");
    let _ = s.set_property("text-overflow", "ellipsis");
    let _ = s.set_property("overflow", "hidden");
    div
}

fn build_media_item_headline(document: &Document,title:&str) -> HtmlElement {
    let div = document
        .create_element("h3")
        .expect("h3")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("media_item_headline");
    div.set_text_content(Some(title));
    let s = div.style();

    let _ = s.set_property("margin", "0 0 3px");
    let _ = s.set_property("display", "-webkit-box");
    let _ = s.set_property("-webkit-box-orient", "vertical");
    let _ = s.set_property("max-height", "2.5em");
    let _ = s.set_property("-webkit-line-clamp", "2");
    let _ = s.set_property("overflow", "hidden");
    let _ = s.set_property("line-height", "1.25");
    let _ = s.set_property("text-overflow", "ellipsis");
    let _ = s.set_property("font-size", "1.4rem");
    let _ = s.set_property("font-weight", "500");
    div
}

fn build_bottom_sheet_renderer(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("bottom_sheet_renderer");
    let s = div.style();

    let _ = s.set_property("flex-shrink", "0");
    div
}

fn build_button(document: &Document) -> HtmlElement {
    let div = document
        .create_element("button")
        .expect("button")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("button");
    let s = div.style();

    let _ = s.set_property("outline", "none");
    let _ = s.set_property("font", "inherit");
    let _ = s.set_property("position", "relative");
    let _ = s.set_property("margin", "0");
    let _ = s.set_property("white-space", "nowrap");
    let _ = s.set_property("min-width", "0");
    let _ = s.set_property("text-transform", "none");
    let _ = s.set_property("font-weight", "500");
    let _ = s.set_property("border", "none");
    let _ = s.set_property("cursor", "pointer");
    let _ = s.set_property("outline-width", "0");
    let _ = s.set_property("box-sizing", "border-box");
    let _ = s.set_property("background", "none");
    let _ = s.set_property("text-decoration", "none");
    let _ = s.set_property("-webkit-tap-highlight-color", "transparent");
    let _ = s.set_property("display", "flex");
    let _ = s.set_property("-webkit-box-orient", "horizontal");
    let _ = s.set_property("-webkit-box-direction", "normal");
    let _ = s.set_property("flex-direction", "row");
    let _ = s.set_property("-webkit-box-align", "center");
    let _ = s.set_property("align-items", "center");
    let _ = s.set_property("-webkit-box-pack", "center");
    let _ = s.set_property("justify-content", "center");
    let _ = s.set_property("-webkit-box-flex", "0");
    let _ = s.set_property("flex", "none");
    let _ = s.set_property("height", "48px");
    let _ = s.set_property("font-size", "18px");
    let _ = s.set_property("line-height", "48px");
    let _ = s.set_property("border-radius", "24px");
    let _ = s.set_property("width", "48px");
    let _ = s.set_property("padding", "0");
    let _ = s.set_property("color", "#0f0f0f");
    div
}

fn build_c3_icon(document: &Document) -> HtmlElement {
    let div = document
        .create_element("div")
        .expect("div")
        .dyn_into::<HtmlElement>()
        .unwrap();
    div.set_class_name("c3_icon");
    let s = div.style();

    let _ = s.set_property("display", "inline-block");
    let _ = s.set_property("flex-shrink", "0");
    let _ = s.set_property("width", "24px");
    let _ = s.set_property("height", "24px");
    let _ = s.set_property("fill", "currentColor");
    let _ = s.set_property("stroke", "none");

    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .expect("svg");
    let _ = svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg.set_attribute("enable-background", "new 0 0 24 24");
    let _ = svg.set_attribute("height", "24");
    let _ = svg.set_attribute("viewBox", "0 0 24 24");
    let _ = svg.set_attribute("width", "24");
    let _ = div.append_child(&svg);
    let path = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        .expect("path");
    let _ = path.set_attribute("d","M12,16.5c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S11.17,16.5,12,16.5z M10.5,12 c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S10.5,11.17,10.5,12z M10.5,6c0,0.83,0.67,1.5,1.5,1.5 s1.5-0.67,1.5-1.5S12.83,4.5,12,4.5S10.5,5.17,10.5,6z");
    let _ = svg.append_child(&path);

    div
}

pub fn render_item(document: &Document, src:&str,title:&str,uri:&str) {
    let body = document.body().unwrap();
    let media_item = build_media_item(document);
    let _ = body.append_child(&media_item);

    let media_item_thumbnail_container = build_media_item_thumbnail_container(document,uri);
    let _ = media_item.append_child(&media_item_thumbnail_container);

    let video_thumbnail_container_large = build_video_thumbnail_container_large(document);
    let _ = media_item_thumbnail_container.append_child(&video_thumbnail_container_large);

    let video_thumbnail_bg = build_video_thumbnail_bg(document);
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_bg);

    let video_thumbnail_img = build_video_thumbnail_img(document, src);
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_img);

    let details = build_details(document);
    let _ = media_item.append_child(&details);

    let media_channel = build_media_channel(document);
    let _ = details.append_child(&media_channel);

    let media_item_info = build_media_item_info(document);
    let _ = details.append_child(&media_item_info);

    let media_item_metadata = build_media_item_metadata(document);
    let _ = media_item_info.append_child(&media_item_metadata);

    let a = build_a(document,uri);
    let _ = media_item_metadata.append_child(&a);

    let media_item_headline = build_media_item_headline(document,title);
    let _ = a.append_child(&media_item_headline);

    let badge_and_byline_renderer = build_badge_and_byline_renderer(document);
    let _ = a.append_child(&badge_and_byline_renderer);

    let bottom_sheet_renderer = build_bottom_sheet_renderer(document);
    let _ = media_item_info.append_child(&bottom_sheet_renderer);

    let button = build_button(document);
    let _ = bottom_sheet_renderer.append_child(&button);

    let c3_icon = build_c3_icon(document);
    let _ = button.append_child(&c3_icon);
}
