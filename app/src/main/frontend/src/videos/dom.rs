use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement};

pub fn render_item(document: &Document, src: &str, title: &str, uri: &str) {
    // let body = document.body().unwrap();
    // let media_item = build_media_item(document);
    // let _ = body.append_child(&media_item);

    // let media_item_thumbnail_container = build_media_item_thumbnail_container(document,uri);
    // let _ = media_item.append_child(&media_item_thumbnail_container);

    // let video_thumbnail_container_large = build_video_thumbnail_container_large(document);
    // let _ = media_item_thumbnail_container.append_child(&video_thumbnail_container_large);

    // let video_thumbnail_bg = build_video_thumbnail_bg(document);
    // let _ = video_thumbnail_container_large.append_child(&video_thumbnail_bg);

    // let video_thumbnail_img = build_video_thumbnail_img(document, src);
    // let _ = video_thumbnail_container_large.append_child(&video_thumbnail_img);

    // let details = build_details(document);
    // let _ = media_item.append_child(&details);

    // let media_channel = build_media_channel(document);
    // let _ = details.append_child(&media_channel);

    // let media_item_info = build_media_item_info(document);
    // let _ = details.append_child(&media_item_info);

    // let media_item_metadata = build_media_item_metadata(document);
    // let _ = media_item_info.append_child(&media_item_metadata);

    // let a = build_a(document,uri);
    // let _ = media_item_metadata.append_child(&a);

    // let media_item_headline = build_media_item_headline(document,title);
    // let _ = a.append_child(&media_item_headline);

    // let badge_and_byline_renderer = build_badge_and_byline_renderer(document);
    // let _ = a.append_child(&badge_and_byline_renderer);

    // let bottom_sheet_renderer = build_bottom_sheet_renderer(document);
    // let _ = media_item_info.append_child(&bottom_sheet_renderer);

    // let button = build_button(document);
    // let _ = bottom_sheet_renderer.append_child(&button);

    // let c3_icon = build_c3_icon(document);
    // let _ = button.append_child(&c3_icon);
    let href=format!("/video.html?uri={}", uri);
    let media_item = document.create_element("div").expect("div");
    let _ = media_item.set_attribute("class", "media_item");
    let body = document
        .body()
        .expect("document expect to have have a body");
    let _ = body.append_child(&media_item);
    let media_item_thumbnail_container = document.create_element("a").expect("a");
    let _ = media_item_thumbnail_container.set_attribute("class", "media_item_thumbnail_container");
    let _ = media_item_thumbnail_container.set_attribute("href",&href);
    let _ = media_item.append_child(&media_item_thumbnail_container);
    let video_thumbnail_container_large = document.create_element("div").expect("div");
    let _ =
        video_thumbnail_container_large.set_attribute("class", "video_thumbnail_container_large");
    let _ = media_item_thumbnail_container.append_child(&video_thumbnail_container_large);
    let video_thumbnail_bg = document.create_element("div").expect("div");
    let _ = video_thumbnail_bg.set_attribute("class", "video_thumbnail_bg");
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_bg);
    let video_thumbnail_img = document.create_element("img").expect("img");
    let _ = video_thumbnail_img.set_attribute("class", "video_thumbnail_img");
    let _ = video_thumbnail_img.set_attribute("src",src);
    let _ = video_thumbnail_container_large.append_child(&video_thumbnail_img);
    let details = document.create_element("div").expect("div");
    let _ = details.set_attribute("class", "details");
    let _ = media_item.append_child(&details);
    let media_channel = document.create_element("div").expect("div");
    let _ = media_channel.set_attribute("class", "media_channel");
    let _ = details.append_child(&media_channel);
    let media_item_info = document.create_element("div").expect("div");
    let _ = media_item_info.set_attribute("class", "media_item_info");
    let _ = details.append_child(&media_item_info);
    let media_item_metadata = document.create_element("div").expect("div");
    let _ = media_item_metadata.set_attribute("class", "media_item_metadata");
    let _ = media_item_info.append_child(&media_item_metadata);
    let a = document.create_element("a").expect("a");
    let _ = a.set_attribute("class", "a");
    let _ = a.set_attribute("href",&href);
    let _ = media_item_metadata.append_child(&a);
    let media_item_headline = document.create_element("h3").expect("h3");
    let _ = media_item_headline.set_attribute("class", "media_item_headline");
    let _ = a.append_child(&media_item_headline);
    let _ = media_item_headline.set_text_content(Some(title
    ));
    let badge_and_byline_renderer = document.create_element("div").expect("div");
    let _ = badge_and_byline_renderer.set_attribute("class", "badge_and_byline_renderer");
    let _ = a.append_child(&badge_and_byline_renderer);
    let bottom_sheet_renderer = document.create_element("div").expect("div");
    let _ = bottom_sheet_renderer.set_attribute("class", "bottom_sheet_renderer");
    let _ = media_item_info.append_child(&bottom_sheet_renderer);
    let button = document.create_element("button").expect("button");
    let _ = button.set_attribute("class", "button");
    let _ = bottom_sheet_renderer.append_child(&button);
    let c3_icon = document.create_element("div").expect("div");
    let _ = c3_icon.set_attribute("class", "c3_icon");
    let _ = button.append_child(&c3_icon);
    let svg = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .expect("svg");
    let _ = svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg.set_attribute("enable-background", "new 0 0 24 24");
    let _ = svg.set_attribute("height", "24");
    let _ = svg.set_attribute("viewBox", "0 0 24 24");
    let _ = svg.set_attribute("width", "24");
    let _ = c3_icon.append_child(&svg);
    let path = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        .expect("path");
    let _ = path.set_attribute("d","M12,16.5c0.83,0,1.5,0.67,1.5,1.5s-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5S11.17,16.5,12,16.5z M10.5,12 c0,0.83,0.67,1.5,1.5,1.5s1.5-0.67,1.5-1.5s-0.67-1.5-1.5-1.5S10.5,11.17,10.5,12z M10.5,6c0,0.83,0.67,1.5,1.5,1.5 s1.5-0.67,1.5-1.5S12.83,4.5,12,4.5S10.5,5.17,10.5,6z");
    let _ = svg.append_child(&path);
}
