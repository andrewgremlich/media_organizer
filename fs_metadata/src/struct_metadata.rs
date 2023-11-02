use chrono::{DateTime, Local};
use faccess::PathExt;
use std::fs::{self, Metadata};
use std::path::Path;

pub struct FileMetadata {
    pub created: String,
    pub modified: String,
    pub accessed: String,
    pub is_file_read_only: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_executable: bool,
    data: Metadata,
}

impl FileMetadata {
    pub fn new(path_str: &str) -> Result<Self, String> {
        let path = Path::new(path_str);
        let metadata = fs::metadata(path_str);

        match metadata {
            Ok(data) => {
                let created_system_time: DateTime<Local> = data
                    .created()
                    .expect("Could not read accessed system time")
                    .into();
                let modified_system_time: DateTime<Local> = data
                    .modified()
                    .expect("Could not read modified system time")
                    .into();
                let accessed_system_time: DateTime<Local> = data
                    .accessed()
                    .expect("Could not read accessed system time")
                    .into();

                return Ok(FileMetadata {
                    accessed: accessed_system_time.format("%Y-%m-%d").to_string(),
                    modified: modified_system_time.format("%Y-%m-%d").to_string(),
                    created: created_system_time.format("%Y-%m-%d").to_string(),
                    is_file_read_only: data.permissions().readonly(),
                    is_dir: data.is_dir(),
                    is_file: data.is_file(),
                    is_readable: path.readable(),
                    is_writable: path.writable(),
                    is_executable: path.executable(),
                    data: data,
                });
            }
            Err(_) => return Err("Failed to read metadata from file".to_string()),
        }
    }

    fn get_human_readable_file_size(&self) -> (f32, f32, f32, f32) {
        let bytes = self.data.len() as f32;
        let kilobytes = bytes / 1024.0;
        let megabytes = bytes / 1_048_576.0;
        let gigabytes = bytes / 1_073_741_824.0;
        let terabytes = bytes / 1_099_511_627_776.0;

        (kilobytes, megabytes, gigabytes, terabytes)
    }

    pub fn readable(&self) -> bool {
        self.is_readable
    }

    pub fn writable(&self) -> bool {
        self.is_writable
    }

    pub fn executable(&self) -> bool {
        self.is_executable
    }

    pub fn get_file_in_kilobytes(&self) -> f32 {
        let (k, _m, _g, _t) = self.get_human_readable_file_size();
        k as f32
    }

    pub fn get_file_in_megabytes(&self) -> f32 {
        let (_k, m, _g, _t) = self.get_human_readable_file_size();
        m as f32
    }

    pub fn get_file_in_gigabytes(&self) -> f32 {
        let (_k, _m, g, _t) = self.get_human_readable_file_size();
        g as f32
    }

    pub fn get_file_in_terabytes(&self) -> f32 {
        let (_k, _m, _g, t) = self.get_human_readable_file_size();
        t as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_file_metadata() {
        let result = FileMetadata::new("./tests/data/test_photo.JPG").unwrap();

        println!("{:?}", result.get_file_in_kilobytes());

        assert_eq!(result.created, "2023-10-30");
        assert_eq!(result.modified, "2023-10-30");
        assert_eq!(result.accessed, "2023-10-31");
    }

    #[test]
    fn has_permissions() {
        let result = FileMetadata::new("./tests/data/test_photo.JPG").unwrap();

        assert_eq!(result.is_readable, true);
        assert_eq!(result.is_writable, true);
        assert_eq!(result.is_executable, false);
    }

    #[test]
    fn is_file_read_only() {
        let result = FileMetadata::new("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result.is_file_read_only, false)
    }

    #[test]
    fn is_dir() {
        let result = FileMetadata::new("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result.is_dir, false);
    }

    #[test]
    fn is_file() {
        let result = FileMetadata::new("./tests/data/test_photo.JPG").unwrap();
        assert_eq!(result.is_file, true);
    }
}
