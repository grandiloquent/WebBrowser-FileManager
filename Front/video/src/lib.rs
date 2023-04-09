use js_sys::Promise;
use urlencoding::encode;
use utils::dom::get_query_string;
use utils::dom::query_selector;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlElement;
use web_sys::HtmlVideoElement;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn start(path_separator: &str) {
    let video = Video::new(path_separator);
    video.init();
}
#[allow(unused_macros)]
macro_rules! onclick {
    (($element:literal,$document:expr) -> $onclick:expr) => {
        let p = query_selector($document, $element)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        let onclick = Closure::wrap(Box::new($onclick) as Box<dyn FnMut()>);
        p.set_onclick(onclick.as_ref().dyn_ref());
        onclick.forget();
    };
}
struct Video {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html
    video: HtmlVideoElement,
    path: String,
    document: Document,
}

impl Video {
    fn new(path_separator: &str) -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Location.html
        let path = get_query_string(&window, "path");
        let video = query_selector(&document, "video")
            .unwrap()
            .dyn_into::<HtmlVideoElement>()
            .unwrap();
        Video {
            video: video,
            path,
            document,
        }
    }
    fn init(&self) {
        self.video.set_src(
            format!("/api/file?path={}", encode(self.path.as_str()).into_owned()).as_str(),
        );

        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html
       
       
        onclick!((".playback-play__play",&self.document)->{
            let video=self.video.clone();
            move || {
                video.play();
            }
        });
    }
}

//  wasm-pack build --target web --out-dir C:\Users\Administrator\Desktop\Resources\Manager\assets\video
