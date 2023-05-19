# Media Organizer

Organize videos, photos, and audio into a folder date-hierarchy format.

[Crate link to Media Organizer](https://crates.io/crates/media_organizer)

## CLI Usage

```txt
USAGE:
    media_organizer [OPTIONS] --target <TARGET_MEDIA> --destination <DESTINATION_PATH>

OPTIONS:
    -c, --copy                              Copy the files instead of moving them.
    -d, --destination <DESTINATION_PATH>    The destination path of sorted media.
    -f, --file-type <FILE_TYPE>             The file type to sort. [default: *]
    -h, --help                              Print help information
    -t, --target <TARGET_MEDIA>             The target media to sort.
    -V, --version                           Print version information
```

## Example CLI usage

To quickly organize photos in the target folder, run the following command to output them into the default destination directory.

```bash
media_organizer --target test-media --destination sorted_media
```

While developing with cargo, forward the flags.

```bash
cargo run -p media_organizer -- --target ./media_organizer/photos --destination ./media_organizer/media
```

## Features

- Organize photos, videos, and audio in a folder structure based off creation dates.

  - Organize a single media file, or a folder containing unorganized media files.

  - Photos organized based off of EXIF creation dates.

    - Whitelisted photo file types. ("tiff", "TIFF", "heif", "HEIF", "HEIC", "heic", "AVIF", "avif", "jpeg", "jpg", "JPEG",
        "JPG", "HEIC", "heic", "PNG", "png", "webp", "WEBP")

  - Video organized based off of FFMPEG creation dates.

    - Whitelisted video file types. ("mp4", "MP4", "mov", "MOV")

  - Audio organized based off of ID3 recorded dates.

    - Whitelisted audio file types. ("mp3", "MP3", "wav", "WAV", "aiff", "AIFF")

  - User provides target folder of unorganized images. Will move photos to a default `photos` directory in the directory where the binary was ran. The outputed folders will be the respective creation dates on the media.

- Allow configurable destination folder.

  - Use the `--dest` flag.

- Organize specific file types. Default to any filetype.

- Option to copy files or move files.

- Creation of file paths from file creation date as an organizer fallback.

- Exposed media_info as a new crate.

## Dependencies

For ffmpeg-next package, [follow this guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building)

## Roadmap

- [Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments)

- tests

- option to compress media with copy feature

- individually modify file metadata and tag metadata (exif, ffmpeg, id3)
