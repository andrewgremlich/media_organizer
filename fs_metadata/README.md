# FS Metadata

A cross-platform wrapper for file system metadata.

- Access creation, modification, and last-accessed times as readable date strings (YYYY-MM-DD)
- Query file permissions (readable, writable, executable)
- Get file size in raw bytes or human-readable units (KB, MB, GB, TB)
- Detect symbolic links
- Unix-specific metadata: inode, device ID, mode, hard link count, uid/gid, block info
- Windows-specific metadata: file attributes, raw FILETIME timestamps

## Functions

Favor the Struct version instead.

```rust
pub fn file_created(path_str: &Path) -> Result<String, String>

pub fn file_modified(path_str: &Path) -> Result<String, String>

pub fn last_accessed(path_str: &Path) -> Result<String, String>
```

## Struct

```rust
struct FileMetadata {
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

    // Unix only (#[cfg(unix)])
    pub dev: u64,       // device ID
    pub ino: u64,       // inode number
    pub mode: u32,      // full permission bits
    pub nlink: u64,     // hard link count
    pub uid: u32,       // owner user ID
    pub gid: u32,       // owner group ID
    pub blksize: u64,   // filesystem block size
    pub blocks: u64,    // number of 512-byte blocks allocated

    // Windows only (#[cfg(windows)])
    pub file_attributes: u32,   // Windows attribute flags
    pub creation_time: u64,     // raw FILETIME
    pub last_write_time: u64,   // raw FILETIME
    pub last_access_time: u64,  // raw FILETIME
}

FileMetadata {
    fn new(path: &Path) -> Result<Self, String>;
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn executable(&self) -> bool;
    fn get_file_in_kilobytes(&self) -> f32;
    fn get_file_in_megabytes(&self) -> f32;
    fn get_file_in_gigabytes(&self) -> f32;
    fn get_file_in_terabytes(&self) -> f32;
}
```

## Run Tests

To activate the tests, run `cargo test -p fs_metadata`.
