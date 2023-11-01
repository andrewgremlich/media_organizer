use chrono::{DateTime, Local};
use std::fs;

pub fn file_created(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(created) = data.created() {
                let datetime: DateTime<Local> = created.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                return Ok(formatted_date);
            } else {
                return Err("Failed to read file creation date".to_string());
            }
        }
        Err(err) => return Err(err.to_string()),
    }
}

pub fn file_modified(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(modified) = data.modified() {
                let datetime: DateTime<Local> = modified.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                return Ok(formatted_date);
            } else {
                return Err("Failed to read file modified date".to_string());
            }
        }
        Err(err) => return Err(err.to_string()),
    }
}

pub fn last_accessed(path_str: &str) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(accessed) = data.accessed() {
                let datetime: DateTime<Local> = accessed.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                return Ok(formatted_date);
            } else {
                return Err("Failed to read file last accessed date".to_string());
            }
        }
        Err(err) => return Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_creation_string() {
        let result = file_created("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result, "2023-10-30");
    }

    #[test]
    fn can_read_modified_string() {
        let result = file_modified("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result, "2023-10-30");
    }

    #[test]
    fn can_read_accessed_string() {
        let result = last_accessed("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result, "2023-10-31");
    }
}
