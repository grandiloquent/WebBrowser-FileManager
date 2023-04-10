use urlencoding::encode;
use utils::date::seconds_to_duration;
use utils::dom::get_query_string;
use utils::dom::query_selector;
use utils::dom::set_text_content;
use utils::strings::StringExt;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlInputElement;
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
struct Video {
    // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html
    video: HtmlVideoElement,
    path: String,
    path_separator: String,
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
            path_separator: path_separator.to_string(),
        }
    }
    fn init(&self) {
        self.video.set_src(
            format!("/api/file?path={}", encode(self.path.as_str()).into_owned()).as_str(),
        );
        self.set_title();
        self.set_play_event();
        // remaining
        // https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlVideoElement.html
        // http://192.168.8.189:3000/video/videos
        {
            onclick!((".fullscreen",&self.document)->{
            let video = self.video.clone();
            let d=self.document.clone();
            move || match d.fullscreen_element() {
                Some(v) => {
                }
                None => {
                    d
                        .document_element()
                        .unwrap()
                        .request_fullscreen();
                }
            }
            });
        }

        {
            onclick!((".button--back",&self.document)->{
            let video = self.video.clone();
            move || {
                let time = video.current_time();
                if time - 10.0 > 0.0 {
                    video.set_current_time(time - 10.0);
                }
            }
            });
        }

        {
            onclick!((".button--forward",&self.document)->{
                        let video=self.video.clone();
                        move || {
                            let   time=video.current_time();
            if time+10.0<=video.duration(){
                video .set_current_time(time+10.0);
            }

                        }
                    });
        }
        {
            let element = query_selector(&self.document, ".range")
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let video = self.video.clone();

            let handler = move |event: Event| {
                event.prevent_default();
                let value = (event
                    .current_target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value()
                    .as_str()
                    .parse::<u16>()
                    .unwrap() as f32)
                    / 100.0;
                video.set_current_time((value as f64) * video.duration());
            };
            let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(Event)>);
            element.set_onchange(closure.as_ref().dyn_ref());
            closure.forget();
        }
        handler!( (set_ontimeupdate,self.video)->{
            let video = self.video.clone();
            let element = query_selector(&self.document, ".elapsed").unwrap();
            move || {
                element.set_text_content(Some(
                    seconds_to_duration(video.current_time() as u64).as_str(),
                ));
            }
        });

        {
            let video = self.video.clone();
            let element = query_selector(&self.document, ".remaining").unwrap();
            let handler = Closure::wrap(Box::new(move || {
                log(format!(
                    "Video Size: {}x{}",
                    video.video_width(),
                    video.video_height()
                )
                .as_str());
                element
                    .set_text_content(Some(seconds_to_duration(video.duration() as u64).as_str()));
            }) as Box<dyn FnMut()>);
            self.video.set_ondurationchange(handler.as_ref().dyn_ref());
            handler.forget();
        }
    }
    fn set_title(&self) {
        let _ = set_text_content(
            &self.document,
            ".video-player-title",
            self.path
                .substring_after_last(self.path_separator.as_str())
                .as_str(),
        );
    }
    fn set_play_event(&self) {
        onclick!((".playback-play__play",&self.document)->{
            let video=self.video.clone();
            move || {
                if video.paused()
                {
                    video.play();
                }
                else{
                    video.pause();
                }
            }
        });
        let path = query_selector(&self.document, ".playback-controls-play path").unwrap();

        {
            let path = path.clone();
            let handler = Closure::wrap(Box::new(move || {
                let _=  path.set_attribute("d", "M113.411 123.175h12.94c6.103 0 9.13-3.027 9.13-9.13V44.073c0-5.86-3.027-8.936-9.13-9.131h-12.94c-6.103 0-9.18 3.027-9.18 9.13v69.971c-.146 6.104 2.881 9.131 9.18 9.131Zm43.604 0h12.939c6.104 0 9.18-3.027 9.18-9.13V44.073c0-5.86-3.076-9.131-9.18-9.131h-12.94c-6.103 0-9.18 3.027-9.18 9.13v69.971c0 6.104 2.93 9.131 9.18 9.131Z");
            }) as Box<dyn FnMut()>);
            self.video.set_onplay(handler.as_ref().dyn_ref());
            handler.forget();
        }
        {
            let path = path.clone();
            let handler = Closure::wrap(Box::new(move || {
                let _=  path.set_attribute("d", "M113.428 127.863c2.588 0 5.03-.733 8.448-2.686l60.302-35.01c4.883-2.88 8.008-6.103 8.008-11.084 0-4.98-3.125-8.203-8.008-11.035l-60.302-35.01c-3.418-2.002-5.86-2.685-8.448-2.685-5.566 0-10.742 4.248-10.742 11.67v74.17c0 7.422 5.176 11.67 10.742 11.67Z");
            }) as Box<dyn FnMut()>);
            self.video.set_onpause(handler.as_ref().dyn_ref());
            handler.forget();
        }
    }
}
