Debug why a media file is not being sorted correctly: $ARGUMENTS

1. Determine the file extension and which media type category it should fall into (photo, video, audio, doc).
2. Check if the extension is in the whitelist in `media_organizer/src/organizer/make_file_destination/mod.rs`.
3. If whitelisted, trace the metadata extraction path in `media_info/src/` for that media type.
4. Run the relevant `read_*_creation_date` function logic mentally against the file to identify where it fails.
5. Check if the date parsing in `make_dir_str.rs` handles the date format returned.
6. Suggest a fix with code changes if an issue is found.