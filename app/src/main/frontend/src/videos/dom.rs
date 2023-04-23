use web_sys::{Document, HtmlElement};

pub fn render_item(document: &Document, parent: &HtmlElement, src: &str, title: &str, uri: &str) {
    let href = format!("/video.html?uri={}", uri);
    let media_item = document.create_element("div").expect("div");
    let _ = media_item.set_attribute("class", "media_item");
    let _ = parent.append_child(&media_item);
    let media_item_thumbnail_container = document.create_element("a").expect("a");
    let _ = media_item_thumbnail_container.set_attribute("class", "media_item_thumbnail_container");
    let _ = media_item_thumbnail_container.set_attribute("href", &href);
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
    let _ = video_thumbnail_img.set_attribute("src", src);
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
    let _ = a.set_attribute("href", &href);
    let _ = media_item_metadata.append_child(&a);
    let media_item_headline = document.create_element("h3").expect("h3");
    let _ = media_item_headline.set_attribute("class", "media_item_headline");
    let _ = a.append_child(&media_item_headline);
    let _ = media_item_headline.set_text_content(Some(title));
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

pub fn build_bottom_bar() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let bar_renderer = document.create_element("div").expect("div");
    let _ = bar_renderer.set_attribute("class", "bar-renderer");
    let body = document
        .body()
        .expect("document expect to have have a body");
    let _ = body.append_child(&bar_renderer);
    let bar_item_renderer = document.create_element("div").expect("div");
    let _ = bar_item_renderer.set_attribute("class", "bar-item-renderer");
    let _ = bar_renderer.append_child(&bar_item_renderer);
    {
        let bar_item_tab = document.create_element("div").expect("div");
        let _ = bar_item_tab.set_attribute("class", "bar-item-tab");
        let _ = bar_item_renderer.append_child(&bar_item_tab);
        let c3_icon = document.create_element("div").expect("div");
        let _ = c3_icon.set_attribute("class", "c3-icon");
        let _ = bar_item_tab.append_child(&c3_icon);
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
        let _ = path.set_attribute("d", "M4,21V10.08l8-6.96l8,6.96V21h-6v-6h-4v6H4z");
        let _ = svg.append_child(&path);
        let pivot_bar_item_title = document.create_element("div").expect("div");
        let _ = pivot_bar_item_title.set_attribute("class", "pivot-bar-item-title");
        let _ = bar_item_tab.append_child(&pivot_bar_item_title);
        let _ = pivot_bar_item_title.set_text_content(Some("主页"));
    }
    {
        let bar_item_tab = document.create_element("div").expect("div");
        let _ = bar_item_tab.set_attribute("class", "bar-item-tab");
        let _ = bar_item_renderer.append_child(&bar_item_tab);
        let c3_icon = document.create_element("div").expect("div");
        let _ = c3_icon.set_attribute("class", "c3-icon");
        let _ = bar_item_tab.append_child(&c3_icon);
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
        let _ = path.set_attribute("d", "M4,20h14v1H3V6h1V20z M21,3v15H6V3H21z M17,10.5L11,7v7L17,10.5z");
        let _ = svg.append_child(&path);
        let pivot_bar_item_title = document.create_element("div").expect("div");
        let _ = pivot_bar_item_title.set_attribute("class", "pivot-bar-item-title");
        let _ = bar_item_tab.append_child(&pivot_bar_item_title);
        let _ = pivot_bar_item_title.set_text_content(Some("库"));
    }
}
