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
        let _ = path.set_attribute(
            "d",
            "M4,20h14v1H3V6h1V20z M21,3v15H6V3H21z M17,10.5L11,7v7L17,10.5z",
        );
        let _ = svg.append_child(&path);
        let pivot_bar_item_title = document.create_element("div").expect("div");
        let _ = pivot_bar_item_title.set_attribute("class", "pivot-bar-item-title");
        let _ = bar_item_tab.append_child(&pivot_bar_item_title);
        let _ = pivot_bar_item_title.set_text_content(Some("库"));
    }
}

pub fn build_top_bar() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let header_bar = document.create_element("div").expect("div");
    let _ = header_bar.set_attribute("class", "header-bar");
    let body = document
        .body()
        .expect("document expect to have have a body");
    let _ = body.append_child(&header_bar);
    let modern_overlay = document.create_element("c3-overlay").expect("c3-overlay");
    let _ = modern_overlay.set_attribute("class", "modern-overlay");
    let _ = header_bar.append_child(&modern_overlay);
    let hidden_button = document.create_element("button").expect("button");
    let _ = hidden_button.set_attribute("class", "hidden-button");
    let _ = modern_overlay.append_child(&hidden_button);
    let topbar_header = document.create_element("header").expect("header");
    let _ = topbar_header.set_attribute("class", "topbar-header");
    let _ = header_bar.append_child(&topbar_header);
    let topbar_back_arrow = document.create_element("button").expect("button");
    let _ = topbar_back_arrow.set_attribute("class", "topbar-back-arrow");
    let _ = topbar_header.append_child(&topbar_back_arrow);
    let c3_icon = document.create_element("div").expect("div");
    let _ = c3_icon.set_attribute("class", "c3-icon");
    let _ = topbar_back_arrow.append_child(&c3_icon);
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
    let _ = path.set_attribute(
        "d",
        "M21,11v1H5.64l6.72,6.72l-0.71,0.71L3.72,11.5l7.92-7.92l0.71,0.71L5.64,11H21z",
    );
    let _ = svg.append_child(&path);
    let searchbox = document.create_element("div").expect("div");
    let _ = searchbox.set_attribute("class", "searchbox");
    let _ = topbar_header.append_child(&searchbox);
    let searchbox_form = document.create_element("form").expect("form");
    let _ = searchbox_form.set_attribute("class", "searchbox-form");
    let _ = searchbox.append_child(&searchbox_form);
    let searchbox_input_wrapper = document.create_element("div").expect("div");
    let _ = searchbox_input_wrapper.set_attribute("class", "searchbox-input-wrapper");
    let _ = searchbox_form.append_child(&searchbox_input_wrapper);
    let searchbox_input_title = document.create_element("input").expect("input");
    let _ = searchbox_input_title.set_attribute("class", "searchbox-input");
    let _ = searchbox_input_title.set_attribute("name", "search");
    let _ = searchbox_input_title.set_attribute("placeholder", "Search YouTube");
    let _ = searchbox_input_title.set_attribute("autocomplete", "off");
    let _ = searchbox_input_title.set_attribute("autocorrect", "off");
    let _ = searchbox_input_title.set_attribute("spellcheck", "false");
    let _ = searchbox_input_title.set_attribute("type", "text");
    let _ = searchbox_input_title.set_attribute("role", "combobox");
    let _ = searchbox_input_title.set_attribute("aria-haspopup", "false");
    let _ = searchbox_input_title.set_attribute("aria-autocomplete", "list");
    let _ = searchbox_input_title.set_attribute("dir", "ltr");
    let _ = searchbox_input_title.set_attribute("style", "outline: none;");
    let _ = searchbox_input_wrapper.append_child(&searchbox_input_title);
    let input = document.create_element("input").expect("input");
    let _ = input.set_attribute("type", "submit");
    let _ = input.set_attribute("hidden", "");
    let _ = searchbox_input_wrapper.append_child(&input);
    let icon_button = document.create_element("button").expect("button");
    let _ = icon_button.set_attribute("class", "icon-button");
    let _ = searchbox_input_wrapper.append_child(&icon_button);
    let c3_icon1 = document.create_element("div").expect("div");
    let _ = c3_icon1.set_attribute("class", "c3-icon");
    let _ = icon_button.append_child(&c3_icon1);
    let svg2 = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
        .expect("svg");
    let _ = svg2.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    let _ = svg2.set_attribute("enable-background", "new 0 0 24 24");
    let _ = svg2.set_attribute("height", "24");
    let _ = svg2.set_attribute("viewBox", "0 0 24 24");
    let _ = svg2.set_attribute("width", "24");
    let _ = c3_icon1.append_child(&svg2);
    let path3 = document
        .create_element_ns(Some("http://www.w3.org/2000/svg"), "path")
        .expect("path");
    let _ = path3.set_attribute("d","M20.87,20.17l-5.59-5.59C16.35,13.35,17,11.75,17,10c0-3.87-3.13-7-7-7s-7,3.13-7,7s3.13,7,7,7c1.75,0,3.35-0.65,4.58-1.71 l5.59,5.59L20.87,20.17z M10,16c-3.31,0-6-2.69-6-6s2.69-6,6-6s6,2.69,6,6S13.31,16,10,16z");
    let _ = svg2.append_child(&path3);
}
