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

# Dry run (preview without moving files or creating folders)
cargo run -p media_organizer -- --source ./test-media --destination ./sorted_media --dry-run

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
| Photos   | JPEG, JPG, PNG, HEIF, HEIC, TIFF, AVIF, WebP, DNG, GIF, RAW |
| Videos   | MP4, MOV, AVI |
| Audio    | MP3, WAV, AIFF, M4A, FLAC |
| Documents | DOC, DOCX, PDF, EPUB, MOBI, TXT, MD, ODT, RTF, PPTX, XLSX |

## CLI Options

| Flag | Short | Description |
|------|-------|-------------|
| `--source` | `-s` | Source folder of media to sort (required) |
| `--destination` | `-d` | Destination folder for sorted media (default: `sorted`) |
| `--file-type` | `-f` | File type glob to filter (default: `*`) |
| `--copy` | `-c` | Copy files instead of moving them |
| `--dry-run` | `-y` | Preview sorting without moving files or creating folders |
| `--log-saved` | `-l` | Write each saved file to `saved_file.log` |
| `--dimensions` | | Append width x height to image and video filenames |
| `--verbose` | `-v` | Print log output to the terminal |

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development setup and guidelines.

## License

[MIT](./LICENSE)
