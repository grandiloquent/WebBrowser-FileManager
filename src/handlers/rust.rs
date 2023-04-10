use std::{env, io};
use std::path::Path;
use std::process::{Child, Command};
use rocket::http::Status;
fn run_wasm_pack(dir: &str, path: &str) -> io::Result<Child> {
    if Path::new(dir).is_dir() {
        env::set_current_dir(dir);
    }
    Command::new("cmd")
        .args(["/C","wasm-pack","build", "--target", "web", "--out-dir", path])
        .spawn()
}
#[get("/wasm?<dir>&<path>")]
pub async fn wasm(dir: String, path: String) -> Result<(), Status> {
    run_wasm_pack(dir.as_str(), path.as_str());
    Ok(())
}
fn run_rustfmt(path: &str) -> io::Result<Child> {
    Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .spawn()
}
#[get("/rustfmt?<path>")]
pub async fn rustfmt(path: String) -> Result<(), Status> {
    run_rustfmt(path.as_str());
    Ok(())
}