use std::collections::HashMap;

use super::strings::find_current_line;
use super::strings::StringExt;
use convert_case::Case;
use convert_case::Casing;
use regex::Regex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::HtmlTextAreaElement;
use web_sys::{Request, RequestInit, Response};
use web_sys::Element;
use serde::Serialize;
use serde_json::Value;

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

fn collect_data(textarea: &HtmlTextAreaElement) -> Result<String, serde_json::Error> {
    let mut s = textarea.value();
    s = s.trim().to_string();
    let title = s.substring_before("\n");
    let content = s.substring_after("\n");
    let mut m: HashMap<&str, String> = HashMap::new();
    m.insert("title", title.trim().to_string());
    m.insert("content", content.trim().to_string());
    serde_json::to_string(&m)
}

#[derive(Serialize)]
struct Article {
    id: i32,
    title: String,
    content: String,
    thumbnail: String,
    tags: Vec<String>,
    create_at: u64,
    update_at: u64,
}

fn collect_article(textarea: &HtmlTextAreaElement) -> Result<String, serde_json::Error> {
    let mut s = textarea.value();
    s = s.trim().to_string();
    let mut title = s.substring_before("\n");
    let content = s.substring_after("\n");
    let regex = Regex::new(r"!\[[^]]*?]\([^)]+\)").unwrap();
    let mut thumbnail = String::new();
    match regex.find(content.as_str()) {
        Some(v) => {
            thumbnail = v.as_str().to_string()
                .substring_after("(")
                .substring_before(")");
        }
        None => {}
    }
    let mut tags: Vec<String> = vec![];
    if title.contains("|") {
        tags = title.substring_after("[")
            .substring_before("]")
            .split(",")
            .map(|x| x.to_string().substring_after("\"")
                .substring_before("\""))
            .collect();
        title = title.substring_before("|").trim().to_string();
    }
    let window = web_sys::window().unwrap();
    let search = window.location().search().unwrap();
    let mut id = 0;
    if Regex::new(r"\?article=[0-9]+").unwrap().is_match(&search) {
        id = search.substring_after("=").parse::<i32>().unwrap();
    };
    let a = Article {
        id: id,
        title: title,
        content: content,
        thumbnail: thumbnail,
        tags: tags,
        create_at: (js_sys::Date::new_0().get_time() / 1000.0) as u64,
        update_at: (js_sys::Date::new_0().get_time() / 1000.0) as u64,
    };
    serde_json::to_string(&a)
}

pub fn format_code(textarea: &HtmlTextAreaElement, around: &str) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let re = Regex::new(r#"([$\\/a-zA-Z0-9 "\t_<>;:.+%'#*=()!?|^&\[\]{},`’-])"#).unwrap();
    let mut start_index = start as usize;
    while start_index > 1
        && re.is_match(
        s.chars()
            .nth(start_index - 1)
            .unwrap_or(' ')
            .to_string().as_str(),
    )
    {
        start_index = start_index - 1;
    }
    let mut end_index = start as usize;
    let x = s.chars().count();
    while end_index + 1 < x && re.is_match(s.chars().nth(end_index).unwrap().to_string().as_str()) {
        end_index = end_index + 1;
    }
    while start_index < x
        &&
        s.chars()
            .nth(start_index)
            .unwrap_or(' ') == ' '
    {
        start_index = start_index + 1;
    }
    while end_index > 0
        &&
        s.chars()
            .nth(end_index - 1)
            .unwrap_or(' ') == ' '
    {
        end_index = end_index - 1;
    }
    let str = s.chars()
        .skip(start_index)
        .take(end_index - start_index)
        .collect::<String>();
    //log(format!("<{}-{}-{}>", start_index, end_index, str).as_str());
    let mut space = " ";
    if str.starts_with("(") && str.ends_with(")") {
        start_index = start_index + 1;
        end_index = end_index - 1;
        space = "";
    }
    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            "{}{}{}{}{}",
            space,
            around, s.chars()
                .skip(start_index)
                .take(end_index - start_index)
                .collect::<String>(),
            around,
            space
        )
            .as_str(),
        start_index as u32,
        end_index as u32,
    );
}

