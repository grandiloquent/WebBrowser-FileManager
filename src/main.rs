mod handlers;

#[macro_use]
extern crate rocket;

use rocket::config::LogLevel;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use local_ip_address::local_ip;
use rocket_contrib::serve::StaticFiles;

#[launch]
fn rocket() -> _ {
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
        .mount("/", StaticFiles::from("/assets"))
        .mount("/",
               routes![
                   handlers::index::index
               ])
}

