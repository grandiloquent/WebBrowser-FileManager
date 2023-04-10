use super::strings::find_current_line;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::HtmlTextAreaElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

}
#[wasm_bindgen(module = "/utils.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "readText")]
    pub async fn read_text() -> Result<JsValue, JsValue>;
}

pub fn format_delete_current_line(textarea: &HtmlTextAreaElement) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);
    let _ = textarea.set_range_text_with_start_and_end("", start_index as u32, end_index as u32);
}
