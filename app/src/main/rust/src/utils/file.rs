use std::fs;
use crate::data::file_item::FileItem;

pub fn get_file_list(path: String) -> Vec<FileItem> {

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