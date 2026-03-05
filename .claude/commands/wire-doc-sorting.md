Wire document sorting into the media_organizer CLI. The `media_info` crate already has `read_doc_creation_date` and document metadata support.

Steps:
1. Add a document file type whitelist in `media_organizer/src/organizer/make_file_destination/mod.rs` following the same pattern as photos/videos/audio. Supported extensions: docx, pdf, epub, mobi, txt, md, odt, rtf (with uppercase variants).
2. Add an `is_doc` detection function and wire it into `sort_and_make`.
3. Add a `make_doc_dir_str` function in `make_dir_str.rs` that calls `read_doc_creation_date`.
4. Add tests for document sorting.
5. Update the CLI help text and README to reflect document support.
6. Run `cargo test --workspace` to verify everything works.