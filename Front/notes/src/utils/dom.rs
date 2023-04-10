use url::Url;
use wasm_bindgen::JsValue;
use web_sys::{Document, Element, Window};

pub fn query_selector(document: &Document, selectors: &str) -> Result<Element, JsValue> {
    let v = document.query_selector(selectors)?;
    match v {
        Some(e) => Ok(e),
        None => Err(JsValue::from_str("Cant find element")),
    }
}
pub fn get_query_string(window: &Window, s: &str) -> String {
    let location = window.location();
    let url = Url::parse(location.href().unwrap().as_str()).unwrap();
    // https://docs.rs/url/2.3.1/url/struct.Url.html#method.query_pairs
    url.query_pairs()
        .find(|(k, _)| k == s)
        .unwrap()
        .1
        .to_string()
}

pub fn set_text_content(document: &Document, selectors: &str, value: &str) -> Result<(), JsValue> {
    let v = document.query_selector(selectors)?;
    match v {
        Some(e) => {
            e.set_text_content(Some(value));
            Ok(())
        }
        None => Err(JsValue::from_str("Cant find element")),
    }
}
