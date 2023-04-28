use super::strings::find_current_line;
use web_sys::HtmlTextAreaElement;
use crate::utils::utils::log;

pub fn format_comment(textarea: &HtmlTextAreaElement, expr: &str) {
    let s = textarea.value();
    let start = textarea.selection_start().unwrap().unwrap();
    let (start_index, end_index) = find_current_line(s.as_str(), start as usize);

    let mut j = end_index + 1;
    let x=s.chars().count();
    while j + 1 < x
        && s.chars().nth(j+1).unwrap().is_whitespace()
    {
        j = j + 1;
    }
    let mut prefix = String::new();
    if j > end_index {
        prefix =s.chars().skip(end_index + 1).take(j-end_index).collect::<String>();
    }
    // log(format!(
    //     "{}\n{}\n{}",
    //     start_index,
    //     end_index,
    //     &s[start_index..end_index]
    // )
    // .as_str());
    let s = s.chars().skip(start_index).take(end_index - start_index)
        .collect::<Vec<char>>()
        .chunks(18)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .iter()
        .map(|l| format!("{}{} {}", prefix, expr, l))
        .collect::<Vec<String>>()
        .join("\n");

    let _ = textarea.set_range_text_with_start_and_end(
        s.as_str(),
        start_index as u32,
        end_index as u32,
    );
}
