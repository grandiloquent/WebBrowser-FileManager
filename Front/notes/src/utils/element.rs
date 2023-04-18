use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use crate::utils::utils::log;


pub fn insert_settings_menu_item() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let bar_item_renderer = document.create_element("div").expect("div");
    bar_item_renderer.set_attribute("class", "bar-item-renderer");

    // let body = document.body().expect("document expect to have have a body");
    // body.append_child(bar_item_renderer);
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").unwrap();
    svg.set_attribute("viewBox", "0 0 24 24");
    bar_item_renderer.append_child(&svg);
    let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path").unwrap();
    path.set_attribute("d", "M12 15.516q1.453 0 2.484-1.031t1.031-2.484-1.031-2.484-2.484-1.031-2.484 1.031-1.031 2.484 1.031 2.484 2.484 1.031zM19.453 12.984l2.109 1.641q0.328 0.234 0.094 0.656l-2.016 3.469q-0.188 0.328-0.609 0.188l-2.484-0.984q-0.984 0.703-1.688 0.984l-0.375 2.625q-0.094 0.422-0.469 0.422h-4.031q-0.375 0-0.469-0.422l-0.375-2.625q-0.891-0.375-1.688-0.984l-2.484 0.984q-0.422 0.141-0.609-0.188l-2.016-3.469q-0.234-0.422 0.094-0.656l2.109-1.641q-0.047-0.328-0.047-0.984t0.047-0.984l-2.109-1.641q-0.328-0.234-0.094-0.656l2.016-3.469q0.188-0.328 0.609-0.188l2.484 0.984q0.984-0.703 1.688-0.984l0.375-2.625q0.094-0.422 0.469-0.422h4.031q0.375 0 0.469 0.422l0.375 2.625q0.891 0.375 1.688 0.984l2.484-0.984q0.422-0.141 0.609 0.188l2.016 3.469q0.234 0.422-0.094 0.656l-2.109 1.641q0.047 0.328 0.047 0.984t-0.047 0.984z");
    svg.append_child(&path);
    let bar_item_title = document.create_element("div").expect("div");
    bar_item_title.set_attribute("class", "bar-item-title");
    bar_item_renderer.append_child(&bar_item_title);
    bar_item_title.set_text_content(Some("配置"));

    document.get_element_by_id("middle").unwrap().append_child(&bar_item_renderer);
    let b = bar_item_renderer.dyn_into::<HtmlElement>();
    let onclick = Closure::wrap(Box::new(move || {

        append_uri_dialog();
    }) as Box<dyn FnMut()>);

    b.unwrap()
        .set_onclick(onclick.as_ref().dyn_ref());
    onclick.forget();
}

fn append_uri_dialog() {
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let set_append_uri = document.create_element("div").expect("div");
    set_append_uri.set_attribute("id", "set-append-uri");
    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");
    let body = document.body().expect("document expect to have have a body");
    body.append_child(&set_append_uri);
    let dialog_container = document.create_element("div").expect("div");
    dialog_container.set_attribute("class", "dialog-container");
    set_append_uri.append_child(&dialog_container);
    let dialog = document.create_element("div").expect("div");
    dialog.set_attribute("class", "dialog");
    dialog_container.append_child(&dialog);
    let dialog_header = document.create_element("div").expect("div");
    dialog_header.set_attribute("class", "dialog-header");
    dialog.append_child(&dialog_header);
    let dialog_header_title = document.create_element("h2").expect("h2");
    dialog_header_title.set_attribute("class", "dialog-header-title");
    dialog_header.append_child(&dialog_header_title);
    dialog_header_title.set_text_content(Some("笔记"));
    let dialog_search = document.create_element("div").expect("div");
    dialog_search.set_attribute("class", "dialog-search");
    dialog.append_child(&dialog_search);
    let div = document.create_element("div").expect("div");
    div.set_attribute("style", "color: #5f6368;position: absolute;top: 18px;left: 20px;");
    dialog_search.append_child(&div);
    let svg = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").expect("svg");
    svg.set_attribute("style", "fill: currentColor;");
    svg.set_attribute("focusable", "false");
    svg.set_attribute("width", "24");
    svg.set_attribute("height", "24");
    svg.set_attribute("viewBox", "0 0 24 24");
    div.append_child(&svg);
    let path = document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path").expect("path");
    path.set_attribute("d", "M20.49 19l-5.73-5.73C15.53 12.2 16 10.91 16 9.5A6.5 6.5 0 1 0 9.5 16c1.41 0 2.7-.47 3.77-1.24L19 20.49 20.49 19zM5 9.5C5 7.01 7.01 5 9.5 5S14 7.01 14 9.5 11.99 14 9.5 14 5 11.99 5 9.5z");
    svg.append_child(&path);
    let div1 = document.create_element("div").expect("div");
    div1.set_attribute("style", "flex: 1; height: 100%; position: relative;");
    dialog_search.append_child(&div1);
    let dialog_search_input = document.create_element("input").expect("input");
    dialog_search_input.set_attribute("type", "text");
    dialog_search_input.set_attribute("class", "dialog-search-input");
    dialog_search_input.set_attribute("placeholder", "搜索笔记");
    div1.append_child(&dialog_search_input);
    let dialog_body = document.create_element("div").expect("div");
    dialog_body.set_attribute("class", "dialog-body");
    dialog.append_child(&dialog_body);
    let dialog_buttons = document.create_element("div").expect("div");
    dialog_buttons.set_attribute("class", "dialog-buttons");
    dialog.append_child(&dialog_buttons);
    let dialog_button = document.create_element("div").expect("div");
    dialog_button.set_attribute("class", "dialog-button");
    dialog_buttons.append_child(&dialog_button);
    dialog_button.set_text_content(Some("取消"));
    let dialog_button2 = document.create_element("div").expect("div");
    dialog_button2.set_attribute("class", "dialog-button");
    dialog_button2.set_attribute("style", "color: #909090;");
    dialog_buttons.append_child(&dialog_button2);
    dialog_button2.set_text_content(Some("继续"));
    let modern_overlay = document.create_element("div").expect("div");
    modern_overlay.set_attribute("class", "modern-overlay");
    dialog_container.append_child(&modern_overlay);
}