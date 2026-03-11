# Contributing to Media Organizer

Thanks for your interest in contributing! Here's how to get started.

## Prerequisites

- Rust (latest stable)
- FFmpeg installed on your system (libavformat, libavcodec)
  - macOS: `brew install ffmpeg`
  - Ubuntu/Debian: `apt install libavformat-dev libavcodec-dev`
  - Windows: use the shared FFmpeg build

## Getting Started

1. Fork and clone the repository
2. Run `cargo build` to verify your setup
3. Run `cargo test --workspace` to make sure everything passes

## Development Workflow

1. Create a branch from `master` for your changes
2. Make your changes
3. Run the full check pipeline before submitting:
   ```bash
   cargo check --workspace
   cargo clippy --workspace -- -D warnings
   cargo test --workspace
   ```
4. Open a pull request against `master`

## Adding a New File Type

1. Add lowercase and uppercase extension variants to the whitelist in `media_organizer/src/organizer/make_file_destination/mod.rs`
2. If the format requires metadata extraction, add a reader function in `media_info/src/` following the existing module pattern (`read_*_creation_date`)
3. Wire the new reader into `media_info/src/doc_info/mod.rs` (for documents) or the appropriate module
4. Add a test media file to `test-media/`
5. Add tests for both detection and date extraction

## Code Style

- Follow existing patterns in the codebase
- Functions return `Result<String, String>` for error handling
- Date strings use `YYYY-MM-DD` format
- Fall back to `"no_date_found"` when metadata extraction fails
- Include both lowercase and uppercase extension variants in whitelists

## Testing

- All new features should include tests
- Test media files go in the `test-media/` directory at the workspace root
- Reference test files with relative paths (`../test-media/`)
- Run `cargo test --workspace` to verify all crates pass

## Reporting Issues

Open an issue on GitHub with:
- Steps to reproduce the problem
- Expected vs actual behavior
- The media file type involved (if applicable)
- Your OS and FFmpeg version
