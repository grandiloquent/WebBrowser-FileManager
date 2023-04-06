use std::fmt::Error;
use std::path::{Path, PathBuf};
use rocket::serde::json::{json, Value};
use std::fs;
use rocket::futures::future::err;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::State;

#[get("/compress_dir?<path>")]
pub async fn compress_zip(path: String) -> Result<(), Status> {
// https://doc.rust-lang.org/std/path/struct.Path.html
    match list_files(path.as_str()) {
        Ok(_) => {}
        Err(_) => {
            return Err(Status::NotFound);
        }
    }
    Ok(())
}

fn list_files(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut list = Vec::new();
    let read_dir = fs::read_dir(path)?;
    for entry in read_dir {
        let dir_entry = entry?;
        list.push(dir_entry.path())
    }
    return Ok(list);
}