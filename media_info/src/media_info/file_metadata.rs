use chrono::{DateTime, Local};
use std::fs;

/// Returns the creation date of the file.
/// 
/// # Examples
/// ```
/// let audio_path = "tests/test_data/audio.mp3";
/// let creation_date = file_created(audio_path);
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn file_created(path_str: &str) -> String {
    let file_metadata = fs::metadata(path_str).expect("Failed to read file metadata");
    let created = file_metadata
        .created()
        .expect("Failed to read file creation date");
    let datetime: DateTime<Local> = created.into();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    formatted_date
}
