// use chrono::{Datelike, Utc};
// use std::path::PathBuf;
use std::ffi::OsStr;
use std::path::Path;
use uuid::Uuid;

// build unique filename
pub fn uuid_file_name(file_path: &String) -> String {
    let uuid = Uuid::new_v4();
    let mut name = uuid.to_string();

    if let Some(ext) = Path::new(file_path).extension().and_then(OsStr::to_str) {
        name = format!("{}.{}", uuid, ext)
    }
    // let now = Utc::now();
    // let (_, year) = now.year_ce();
    // format!("{}_{}_{}_{}", year, now.month(), now.day(), uuid)
    name
}

// build "unique" filename with current date prefix
// pub fn file_name_with_prefix(file_path: &String) -> String {
//     let now = Utc::now();
//     let (_, year) = now.year_ce();
//     format!("{}_{}_{}_{}", year, now.month(), now.day(), file_path)
// }

// copy file from /tmp to static/upload with new filename
pub fn save_file(path: &std::path::PathBuf, file_name: &String) {
    if let Err(error) = std::fs::copy(path, format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}

// delete file from static/upload
pub fn delete_file(file_name: &String) {
    if let Err(error) = std::fs::remove_file(format!("static/upload/{}", file_name)) {
        println!("File error: {}", error);
    }
}
