use web_sys::{Document, HtmlElement};

pub fn set_style(document: &Document, body: &HtmlElement) {
    let style = document.create_element("style").unwrap();
    style.set_text_content(Some(
        r#"
    html {
        color: #0f0f0f;
        background-color: #fff;
        font-size: 10px;
        font-family: Roboto,Arial,sans-serif
    }
    body {
        margin: 0;
        padding: 0;
        padding: 0 env(safe-area-inset-right) env(safe-area-inset-bottom) env(safe-area-inset-left);
        font-size: 1.2rem;
        overflow-x: hidden
    }
    "#,
    ));
    body.append_child(&style);
}
