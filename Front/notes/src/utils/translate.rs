use super::strings::find_current_line;
use serde_json::Value;
use urlencoding::encode;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlTextAreaElement;
use web_sys::{Request, RequestInit, Response};


async fn translate(q: &str, to: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();

    let mut opts = RequestInit::new();
    opts.method("GET");

    let url = format!(
        "{}//kpkpkp.cn/api/trans?q={}&to={}",
        window.location().protocol().unwrap(),
        encode(q),
        to
    );

    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.text()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}


pub fn format_translate(textarea: &HtmlTextAreaElement, is_chinese: bool) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);

    let textarea = textarea.clone();
    spawn_local(async move {
        let mut to = "zh";
        if !is_chinese {
            to = "en";
        }
        let result = translate(s.chars()
                                   .skip(start_index)
                                   .take(end_index - start_index)
                                   .collect::<String>().as_str(), to).await;

        let data = result.unwrap().as_string().unwrap();
        let v: Value = serde_json::from_str(&data).unwrap();
        let mut res = v["sentences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_object().unwrap()["trans"].as_str().unwrap())
            .collect::<Vec<&str>>()
            .join("");

        let _ = textarea.set_range_text_with_start_and_end(
            format!("{}", res).as_str(),
            start_index as u32,
            end_index as u32,
        );
    });
}


pub fn format_translate_chinese(textarea: &HtmlTextAreaElement, patterns:&Vec<Vec<String>>) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);

    let textarea = textarea.clone();
    let pattern = (*patterns).clone();
    spawn_local(async move {
        let mut to = "zh";
        let result = translate(s.chars()
                                   .skip(start_index)
                                   .take(end_index - start_index)
                                   .collect::<String>().as_str(), to).await;

        let data = result.unwrap().as_string().unwrap();
        let v: Value = serde_json::from_str(&data).unwrap();
        let mut res = v["sentences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_object().unwrap()["trans"].as_str().unwrap())
            .collect::<Vec<&str>>()
            .join("");
        for s in pattern {
            res=res.replace(&s[0],&s[1]);
        }
        let _ = textarea.set_range_text_with_start_and_end(
            format!("{}", res).as_str(),
            start_index as u32,
            end_index as u32,
        );
    });
}

