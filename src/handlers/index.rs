use std::path::{Path};
use crate::seek_stream::seekstream::SeekStream;
#[get("/")]
pub fn index<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/index/index.html");
    SeekStream::from_path(p)
}