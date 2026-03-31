# CLAUDE.md

## Project Overview

Media Organizer is a Rust workspace that organizes media files (photos, videos, audio, documents) into a date-centric YYYY/MM/DD folder structure. It extracts creation dates from EXIF, FFmpeg, ID3 tags, and document metadata.

## Workspace Structure

Three crates in a Cargo workspace (resolver 2, edition 2024):

- **media_organizer/** - CLI binary (v0.9.5). Entry point: `src/main.rs`. Uses clap for arg parsing.
- **media_info/** - Library (v0.7.3). Extracts metadata from photos/videos/audio/docs. Feature-gated (`video`, `audio`, `photo`, `doc` — all default).
- **fs_metadata/** - Library (v0.5.1). Cross-platform file metadata (timestamps, permissions, sizes, symlink detection, Unix/Windows platform-specific fields).

## Build & Run

Requires FFmpeg installed on the system (libavformat, libavcodec). On Windows, use the shared version.

```bash
cargo build                    # Build all crates
cargo run -- --source <path> --destination <path> [--file-type <type>] [--copy] [--dry-run] [--log-saved] [--dimensions] [--verbose]
```

## Testing

```bash
cargo test                     # Run all tests
cargo test -p media_info       # Run tests for a specific crate
```

Tests use example media files in `test-media/` at the workspace root. Test files are referenced via relative paths (`../test-media/`).

### Test Coverage Summary

- **fs_metadata** (12 tests) — file metadata functions, struct methods, permissions, file size, symlink detection, and Unix-specific metadata
- **media_info** (23 tests) — date extraction and dimension reading for each media type (photo, video, audio, doc), format validation, and error cases
- **media_organizer** (36 tests) — type detection for all supported formats, dir string construction, date reading for each media type, dimension filename appending, nonexistent file fallbacks, and edge cases
- **Doc-tests** (4 tests) — AudioInfo, PhotoInfo, and VideoInfo struct examples

### Test Media Files

Located in `test-media/`: JPEG photos, MP4 video, M4A audio, and documents (DOCX, PDF, EPUB, TXT, MD, ODT, RTF, PPTX, XLSX).

## Supported File Types

- **Photos:** JPEG, PNG, HEIF, HEIC, TIFF, AVIF, WebP, DNG, GIF, RAW
- **Videos:** MP4, MOV, AVI
- **Audio:** MP3, WAV, AIFF, M4A, FLAC
- **Documents:** DOC, DOCX, PDF, EPUB, MOBI, TXT, MD, ODT, RTF, PPTX, XLSX

## Changelog

Generated automatically by [git-cliff](https://git-cliff.org/) from commit history. Configuration is in `cliff.toml`.

```bash
git-cliff -o CHANGELOG.md       # Regenerate full changelog
git-cliff --unreleased           # Preview unreleased changes
git-cliff --tag v0.8.0           # Tag unreleased as a version
```

Use [conventional commits](https://www.conventionalcommits.org/) for commit messages so they get categorized properly (e.g. `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).

## Code Conventions

- Functions return `Result<String, String>` for error handling
- Date strings formatted as `YYYY-MM-DD`, organized into `YYYY/MM/DD` directory paths
- Fallback to `"no_date_found"` when date extraction fails
- Feature-gated compilation in media_info via Cargo features
- Environment variables set in main.rs: `DEST_FOLDER`, `FILE_TYPE`, `COPY`, `DRY_RUN`, `LOG_SAVED`, `DIMENSIONS`
- File type detection uses explicit whitelists (case-sensitive with common variations)
- `get_exif_field!` macro used for EXIF field extraction in media_info

## Common Tasks

- To add a new media type, follow the pattern in `media_organizer/src/organizer/make_file_destination/mod.rs` — add a whitelist function, detection function, and wire it into `sort_and_make`.
- Document sorting is wired end-to-end — `media_info` has `read_doc_creation_date` and the organizer routes to it via `make_doc_dir_str` and `is_document`.
- When adding file extensions, always include both lowercase and uppercase variants in the whitelist.
- The `ffmpeg-next` version must match the system FFmpeg version. Check compatibility before upgrading.

## Architecture Notes

- Config is passed via environment variables (`DEST_FOLDER`, `FILE_TYPE`, `COPY`, `DRY_RUN`, `LOG_SAVED`, `DIMENSIONS`) set in `main.rs` using `unsafe env::set_var`. This is intentional — don't refactor to struct-passing without being asked.
- Structured logging via `log` + `structured-logger`. Log files (`same_file.log`, `saved_file.log`) are only created when needed — `same_file.log` is skipped during dry runs, `saved_file.log` only appears with `--log-saved`. Terminal output is suppressed by default; use `--verbose` to enable it. Dry run implicitly enables verbose output.
- Dry run (`--dry-run`) skips directory creation and file operations entirely — no folders or log files are created. Terminal output is automatically enabled.
- Windows-specific file metadata copying is in `set_creation_time_windows.rs`, guarded by `#[cfg(target_os = "windows")]`.
- Each media type module in `media_info` follows the same pattern: a `read_*_creation_date` function and an info struct with `::new(path)`. Photo and video modules also have `read_*_dimensions` functions returning `(u32, u32)`.
- The `get_exif_field!` macro in `media_info` handles EXIF field extraction with fallback to empty strings.

## Linting

- Run `cargo clippy --workspace -- -D warnings` before committing.
- Run `cargo test --workspace` to verify all crates pass.

## Slash Commands

- `/test` — Run workspace tests, diagnose and fix failures
- `/check` — Full pipeline: cargo check + clippy + test
- `/explore <topic>` — Trace and explain how a part of the codebase works
- `/upgrade-deps` — Safely check and upgrade outdated dependencies
- `/debug-media <file>` — Debug why a specific media file isn't sorting correctly
- `/release` — Prepare a versioned release (tests, lint, version bump)

## Key Dependencies

- **ffmpeg-next** (v8.0.0) - Video metadata extraction
- **kamadak-exif** - EXIF/photo metadata
- **id3** - Audio ID3 tag reading
- **clap** - CLI argument parsing
- **chrono** - Date/time handling
- **epub, mobi, pdf** - Document metadata readers
- **zip, quick-xml** - DOCX/ODT metadata extraction (manual XML parsing)
- **log, structured-logger** - Structured JSON logging to file
- **faccess** - File permission checks
