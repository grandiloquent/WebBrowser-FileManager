use std::path::{Path};
use rocket::serde::json::{json, Value};
use std::fs;
use rocket::futures::future::err;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::State;
#[post("/api/file/delete", data = "<list>")]
pub async fn api_file_delete(list: Json<Vec<String>>) -> Result<(), Status> {
// https://doc.rust-lang.org/std/path/struct.Path.html
    for path in list.into_inner() {
        let p = Path::new(path.as_str());
        if p.is_dir() {
            // https://doc.rust-lang.org/stable/std/fs/fn.remove_dir_all.html
            return match fs::remove_dir_all(p) {
                Ok(_) => {
                    Ok(())
                }
                Err(error) => {
                    Err(Status::NotFound)
                }
            };
        } else {
            return match fs::remove_file(p) {
                Ok(_) => {
                    Ok(())
                }
                Err(error) => {
                    Err(Status::NotFound)
                }
            };
        }
    }
    Ok(())
}