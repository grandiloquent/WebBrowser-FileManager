use std::path::{Path};
// https://doc.rust-lang.org/std/fs/
use std::fs;
use rocket::http::Status;
#[get("/api/file/new_dir?<path>")]
pub fn api_file_new_dir(path: String)  -> Result<(), Status> {
    let p = Path::new(path.as_str());
// https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file
    if !p.is_dir() {
        return match fs::create_dir_all(p) {
            Ok(_) => {
                Ok(())
            }
            // https://doc.rust-lang.org/std/io/struct.Error.html
            Err(error) => {
                Err(Status::NotFound)
            }
        };
    }
    Err(Status::NotFound)
}