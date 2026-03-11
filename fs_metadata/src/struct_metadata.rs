use chrono::{DateTime, Local};
use std::fs::{self, Metadata};
use std::path::Path;
use faccess::PathExt;

pub struct FileMetadata {
    pub created: String,
    pub modified: String,
    pub accessed: String,
    pub size_in_bytes: u64,
    pub is_file_read_only: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_symlink: bool,
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_executable: bool,
    #[cfg(unix)]
    pub dev: u64,
    #[cfg(unix)]
    pub ino: u64,
    #[cfg(unix)]
    pub mode: u32,
    #[cfg(unix)]
    pub nlink: u64,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
    #[cfg(unix)]
    pub blksize: u64,
    #[cfg(unix)]
    pub blocks: u64,
    #[cfg(windows)]
    pub file_attributes: u32,
    #[cfg(windows)]
    pub creation_time: u64,
    #[cfg(windows)]
    pub last_write_time: u64,
    #[cfg(windows)]
    pub last_access_time: u64,
    data: Metadata,
}

/// Methods for `FileMetadata`, providing functionality to create a new instance
/// from a file path, retrieve file metadata, and access file size information.
///
/// # Fields
///
/// ## Cross-platform
/// - `created`, `modified`, `accessed` — date strings (YYYY-MM-DD)
/// - `size_in_bytes` — raw file size in bytes
/// - `is_file_read_only`, `is_dir`, `is_file`, `is_symlink` — file type flags
/// - `is_readable`, `is_writable`, `is_executable` — permission checks
///
/// ## Unix only (`#[cfg(unix)]`)
/// - `dev` — device ID
/// - `ino` — inode number
/// - `mode` — full permission bits (octal)
/// - `nlink` — hard link count
/// - `uid`, `gid` — owner/group IDs
/// - `blksize`, `blocks` — filesystem block size and allocated 512-byte blocks
///
/// ## Windows only (`#[cfg(windows)]`)
/// - `file_attributes` — Windows attribute flags (hidden, system, archive, etc.)
/// - `creation_time`, `last_write_time`, `last_access_time` — raw FILETIME values
///
/// # Methods
///
/// - `new(path: &Path) -> Result<Self, String>` — construct from a file path
/// - `readable(&self) -> bool` — whether the file is readable
/// - `writable(&self) -> bool` — whether the file is writable
/// - `executable(&self) -> bool` — whether the file is executable
/// - `get_file_in_kilobytes(&self) -> f32` — file size in KB
/// - `get_file_in_megabytes(&self) -> f32` — file size in MB
/// - `get_file_in_gigabytes(&self) -> f32` — file size in GB
/// - `get_file_in_terabytes(&self) -> f32` — file size in TB
impl FileMetadata {

    pub fn new(path: &Path) -> Result<Self, String> {
        let metadata = fs::metadata(path);
        let symlink_metadata = fs::symlink_metadata(path);

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

                let is_symlink = symlink_metadata
                    .as_ref()
                    .map(|m| m.file_type().is_symlink())
                    .unwrap_or(false);

                Ok(FileMetadata {
                    accessed: accessed_system_time.format("%Y-%m-%d").to_string(),
                    modified: modified_system_time.format("%Y-%m-%d").to_string(),
                    created: created_system_time.format("%Y-%m-%d").to_string(),
                    size_in_bytes: data.len(),
                    is_file_read_only: data.permissions().readonly(),
                    is_dir: data.is_dir(),
                    is_file: data.is_file(),
                    is_symlink,
                    is_readable: path.readable(),
                    is_writable: path.writable(),
                    is_executable: path.executable(),
                    #[cfg(unix)]
                    dev: std::os::unix::fs::MetadataExt::dev(&data),
                    #[cfg(unix)]
                    ino: std::os::unix::fs::MetadataExt::ino(&data),
                    #[cfg(unix)]
                    mode: std::os::unix::fs::MetadataExt::mode(&data),
                    #[cfg(unix)]
                    nlink: std::os::unix::fs::MetadataExt::nlink(&data),
                    #[cfg(unix)]
                    uid: std::os::unix::fs::MetadataExt::uid(&data),
                    #[cfg(unix)]
                    gid: std::os::unix::fs::MetadataExt::gid(&data),
                    #[cfg(unix)]
                    blksize: std::os::unix::fs::MetadataExt::blksize(&data),
                    #[cfg(unix)]
                    blocks: std::os::unix::fs::MetadataExt::blocks(&data),
                    #[cfg(windows)]
                    file_attributes: std::os::windows::fs::MetadataExt::file_attributes(&data),
                    #[cfg(windows)]
                    creation_time: std::os::windows::fs::MetadataExt::creation_time(&data),
                    #[cfg(windows)]
                    last_write_time: std::os::windows::fs::MetadataExt::last_write_time(&data),
                    #[cfg(windows)]
                    last_access_time: std::os::windows::fs::MetadataExt::last_access_time(&data),
                    data,
                })
            }
            Err(_) => Err("Failed to read metadata from file".to_string()),
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
        k
    }

    pub fn get_file_in_megabytes(&self) -> f32 {
        let (_k, m, _g, _t) = self.get_human_readable_file_size();
        m
    }

    pub fn get_file_in_gigabytes(&self) -> f32 {
        let (_k, _m, g, _t) = self.get_human_readable_file_size();
        g
    }

    pub fn get_file_in_terabytes(&self) -> f32 {
        let (_k, _m, _g, t) = self.get_human_readable_file_size();
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid_date(s: &str) -> bool {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok()
    }

    #[test]
    fn can_make_file_metadata() {
        let result = FileMetadata::new(Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg")).unwrap();

        assert!(is_valid_date(&result.created), "created date '{}' is not YYYY-MM-DD", result.created);
        assert!(is_valid_date(&result.modified), "modified date '{}' is not YYYY-MM-DD", result.modified);
        assert!(is_valid_date(&result.accessed), "accessed date '{}' is not YYYY-MM-DD", result.accessed);
    }

    #[test]
    fn has_permissions() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();

        assert_eq!(result.is_readable, true);
        assert_eq!(result.is_writable, true);
        assert_eq!(result.is_executable, false);
    }

    #[test]
    fn is_file_read_only() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert_eq!(result.is_file_read_only, false)
    }

    #[test]
    fn is_dir() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert_eq!(result.is_dir, false);
    }

    #[test]
    fn is_file() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert_eq!(result.is_file, true);
    }

    #[test]
    fn can_get_file_size() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert_eq!(true, result.get_file_in_kilobytes() > 0.0);
        assert_eq!(true, result.get_file_in_megabytes() > 0.0);
        assert_eq!(true, result.get_file_in_gigabytes() > 0.0);
        assert_eq!(true, result.get_file_in_terabytes() > 0.0);
    }

    #[test]
    fn has_size_in_bytes() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert!(result.size_in_bytes > 0);
    }

    #[test]
    fn is_not_symlink() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert_eq!(result.is_symlink, false);
    }

    #[cfg(unix)]
    #[test]
    fn has_unix_metadata() {
        let path: &Path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let result = FileMetadata::new(path).unwrap();
        assert!(result.ino > 0);
        assert!(result.nlink > 0);
        assert!(result.mode > 0);
        assert!(result.blksize > 0);
        assert!(result.blocks > 0);
    }
}
