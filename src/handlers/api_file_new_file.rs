use std::path::{Path};
// https://doc.rust-lang.org/std/fs/
use std::fs;
use rocket::http::Status;
#[get("/api/file/new_file?<path>")]
pub fn api_file_new_file(path: String) -> Result<(), Status> {
    let p = Path::new(path.as_str());
// https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file
    if !p.is_file() {
        return match fs::write(p, b"") {
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