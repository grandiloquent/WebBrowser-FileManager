use utils::dom::query_selector;
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::KeyboardEvent;

use crate::utils::format::format_comment;
use crate::utils::translate::format_translate_chinese;
use crate::utils::utils::format_delete_current_line;
use crate::utils::utils::log;
mod utils;

#[wasm_bindgen]
pub fn start(path_separator: &str) {
    let window = web_sys::window().unwrap();
    //  https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.document
    let document = window.document().unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
    let textarea = query_selector(&document, "#textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();
    onclick!(("#format-comment",&document)->{
        let textarea=textarea.clone();
        move||{
            format_comment(&textarea,"//");
        }
    });
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.insert_adjacent_element
    handler!(( set_onkeydown,document)->{
        move |e:KeyboardEvent|{
            log(e.key().as_str());
            if e.ctrl_key() {
                match e.key().as_str() {
                    "d"=>{
                        e.prevent_default();
                      format_comment(&textarea,"//");
                    }
                    "w"=>{
                        e.prevent_default();
                         }
                    _=>{

                    }
                }
            }else {
                match e.key().as_str() {
                    "F1"=>{
                        e.prevent_default();
                      format_translate_chinese(&textarea);
                    }
                    "F2"=>{
                        log("------------");
                        e.prevent_default();
                        format_delete_current_line(&textarea);
                    }
                    _=>{

                    }
                }
            }

        }
    });
}

// &env:RUSTFLAGS="--cfg=web_sys_unstable_apis";wasm-pack build --target web --out-dir C:\Users\Administrator\Desktop\Resources\Manager\assets\notes
