# CLAUDE.md

## Project Overview

Media Organizer is a Rust workspace that organizes media files (photos, videos, audio, documents) into a date-centric YYYY/MM/DD folder structure. It extracts creation dates from EXIF, FFmpeg, ID3 tags, and document metadata.

## Workspace Structure

Three crates in a Cargo workspace (resolver 2, edition 2024):

- **media_organizer/** - CLI binary (v0.7.0). Entry point: `src/main.rs`. Uses clap for arg parsing.
- **media_info/** - Library (v0.5.1). Extracts metadata from photos/videos/audio/docs. Feature-gated (`video`, `audio`, `photo`, `doc` — all default).
- **fs_metadata/** - Library (v0.4.3). Cross-platform file metadata (timestamps, permissions, sizes).

## Build & Run

Requires FFmpeg installed on the system (libavformat, libavcodec). On Windows, use the shared version.

```bash
cargo build                    # Build all crates
cargo run -- --target <path> --destination <path> [--file-type <type>] [--copy]
```

## Testing

```bash
cargo test                     # Run all tests
cargo test -p media_info       # Run tests for a specific crate
```

Tests use example media files in `test-media/` at the workspace root. Test files are referenced via relative paths (`../test-media/`).

## Supported File Types

- **Photos:** JPEG, PNG, HEIF, HEIC, TIFF, AVIF, WebP
- **Videos:** MP4, MOV
- **Audio:** MP3, WAV, AIFF, M4A, FLAC
- **Documents:** DOCX, PDF, EPUB, MOBI, TXT, MD, ODT, RTF

## Code Conventions

- Functions return `Result<String, String>` for error handling
- Date strings formatted as `YYYY-MM-DD`, organized into `YYYY/MM/DD` directory paths
- Fallback to `"no_date_found"` when date extraction fails
- Feature-gated compilation in media_info via Cargo features
- Environment variables set in main.rs: `DEST_FOLDER`, `FILE_TYPE`, `COPY`
- File type detection uses explicit whitelists (case-sensitive with common variations)
- `get_exif_field!` macro used for EXIF field extraction in media_info

## Key Dependencies

- **ffmpeg-next** (v7.1.0) - Video metadata extraction
- **kamadak-exif** - EXIF/photo metadata
- **id3** - Audio ID3 tag reading
- **clap** - CLI argument parsing
- **chrono** - Date/time handling
- **epub, mobi, pdf, docx** - Document metadata readers
