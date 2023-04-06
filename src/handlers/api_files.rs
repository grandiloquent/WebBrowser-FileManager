use rocket::response::content::RawJson;
use rocket::serde::json::serde_json;
use crate::utils::get_file_list::get_file_list;
const DIR: &str = r#"C:\Users\Administrator\Desktop"#;
#[get("/api/files?<path>")]
pub fn api_files(path: String) -> RawJson<String> {
    RawJson(serde_json::to_string(&get_file_list(path, DIR)).unwrap_or("".to_string()))
}