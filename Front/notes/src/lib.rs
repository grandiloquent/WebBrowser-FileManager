#[cfg(feature = "static_ref")]
use static_ref_macro::static_ref;

use utils::dom::query_selector;
use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use web_sys::KeyboardEvent;

use crate::utils::format::format_comment;
use crate::utils::translate::format_translate;
use crate::utils::utils::format_code;
use crate::utils::utils::format_delete_current_line;
use crate::utils::utils::format_replace_text;
use crate::utils::utils::jump_link;
use crate::utils::utils::log;
use crate::utils::utils::save_data;
use crate::utils::utils::load_data;
use crate::utils::timer::Timer;

mod utils;

use futures_signals::signal::{Mutable , MutableLockRef, MutableSignalCloned};
use futures_signals::signal::Signal;


// -- timeout --

#[static_ref]
fn timeout() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
}

fn timeout_enabled() -> impl Signal<Item=bool> {
    timeout().signal_ref(Option::is_some)
}

fn start_timeout() {
    timeout().set(Some(Timer::once(2_000, stop_timeout)));
}

fn stop_timeout() {
    timeout().get();
    log("13");
}


#[wasm_bindgen]
pub fn start(path_separator: &str) {
    start_timeout();
    let window = web_sys::window().unwrap();
    //  https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.document
    let document = window.document().unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html
    let textarea = query_selector(&document, "#textarea")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .unwrap();
    load_data(&textarea);
    onclick!(("#format-comment",&document)->{
        let textarea=textarea.clone();
        move||{
            format_comment(&textarea,"//");
        }
    });
    onclick!(("#translate-chinese",&document)->{
        let textarea=textarea.clone();
        move||{
            format_translate(&textarea,true);
        }
    });
    onclick!(("#translate-english",&document)->{
        let textarea=textarea.clone();
        move||{
            format_translate(&textarea,false);
        }
    });
    onclick!(("#delete-line",&document)->{
        let textarea=textarea.clone();
        move||{
            format_delete_current_line(&textarea);
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
                    "g"=>{
                        e.prevent_default();
                      format_replace_text(&textarea);
                    }
                    "j"=>{
                        e.prevent_default();
                      jump_link(&textarea);
                    }
                    "s"=>{
                        e.prevent_default();
                        save_data(&textarea);
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
                      format_translate(&textarea,true);
                    }
                    "F2"=>{
                        e.prevent_default();
                        format_delete_current_line(&textarea);
                    }
                    "F3"=>{
                        e.prevent_default();
                        format_code(&textarea);
                    }

                    _=>{

                    }
                }
            }

        }
    });
}

// &env:RUSTFLAGS="--cfg=web_sys_unstable_apis";wasm-pack build --target web --out-dir C:\Users\Administrator\Desktop\Resources\Manager\assets\notes
