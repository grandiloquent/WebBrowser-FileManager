use std::fs;
use std::path::{Path, PathBuf};
use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::seek_stream::seekstream::SeekStream;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;



#[get("/api/file?<path>")]
pub fn api_file<'a>(path: String) -> std::io::Result<SeekStream<'a>> {
    let p = Path::new(path.as_str());
    SeekStream::from_path(p)
}

#[get("/<path..>")]
pub fn file<'a>(path: PathBuf) -> std::io::Result<SeekStream<'a>> {
    SeekStream::from_path(Path::new(format!("assets/{}", path.to_str().unwrap_or("")).as_str()))
}

const DIR: &str = r#"C:\Users\Administrator\Desktop"#;

#[get("/api/files?<path>")]
pub fn api_files(path: String) -> RawJson<String> {
    RawJson(serde_json::to_string(&get_file_list(path, DIR)).unwrap_or("".to_string()))
}
fn get_file_list(query: String, default_path: &str) -> Vec<FileItem> {
    let mut path = query;
    if path.is_empty() {
        path = default_path.to_string();
    }
    match fs::read_dir(path) {
        Ok(v) => {
            v.map(|res| res.map(|e| {
                FileItem {
                    path: e.path().display().to_string(),
                    is_directory: e.file_type().unwrap().is_dir(),
                }
            }))
                .collect::<Result<Vec<_>, std::io::Error>>().unwrap()
        }
        Err(_) => Vec::new()
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileItem {
    pub path: String,
    pub is_directory: bool,
}