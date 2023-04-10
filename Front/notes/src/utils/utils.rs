use super::strings::find_current_line;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlTextAreaElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}
#[wasm_bindgen(module = "/utils.js")]
extern "C" {

    #[wasm_bindgen(catch, js_name = "onTranslateChinese")]
    pub async fn translate_chinese(q: &str) -> Result<JsValue, JsValue>;
}
#[wasm_bindgen(module = "/utils.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "readText")]
    pub async fn read_text() -> Result<JsValue, JsValue>;
}
pub fn format_comment(textarea: &HtmlTextAreaElement, expr: &str) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);
    let mut j = end_index + 1;
    while j + 1 < s.len()
        && (&s[j..j + 1])
            .as_bytes()
            .iter()
            .any(u8::is_ascii_whitespace)
    {
        j = j + 1;
    }
    let mut prefix = "";
    if j > end_index {
        prefix = &s[end_index + 1..j];
    }
    // log(format!(
    //     "{}\n{}\n{}",
    //     start_index,
    //     end_index,
    //     &s[start_index..end_index]
    // )
    // .as_str());
    let _ = textarea.set_range_text_with_start_and_end(
        format!("{}{} {}", prefix, expr, &s[start_index..end_index]).as_str(),
        start_index as u32,
        end_index as u32,
    );
}

pub fn format_translate_chinese(textarea: &HtmlTextAreaElement) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);
    let textarea = textarea.clone();
    spawn_local(async move {
        let result = translate_chinese(&s[start_index..end_index]).await;
       match result.clone(){
        Ok(v) => {
            log("1");
        },
        Err(v) => {
            log(v.as_string().unwrap().as_str());
        }
    }
        let s = result.unwrap().as_string().unwrap();
        log(s.as_str());
        let _ = textarea.set_range_text_with_start_and_end(
            format!("{}", s).as_str(),
            start_index as u32,
            end_index as u32,
        );
    });
    // Convert that promise into a future and make the test wait on it.
}

// js_sys::Reflect::get