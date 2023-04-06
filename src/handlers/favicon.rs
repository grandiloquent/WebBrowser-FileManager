use rocket::http::Status;
#[get("/favicon.ico")]
pub fn favicon() -> Status {
    return Status::NotFound;
}