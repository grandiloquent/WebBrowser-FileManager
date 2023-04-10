mod handlers;
mod seek_stream;
mod models;
mod utils;
mod headers;

#[macro_use]
extern crate rocket;
extern crate core;
extern crate diesel;

use rocket::config::LogLevel;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use local_ip_address::local_ip;
use rocket::{Build, Config, Rocket};
use rocket::figment::providers::{Format, Toml};
use rusqlite::Connection;

use rocket_sync_db_pools::{database};

#[database("notes")]
pub struct NotesConnection(diesel::SqliteConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // 获取本机在局域网中的IP地址
    let my_local_ip = local_ip().unwrap();

    let limits = Limits::default()
        .limit("json", 3.mebibytes())
        .limit("string", 3.mebibytes())
        .limit("file", 5.gibibytes());
    let toml = Toml::string(r#"
    [default.databases]
    notes = { url = "notes.db", pool_size = 1 }
     "#).nested();
    let figment = Figment::from(rocket::Config::default())
        .merge((rocket::Config::ADDRESS, my_local_ip))
        .merge((rocket::Config::PORT, 3000))
        .merge((rocket::Config::TEMP_DIR, ""))
        .merge((rocket::Config::LIMITS, limits))

        .merge((rocket::Config::LOG_LEVEL, LogLevel::Critical))
        .merge(toml);

    rocket::custom(figment)
        .attach(NotesConnection::fairing())
        .mount("/",
               routes![handlers::api_asset_file::api_asset_file,handlers::api_file::api_file,handlers::api_file::api_file_post,handlers::api_files::api_files,handlers::api_file_delete::api_file_delete,handlers::api_file_new_dir::api_file_new_dir,handlers::api_file_new_file::api_file_new_file,handlers::auto_it::auto_it,handlers::compress_zip::compress_zip,handlers::db::get_notes,handlers::db::get_snippet,handlers::db::insert_snippet,handlers::db::get_notes_page,handlers::editor::index,handlers::extract_zip::api_zip,handlers::favicon::favicon,handlers::file::file,handlers::index::index,handlers::rust::wasm,handlers::rust::rustfmt,handlers::video::videos,handlers::video::video,handlers::video::subtitle])
        .register("/", catchers![   handlers::not_found::not_found])
        .launch().await?;

    Ok(())
}
