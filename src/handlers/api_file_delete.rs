use std::path::{Path};
use rocket::serde::json::{json, Value};
use std::fs;
use rocket::futures::future::err;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::State;

fn remove_file(path: &Path) -> Result<(), std::io::Error> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

#[post("/api/file/delete", data = "<list>")]
pub async fn api_file_delete(list: Json<Vec<String>>) -> Result<(), Status> {
// https://doc.rust-lang.org/std/path/struct.Path.html
    for path in list.into_inner() {
        let p = Path::new(path.as_str());

        // https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html
        match remove_file(p) {
            Ok(()) => continue,
            Err(error) => {
                println!("{}", error);
                return Err(Status::NotFound);
            }
        };
    }
    Ok(())
}