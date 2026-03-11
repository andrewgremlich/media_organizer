use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

/// Returns the creation date of the file at the given path as a formatted string (`YYYY-MM-DD`).
///
/// # Arguments
///
/// * `path_str` - A reference to a `Path` representing the file path.
///
/// # Returns
///
/// * `Ok(String)` containing the formatted creation date if successful.
/// * `Err(String)` if the file does not exist or the creation date cannot be retrieved.
pub fn file_created(path_str: &Path) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(created) = data.created() {
                let datetime: DateTime<Local> = created.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file creation date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

/// Returns the last modification date of the file at the given path as a formatted string (`YYYY-MM-DD`).
///
/// # Arguments
///
/// * `path_str` - A reference to a `Path` representing the file path.
///
/// # Returns
///
/// * `Ok(String)` containing the formatted modification date if successful.
/// * `Err(String)` if the file does not exist or the modification date cannot be retrieved.
pub fn file_modified(path_str: &Path) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(modified) = data.modified() {
                let datetime: DateTime<Local> = modified.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file modified date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

/// Returns the last accessed date of the file at the given path as a formatted string (`YYYY-MM-DD`).
///
/// # Arguments
///
/// * `path_str` - A reference to a `Path` representing the file path.
///
/// # Returns
///
/// * `Ok(String)` containing the formatted last accessed date if successful.
/// * `Err(String)` if the file does not exist or the accessed date cannot be retrieved.
pub fn last_accessed(path_str: &Path) -> Result<String, String> {
    match fs::metadata(path_str) {
        Ok(data) => {
            if let Ok(accessed) = data.accessed() {
                let datetime: DateTime<Local> = accessed.into();
                let formatted_date = datetime.format("%Y-%m-%d").to_string();

                Ok(formatted_date)
            } else {
                Err("Failed to read file last accessed date".to_string())
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_temp_file() -> tempfile::NamedTempFile {
        let mut f = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        writeln!(f, "test content").unwrap();
        f
    }

    #[test]
    fn can_read_creation_string() {
        let f = create_temp_file();
        let result = file_created(f.path()).unwrap();
        let today = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(result, today);
    }

    #[test]
    fn can_read_modified_string() {
        let f = create_temp_file();
        let result = file_modified(f.path()).unwrap();
        let today = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(result, today);
    }

    #[test]
    fn can_read_accessed_string() {
        let f = create_temp_file();
        let result = last_accessed(f.path()).unwrap();
        let today = Local::now().format("%Y-%m-%d").to_string();
        assert_eq!(result, today);
    }
}
