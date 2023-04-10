#[allow(unused_macros)]
#[macro_export]
macro_rules! handler {
    ( ($name:ident,$element:expr) -> $v:expr) => {
        let handler = Closure::wrap(Box::new($v) as Box<dyn FnMut()>);
        $element.$name(handler.as_ref().dyn_ref());
        handler.forget();
    };
}
