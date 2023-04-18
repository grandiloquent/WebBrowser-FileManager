use std::path::Path;
use crate::seek_stream::SeekStream;
#[get("/notes/notes")]
pub fn get_notes_page<'a>() -> std::io::Result<SeekStream<'a>> {
    let p = Path::new("assets/notes/notes.html");
    SeekStream::from_path(p)
}