
use std::path::{Path, PathBuf};
use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::seek_stream::seekstream::SeekStream;




#[get("/api/file?<path>")]
pub fn api_file<'a>(path: String) -> std::io::Result<SeekStream<'a>> {
    let p = Path::new(path.as_str());
    SeekStream::from_path(p)
}
