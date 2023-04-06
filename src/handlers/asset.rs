use std::path::Path;
use crate::seek_stream::seekstream::SeekStream;

#[get("/")]
pub fn index<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/index/index.html");
    SeekStream::from_path(p)
}

#[get("/api/file?<path>")]
pub fn api_file<'a>(path: String) -> std::io::Result<SeekStream<'a>> {
    let p = Path::new(path.as_str());
    SeekStream::from_path(p)
}

#[get("/<dir>/<path>")]
pub fn file<'a>(dir: String, path: String) -> std::io::Result<SeekStream<'a>> {
    let p = Path::new(format!("assets/{}/{}", dir, path).as_str());
    SeekStream::from_path(p)
}


