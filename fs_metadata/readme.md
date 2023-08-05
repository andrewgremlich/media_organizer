# FS Metadata

A simple wrapper over Rust File Metadata API. The original intent is to handle System time better and providing a default date string for a creation date.

## Functions

```rust
pub fn file_created(path_str: &str) -> Result<String, String>

pub fn file_modified(path_str: &str) -> Result<String, String>

pub fn last_accessed(path_str: &str) -> Result<String, String>
```

## Run Tests

To activate the tests, run `cargo test -p fs_metadata`.
