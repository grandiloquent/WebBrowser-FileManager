use std::fs;
use std::path::{Path, PathBuf};
use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::seek_stream::seekstream::SeekStream;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[get("/")]
pub fn index<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/index/index.html");
    SeekStream::from_path(p)
}