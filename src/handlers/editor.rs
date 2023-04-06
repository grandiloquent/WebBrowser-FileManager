use std::path::{Path};
use rocket::http::Status;
use crate::seek_stream::seekstream::SeekStream;
#[get("/editor")]
pub fn index<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/editor/editor.html");
    SeekStream::from_path(p)
}