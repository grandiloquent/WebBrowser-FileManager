use urlencoding::encode;
use utils::date::seconds_to_duration;
use utils::dom::get_query_string;
use utils::dom::query_selector;
use utils::dom::set_text_content;
use utils::strings::StringExt;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlTextAreaElement;
use web_sys::KeyboardEvent;

use crate::utils::utils::format_comment;
mod utils;

#[wasm_bindgen]
pub fn start(path_separator: &str) {
    let window = web_sys::window().unwrap();
    //  https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.document
    let document = window.document().unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
    let textarea = query_selector(&document, "textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.insert_adjacent_element
    handler!(( set_onkeydown,document)->{
        move |e:KeyboardEvent|{
            if e.ctrl_key() {
                match e.key().as_str() {
                    "d"=>{
                        e.prevent_default();
                        format_comment(&textarea,"//");
                    }
                    _=>{

                    }
                }
            }else {
                match e.key().as_str() {
                    "F1"=>{
                        e.prevent_default();
                    }
                    _=>{

                    }
                }
            }

        }
    });
}
