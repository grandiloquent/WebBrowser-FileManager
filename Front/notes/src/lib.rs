use std::io::Read;
use std::sync::Arc;
#[cfg(feature = "static_ref")]
use static_ref_macro::static_ref;

use utils::dom::query_selector;
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::KeyboardEvent;

use crate::utils::format::format_comment;
use crate::utils::translate::*;
use crate::utils::utils::*;

mod utils;

use crate::utils::element::insert_settings_menu_item;


#[wasm_bindgen]
pub fn start(path_separator: &str) {
    //start_timeout();
    let window = web_sys::window().unwrap();
    //  https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.document
    let document = window.document().unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
    let textarea = query_selector(&document, "#textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();
    let toast = query_selector(&document, "#toast").unwrap();
    load_data(&textarea);
    let mut patterns: Vec<Vec<String>> = vec![];
    if let Some(v) = window.local_storage()
        .unwrap()
        .unwrap()
        .get_item("pattern")
        .unwrap() {
        patterns = v.split("\n")
            .filter(|f| !f.trim().is_empty())
            .map(|f| {
                let array = f.split("|")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                array
            })
            .collect::<Vec<_>>();
        ;
    }
    let t = Arc::new(textarea);
    onclick!(("#format-comment",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_comment(&textarea,"//");
        }
    });
    onclick!(("#delete-block",&document)->{
        let textarea=Arc::clone(&t);
        move||{
           delete_code_block(&textarea);
        }
    });
    onclick!(("#translate-chinese",&document)->{
        let textarea=Arc::clone(&t);
        let patterns=patterns.clone();
        move||{
            format_translate_chinese(&textarea,&patterns);
        }
    });
    onclick!(("#translate-english",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_translate(&textarea,false);
        }
    });
    onclick!(("#format-head",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_head(&textarea);
        }
    });
    onclick!(("#format-indent-increase",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_indent_increase(&textarea);
        }
    });
    onclick!(("#format-code-block",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_code_block(&textarea);
        }
    });
    onclick!(("#format-list",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_list(&textarea);
        }
    });
    onclick!(("#preview",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            render_markdown(&textarea);
        }
    });

    onclick!(("#format-save",&document)->{
        let textarea=Arc::clone(&t);
        let toast=toast.clone();
        move||{
            save_data(&textarea,&toast);
        }
    });


    onclick!(("#format-code",&document)->{
        let textarea=Arc::clone(&t);
        let toast=toast.clone();
        move||{
            format_code(&textarea,"`");
        }
    });
    onclick!(("#format-bold",&document)->{
        let textarea=Arc::clone(&t);
        let toast=toast.clone();
        move||{
            format_code(&textarea,"**");
        }
    });
    onclick!(("#delete-line",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            format_delete_current_line(&textarea);
        }
    });
    onclick!(("#format-paste-end",&document)->{
        let textarea=Arc::clone(&t);
        move||{
            eval(&textarea);
        }
    });


    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html#method.insert_adjacent_element
    handler!(( set_onkeydown,document)->{

        move |e:KeyboardEvent|{

            if e.ctrl_key() {
                match e.key().as_str() {
                    "d"=>{
                        e.prevent_default();
                      format_comment(&Arc::clone(&t),"//");
                    }
                    "g"=>{
                        e.prevent_default();
                      format_replace_text(&Arc::clone(&t));
                    }
                    "h"=>{
                        e.prevent_default();
                        format_head(&Arc::clone(&t));
                    }
                    "j"=>{
                        e.prevent_default();
                      jump_link(&Arc::clone(&t));
                    }
                    "s"=>{

                        e.prevent_default();
                        save_data(&Arc::clone(&t),&toast);
                         }
                    "e"=>{
                        e.prevent_default();
                        format_indent_increase(&Arc::clone(&t));
                         }
                    "p"=>{
                        e.prevent_default();
                        render_markdown(&Arc::clone(&t));
                    }
                    _=>{

                    }
                }
            }else {
                match e.key().as_str() {

                    "F1"=>{
                        e.prevent_default();

                        // format_translate_chinese
                        // format_translate_chinese
                      format_translate_chinese(&Arc::clone(&t),&patterns);
                    }
                    "F2"=>{
                        e.prevent_default();
                        format_delete_current_line(&Arc::clone(&t));
                    }
                    "F3"=>{
                        e.prevent_default();
                        format_code(&Arc::clone(&t),"`");
                    }

                    _=>{

                    }
                }
            }

        }
    });
}

// &env:RUSTFLAGS="--cfg=web_sys_unstable_apis";wasm-pack build --target web --out-dir C:\Users\Administrator\Desktop\Resources\Manager\assets\notes
// &env:RUSTFLAGS="--cfg=web_sys_unstable_apis";wasm-pack build --target web --out-dir C:\Users\Administrator\Desktop\Killer\app\src\main\assets\notes