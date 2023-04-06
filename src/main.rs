mod handlers;
mod seek_stream;

#[macro_use]
extern crate rocket;

use rocket::config::LogLevel;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use local_ip_address::local_ip;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let my_local_ip = local_ip().unwrap();

    let limits = Limits::default()
        .limit("json", 3.mebibytes())
        .limit("string", 3.mebibytes())
        .limit("file", 5.gibibytes());

    let figment = Figment::from(rocket::Config::default())
        .merge((rocket::Config::ADDRESS, my_local_ip))
        .merge((rocket::Config::PORT, 3000))
        .merge((rocket::Config::TEMP_DIR, ""))
        .merge((rocket::Config::LIMITS, limits))
        .merge((rocket::Config::LOG_LEVEL, LogLevel::Critical));

    rocket::custom(figment)
        .mount("/",
               routes![handlers::asset::index,handlers::asset::api_file,handlers::asset::file,handlers::asset::api_files])
        .register("/", catchers![   handlers::not_found::not_found])
        .launch().await?;

    Ok(())
}
