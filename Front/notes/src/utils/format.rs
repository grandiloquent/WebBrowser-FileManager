use regex::Regex;
use web_sys::HtmlTextAreaElement;

use super::utils::log;

pub fn format_comment(textarea: &HtmlTextAreaElement, expr: &str) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap() as usize;

    let length = s.chars().count();
    let mut start_index = start;
    while start_index > 0 && s.chars().nth(start_index - 1).unwrap() != '\n' {
        start_index = start_index - 1;
    }
    let mut end_index = start;
    while end_index + 1 < length && s.chars().nth(end_index + 1).unwrap() != '\n' {
        end_index = end_index + 1;
    }

    let s = s
        .chars()
        .skip(start_index)
        .take(end_index - start_index + 1)
        .collect::<String>();

    let mut buf: Vec<String> = Vec::new();
    let mut index = 0;
    let mut str = String::new();
    let mut count = 0;
    let regex = Regex::new(r"[\u4e00-\u9fa5\\W]").unwrap();
    let length = s.chars().count();
    loop {
        let ch = s.chars().nth(index).unwrap();
        str.push(ch);
        count = count + 1;
        if count > 3 {
            if regex.is_match(&ch.to_string())
                || (index + 1 < length
                    && regex.is_match(&(s.chars().nth(index + 1).unwrap()).to_string()))
            {
                buf.push(str.clone());
                str.clear();
                count = 0;
            }
        }
        index = index + 1;
        if index == length {
            break;
        }
    }
    buf.push(str);
    let mut prefix = String::new();
    let length = textarea.value().chars().count();
    let mut end = end_index + 3;
    if (end + 1 < length) {
        while end + 1 < length
            && (textarea
                .value()
                .chars()
                .nth(end + 1)
                .unwrap()
                .is_whitespace()
                && textarea.value().chars().nth(end + 1).unwrap() != '\n')
        {
            end = end + 1;
        }
       
        prefix = textarea
            .value()
            .chars()
            .skip(end_index + 2)
            .take(end - end_index - 1)
            .collect::<String>();
    }

    let _ = textarea.set_range_text_with_start_and_end(
        format!(
            "{}{} {}",
            prefix,
            expr,
            buf.join(format!("\n{}{} ", prefix, expr).as_str())
        )
        .as_str(),
        start_index as u32,
        (end_index + 1) as u32,
    );
}