pub fn format_code_block(textarea: &HtmlTextAreaElement) {
    let (start_index, end_index) = find_block(textarea);
    let s = textarea.value();
    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            "```rust\n{}\n```",
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

pub fn format_list(textarea: &HtmlTextAreaElement) {
    let (start_index, end_index) = find_block(textarea);
    let s = textarea.value();
    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            "{}",
            s.chars()
                .skip(start_index)
                .take(end_index - start_index)
                .collect::<String>()
                .split("\n")
                .map(|s| format!("- {}", s))
                .collect::<Vec<String>>()
                .join("\n")
        )
            .as_str(),
        start_index as u32,
        end_index as u32,
    );
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

pub fn format_head(textarea: &HtmlTextAreaElement) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (mut start_index, mut end_index) = find_current_line(s.as_str(), start as usize);
    let mut s = s.chars()
        .skip(start_index)
        .take(end_index - start_index)
        .collect::<String>();
// https://doc.rust-lang.org/std/string/struct.String.html
    if s.starts_with("#") {
        s = format!("#{}", s);
    } else {
        s = format!("### {}", s);
    }
    let _ = textarea.set_range_text_with_start_and_end(s.as_str(),
                                                       start_index as u32,
                                                       end_index as u32,
    );
}

fn find_block(textarea: &HtmlTextAreaElement) -> (usize, usize) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (mut start_index, mut end_index) = find_current_line(s.as_str(), start as usize);
    let x = s.chars().count();
    if start_index > 1 {
        let mut next_start_index = start_index - 1;
        //log(format!("start_index = {}\nnext_start_index = {}\nx = {}", start_index, next_start_index, x).as_str());
        loop {
            while next_start_index > 0
                && s.chars()
                .nth(next_start_index - 1)
                .unwrap_or(' ') != '\n'
            {
                // log(format!("start_index = {}\nnext_start_index = {}\ns = {}-{}", start_index, next_start_index, s.chars()
                //     .nth(next_start_index)
                //     .unwrap_or(' ') as u32,s.chars()
                //     .nth(next_start_index)
                //     .unwrap_or(' ')).as_str());
                next_start_index = next_start_index - 1;
            }

            let str = s.chars()
                .skip(next_start_index)
                .take(start_index - next_start_index)
                .collect::<String>();
            //log(format!("start_index = {}\nnext_start_index = {}\ns = {}", start_index, next_start_index, str).as_str());

            if str.trim().is_empty() {
                break;
            }
            start_index = next_start_index;
            if next_start_index > 0 && s.chars()
                .nth(next_start_index - 1)
                .unwrap_or(' ') == '\n' {
                next_start_index = next_start_index - 1;
            }
        }
    }

    if end_index != x {
        let mut next_end_index = end_index + 1;
        // log(format!("next_end_index = {}\nend_index = {}\ns = {}", next_end_index, end_index, s.chars()
        //     .nth(next_end_index + 1)
        //     .unwrap_or(' ')
        //     as u32).as_str());
        loop {
            while next_end_index + 1 < x
                && s.chars()
                .nth(next_end_index)
                .unwrap_or(' ') != '\n'
            {
                next_end_index = next_end_index + 1;
                // log(format!("next_end_index = {}\nc = {}\ns = {}", next_end_index, s.chars()
                //     .nth(next_end_index )
                //     .unwrap_or(' ') as u32, s.chars()
                //                 .nth(next_end_index + 1)
                //                 .unwrap_or(' ')).as_str());
            }

            let str = s.chars()
                .skip(end_index)
                .take(next_end_index - end_index)
                .collect::<String>();
            //log(format!("end_index = {}\nnext_end_index = {}\nstr = {}",end_index,next_end_index,str).as_str());
            if str.trim().is_empty() {
                break;
            }

            end_index = next_end_index;
            if next_end_index + 1 < x
                && s.chars()
                .nth(next_end_index + 1)
                .unwrap_or(' ') != '\n'
            {
                next_end_index = next_end_index + 1;
            }
        }
    }
    // log(format!("start_index = {}\nend_index = {}\ns = {}", start_index, end_index, s.chars()
    //     .skip(start_index)
    //     .take(end_index - start_index)
    //     .collect::<String>()).as_str());
    return (start_index, end_index);
}

pub fn format_indent_increase(textarea: &HtmlTextAreaElement) {
    let (start_index, end_index) = find_block(textarea);
    let s = textarea.value();
    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            "{}",
            s.chars()
                .skip(start_index)
                .take(end_index - start_index)
                .collect::<String>()
                .split("\n")
                .map(|s| format!("    {}", s))
                .collect::<Vec<String>>()
                .join("\n")
        )
            .as_str(),
        start_index as u32,
        end_index as u32,
    );
}

