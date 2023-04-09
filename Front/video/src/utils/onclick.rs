
#[allow(unused_macros)]
#[macro_export]
macro_rules! onclick {
    (($element:literal,$document:expr) -> $onclick:expr) => {
        let p = query_selector($document, $element)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        let onclick = Closure::wrap(Box::new($onclick) as Box<dyn FnMut()>);
        p.set_onclick(onclick.as_ref().dyn_ref());
        onclick.forget();
    };
}
