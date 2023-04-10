
use super::strings::find_current_line;
use web_sys::HtmlTextAreaElement;

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
