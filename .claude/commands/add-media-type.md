Add support for a new media file type: $ARGUMENTS

Follow the existing pattern:
1. Add metadata extraction in `media_info/src/` — create a new module or extend an existing one (photo_info, video_info, audio_info, doc_info).
2. Add the file extension(s) to the whitelist in `media_organizer/src/organizer/make_file_destination/mod.rs` (include both lowercase and uppercase variants).
3. Add a corresponding `make_*_dir_str` function in `make_dir_str.rs`.
4. Wire the new type into `sort_and_make` in `make_file_destination/mod.rs`.
5. Add a test media sample file to `test-media/` and write tests.
6. Update the supported file types in `CLAUDE.md`, `readme.md`, and `media_organizer/readme.md`.
7. Run `cargo test --workspace` to verify.