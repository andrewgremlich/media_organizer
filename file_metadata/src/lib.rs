use chrono::{DateTime, Local};
use std::fs;

pub fn file_created(path_str: &str) -> String {
    let file_metadata = fs::metadata(path_str).expect("Failed to read file metadata");
    let created = file_metadata
        .created()
        .expect("Failed to read file creation date");
    let datetime: DateTime<Local> = created.into();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    return formatted_date;
}