pub fn format_replace_text(textarea: &HtmlTextAreaElement) {
    let start = textarea.selection_start().unwrap().unwrap();
    let end = textarea.selection_end().unwrap().unwrap();
    if start != end {} else {
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

pub fn jump_link(textarea: &HtmlTextAreaElement) {
    let s = textarea.value();
    let mut start_index = textarea.selection_start().unwrap().unwrap() as usize;
    let mut end_index = start_index;
    while start_index > 0
        && !s
        .chars()
        .nth(start_index - 1)
        .unwrap_or(' ')
        .is_whitespace()
    {
        start_index = start_index - 1;
    }
    let x = s.chars().count();
    while end_index + 1 < x && !s.chars().nth(end_index).unwrap_or(' ').is_whitespace() {
        end_index = end_index + 1;
    }
    let w = web_sys::window().unwrap();
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html#method.open_with_url_and_target
    w.open_with_url_and_target(
        s.chars()
            .skip(start_index)
            .take(end_index - start_index)
            .collect::<String>()
            .as_str(),
        "_blank",
    );
}

async fn load_article(path: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("/api/article{}", path);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}

pub fn load_data(textarea: &HtmlTextAreaElement) {
    let window = web_sys::window().unwrap();
    let search = window.location().search().unwrap();
    if Regex::new(r"\?path=.+").unwrap().is_match(&search) {
        let textarea = textarea.clone();
        spawn_local(async move {
            let s = load_file(&search)
                .await
                .unwrap().as_string().unwrap();
            textarea.set_value(&s);
        });
        return;
    }
    {
        if Regex::new(r"\?id=[0-9]+").unwrap().is_match(&search) {
            let textarea = textarea.clone();
            spawn_local(async move {
                let s = load_server(&search)
                    .await
                    .unwrap().as_string().unwrap();
                textarea.set_value(&s);
            });
            return;
        };
    };
    {
        if Regex::new(r"\?article=[0-9]+").unwrap().is_match(&search) {
            let textarea = textarea.clone();
            spawn_local(async move {
                let s = load_article(&search)
                    .await
                    .unwrap().as_string().unwrap();
                let obj: Value = serde_json::from_str(&s).unwrap();
                textarea.set_value(format!("{}|{}\n\n{}", obj["title"].as_str().unwrap(), obj["tags"].to_string(), obj["content"].as_str().unwrap()).as_str());
            });
            return;
        };
    };
}

async fn load_file(path: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("/api/file{}", path);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}

async fn load_server(path: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("/api/note{}", path);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.text()?).await?;
    Ok(json)
}

pub async fn post_data(url: &str, json_body: &str) -> Result<JsValue, JsValue> {
    // https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RequestInit.html
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(json_body)));
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.text()?).await?;
    // Send the JSON response back to JS.
    Ok(json)
}

fn save_article(textarea: &HtmlTextAreaElement) {
    if let Ok(m) = collect_article(textarea) {
        spawn_local(async move {
            let mut url = "/api/article".to_string();
            post_data(url.as_str(), &m).await;
        })
    }
}

pub fn save_data(textarea: &HtmlTextAreaElement, toast: &Element) {
    let window = web_sys::window().unwrap();
    let regex = Regex::new(r"\?path=.+").unwrap();
    let search = window.location().search().unwrap();
    if regex.is_match(&search) {
        save_local_file(textarea, search)
    } else if Regex::new(r"\?article=[0-9]+").unwrap().is_match(&search) {
        save_article(textarea);
    } else {
        save_server(textarea);
    }
    toast.set_attribute("message", "成功");
}

fn save_local_file(textarea: &HtmlTextAreaElement, search: String) {
    let textarea = textarea.clone();
    spawn_local(async move {
        let s = format!("/api/file{}", search);
        post_data(s.as_str(), &textarea.value()).await;
    })
}

fn save_server(textarea: &HtmlTextAreaElement) {
    if let Ok(m) = collect_data(textarea) {
        let window = web_sys::window().unwrap();
        let search = window.location().search().unwrap();
        let mut id = String::new();
        if Regex::new(r"\?id=[0-9]+").unwrap().is_match(&search) {
            id = search.substring_after("=");
        };
        spawn_local(async move {
            let mut url = "/api/note/insert".to_string();
            if !id.is_empty() {
                url = format!("/api/note/insert?id={}", id)
            }
            post_data(url.as_str(), &m).await;
        })
    }
}