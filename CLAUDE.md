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

## Common Tasks

- To add a new media type, follow the pattern in `media_organizer/src/organizer/make_file_destination/mod.rs` — add a whitelist function, detection function, and wire it into `sort_and_make`.
- Document sorting is partially implemented — `media_info` has `read_doc_creation_date` but the CLI doesn't route to it yet. Use `/wire-doc-sorting` to complete it.
- When adding file extensions, always include both lowercase and uppercase variants in the whitelist.
- The `ffmpeg-next` version must match the system FFmpeg version. Check compatibility before upgrading.

## Architecture Notes

- Config is passed via environment variables (`DEST_FOLDER`, `FILE_TYPE`, `COPY`) set in `main.rs` using `unsafe env::set_var`. This is intentional — don't refactor to struct-passing without being asked.
- Each media type module in `media_info` follows the same pattern: a `read_*_creation_date` function and an info struct with `::new(path)`.
- The `get_exif_field!` macro in `media_info` handles EXIF field extraction with fallback to empty strings.

## Linting

- Run `cargo clippy --workspace -- -D warnings` before committing.
- Run `cargo test --workspace` to verify all crates pass.

## Slash Commands

- `/test` — Run workspace tests, diagnose and fix failures
- `/check` — Full pipeline: cargo check + clippy + test
- `/wire-doc-sorting` — Complete the partial document sorting feature in the CLI
- `/add-media-type <type>` — Template for adding a new file type end-to-end
- `/explore <topic>` — Trace and explain how a part of the codebase works
- `/upgrade-deps` — Safely check and upgrade outdated dependencies
- `/debug-media <file>` — Debug why a specific media file isn't sorting correctly
- `/release` — Prepare a versioned release (tests, lint, version bump)

## Key Dependencies

- **ffmpeg-next** (v7.1.0) - Video metadata extraction
- **kamadak-exif** - EXIF/photo metadata
- **id3** - Audio ID3 tag reading
- **clap** - CLI argument parsing
- **chrono** - Date/time handling
- **epub, mobi, pdf, docx** - Document metadata readers
