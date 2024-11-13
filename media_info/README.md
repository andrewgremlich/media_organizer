# Media Info

A tool to extract media information from a media source.

This is a supplemental library crate for [Media Organizer](https://crates.io/crates/media_organizer).

## API

```rust
read_audio_creation_date(path_str: &str) -> Result<String, String>

read_photo_creation_date(path_str: &str) -> Result<String, String>

read_video_creation_date(path_str: &str) -> Result<String, String>
```

## TODO

1. More metadata functions.
