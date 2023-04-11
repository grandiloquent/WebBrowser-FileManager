use super::strings::find_current_line;
use regex::Regex;
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
    let (mut start_index, mut end_index) = find_current_line(s.as_str(), start as usize);

    while start_index > 0 &&  s.chars()
    .nth(start_index - 1)
    .unwrap_or(' ').is_whitespace() {
        start_index = start_index - 1;
    }
    let x = s.chars().count();
    while end_index + 1 < x && s.chars().nth(end_index).unwrap_or(' ').is_whitespace() {
        end_index = end_index + 1;
    }
    let _ =
        textarea.set_range_text_with_start_and_end("\n\n", start_index as u32, end_index as u32);
}
pub fn format_code(textarea: &HtmlTextAreaElement) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let re = Regex::new(r#"([a-zA-Z0-9 "\t_<>;:.+%'#*=()!?|^&\[\]{},`’-])"#).unwrap();
    let mut start_index = start as usize;
    while start_index > 1
        && re.is_match(
            s.chars()
                .nth(start_index - 1)
                .unwrap_or(' ')
                .to_string()
                .as_str(),
        )
    {
        start_index = start_index - 1;
    }
    let mut end_index = start as usize;

    let x = s.chars().count();
    while end_index + 1 < x && re.is_match(s.chars().nth(end_index).unwrap().to_string().as_str()) {
        end_index = end_index + 1;
    }
    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            " `{}` ",
            s.chars()
                .skip(start_index)
                .take(end_index - start_index)
                .collect::<String>()
        )
        .as_str(),
        start_index as u32,
        end_index as u32,
    );
}
