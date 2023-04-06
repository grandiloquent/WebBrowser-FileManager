use std::path::{Path, PathBuf};
use crate::seek_stream::seekstream::SeekStream;
#[get("/<path..>")]
pub fn file<'a>(path: PathBuf) -> std::io::Result<SeekStream<'a>> {
    SeekStream::from_path(Path::new(format!("assets/{}", path.to_str().unwrap_or("")).as_str()))
}