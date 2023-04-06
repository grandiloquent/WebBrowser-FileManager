




use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileItem {
    pub path: String,
    pub is_directory: bool,
}