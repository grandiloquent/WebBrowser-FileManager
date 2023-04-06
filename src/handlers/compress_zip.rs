use std::path::{Path};
use rocket::serde::json::{json, Value};
use std::fs;
use rocket::futures::future::err;
use rocket::http::Status;
use rocket::serde::{json::Json};
use rocket::State;

#[post("/compress_zip", data = "<list>")]
pub async fn compress_zip(list: Json<Vec<String>>) -> Result<(), Status> {
// https://doc.rust-lang.org/std/path/struct.Path.html
    for path in list.into_inner() {
        let p = Path::new(path.as_str());
    }
    Ok(())
}