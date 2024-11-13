use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

fn read_metadata(path_str: &Path) -> Result<fs::Metadata, String> {
    match fs::metadata(path_str) {
        Ok(data) => return Ok(data),
        Err(err) => return Err(err.to_string()),
    }
}

pub fn file_created(path_str: &Path) -> Result<String, String> {
    if !path_str.exists() {
        return Err("File does not exist".to_string());
    }

    let created_date = read_metadata(path_str)?.created().unwrap();
    let datetime: DateTime<Local> = created_date.into();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    Ok(formatted_date)
}

pub fn file_modified(path_str: &Path) -> Result<String, String> {
    if !path_str.exists() {
        return Err("File does not exist".to_string());
    }

    let modified_date = read_metadata(path_str)?.modified().unwrap();
    let datetime: DateTime<Local> = modified_date.into();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    Ok(formatted_date)
}

pub fn last_accessed(path_str: &Path) -> Result<String, String> {
    if !path_str.exists() {
        return Err("File does not exist".to_string());
    }

    let accessed_date = read_metadata(path_str)?.accessed().unwrap();
    let datetime: DateTime<Local> = accessed_date.into();
    let formatted_date = datetime.format("%Y-%m-%d").to_string();

    Ok(formatted_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_creation_string() {
        let path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = file_created(path).unwrap();
        assert_eq!(result, "2024-11-10");
    }

    #[test]
    fn can_read_modified_string() {
        let path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = file_modified(path).unwrap();
        assert_eq!(result, "2024-11-10");
    }

    #[test]
    fn can_read_accessed_string() {
        let path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = last_accessed(path).unwrap();
        assert_eq!(result, "2024-11-11");
    }
}
