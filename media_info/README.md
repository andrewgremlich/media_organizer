# Media Info

A tool to extract media information from a media source.

This is a supplemental library crate for
[Media Organizer](https://crates.io/crates/media_organizer).

## API

```rust
read_audio_creation_date(path_str: &Path) -> Result<String, String>

read_photo_creation_date(path_str: &Path) -> Result<String, String>

read_video_creation_date(path_str: &Path) -> Result<String, String>
```

```rust
let raw_path_str = "../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg";
let path = Path::new(raw_path_str);
let photo_info = PhotoInfo::new(path).unwrap();

assert_eq!(photo_info.date_time.contains("2024-10-22"), true);
```

## TODO

1. More metadata functions.
