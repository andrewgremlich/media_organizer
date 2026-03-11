# FS Metadata

A cross-platform wrapper for file system metadata.

- Access creation, modification, and last-accessed times as readable date strings (YYYY-MM-DD)
- Query file permissions (readable, writable, executable)
- Get human-readable file sizes (KB, MB, GB, TB)

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
    pub is_file_read_only: bool,
    pub is_dir: bool,
    pub is_file: bool,
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_executable: bool,
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
