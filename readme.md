# Media Organizer

A Rust workspace for organizing media files (photos, videos, audio, and documents) into a date-centric folder structure (YYYY/MM/DD). Extracts creation dates from EXIF data, FFmpeg metadata, ID3 tags, and document metadata.

## Prerequisites

- **Rust** (edition 2024)
- **FFmpeg** installed on your system (libavformat, libavcodec). See [build notes](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building). On Windows, use the _shared_ version.

## Quick Start

```bash
# Build
cargo build

# Run
cargo run -p media_organizer -- --source ./test-media --destination ./sorted_media

# Test
cargo test
```

## Examples

Copy (`-c`) from source (`-s D:\iPhone.Photos`) into destination relative to current directory (`-d iPhone.Photos1`):

```bash
..../media_organizer.exe -s D:\iPhone.Photos -d iPhone.Photos1 -c
```

## Workspace Crates

- [media_organizer](./media_organizer/readme.md) - CLI binary for organizing media files
- [media_info](./media_info/README.md) - Library for extracting metadata from photos, videos, audio, and documents
- [fs_metadata](./fs_metadata/readme.md) - Library for cross-platform file metadata (timestamps, permissions, sizes)

## Supported File Types

| Category | Extensions |
|----------|------------|
| Photos   | JPEG, JPG, PNG, HEIF, HEIC, TIFF, AVIF, WebP |
| Videos   | MP4, MOV |
| Audio    | MP3, WAV, AIFF, M4A, FLAC |
| Documents | DOCX, PDF, EPUB, MOBI, TXT, MD, ODT, RTF |

> Note: Document sorting is supported in the `media_info` library but is not yet wired into the CLI organizer.

## License

[MIT](./LICENSE)
