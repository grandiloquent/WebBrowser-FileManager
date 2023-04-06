use std::fs;






use crate::models::file_item::FileItem;

pub fn get_file_list(query: String, default_path: &str) -> Vec<FileItem> {
    let mut path = query;
    if path.is_empty() {
        path = default_path.to_string();
    }
    match fs::read_dir(path) {
        Ok(v) => {
            v.map(|res| res.map(|e| {
                FileItem {
                    path: e.path().display().to_string(),
                    is_directory: e.file_type().unwrap().is_dir(),
                }
            }))
                .collect::<Result<Vec<_>, std::io::Error>>().unwrap()
        }
        Err(_) => Vec::new()
    }
}
