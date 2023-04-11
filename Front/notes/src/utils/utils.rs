use super::strings::find_current_line;
use super::strings::StringExt;
use convert_case::Case;
use convert_case::Casing;
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

    while start_index > 0
        && s.chars()
            .nth(start_index - 1)
            .unwrap_or(' ')
            .is_whitespace()
    {
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
pub fn format_replace_text(textarea: &HtmlTextAreaElement) {
    let start = textarea.selection_start().unwrap().unwrap();
    let end = textarea.selection_end().unwrap().unwrap();
    if start != end {
    } else {
        let mut str = textarea.value();
        str = str.trim().to_string();
        let lines = str.lines();
        if lines.count() > 2 {
            
            let f = str.lines().nth(0).unwrap();
            let s = str.lines().nth(1).unwrap();
            let mut str = str.lines().skip(2).collect::<Vec<&str>>().join("\n");
            
            str = str.replace(f, s);
            str = str.replace(&f.to_case(Case::Snake), &s.to_case(Case::Snake));
            str = str.replace(&f.to_case(Case::Camel), &s.to_case(Case::Camel));
            str = str.replace(&f.to_case(Case::UpperCamel), &s.to_case(Case::UpperCamel));
            str = str.replace(&f.to_case(Case::Upper), &s.to_case(Case::Upper));
            str = str.replace(&f.to_case(Case::Lower), &s.to_case(Case::Lower));
            textarea.set_value(format!("{}\n{}\n{}", f, s, str).as_str());
        }
    }
}
