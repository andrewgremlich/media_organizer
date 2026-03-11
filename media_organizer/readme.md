# Media Organizer

Organize photos, videos, and audio into a YYYY/MM/DD folder structure based on creation dates extracted from EXIF, FFmpeg, and ID3 metadata.

## CLI Usage

```txt
USAGE:
    media_organizer [OPTIONS] --source <SOURCE_FOLDER> --destination <DESTINATION_FOLDER>

OPTIONS:
    -c, --copy                              Copy the files instead of moving them.
    -d, --destination <DESTINATION_FOLDER>  The destination folder of sorted media.
    -f, --file-type <FILE_TYPE>             The file type to sort. [default: *]
    -h, --help                              Print help information
    -s, --source <SOURCE_FOLDER>            The absolute path to the source folder of the media to be sorted.
    -V, --version                           Print version information
    -y, --dry-run                           Dry-run with statistics but without actually copying or moving.
    -l, --log-saved                         Log each saved file in a log-file.
      --dimensions                          Append width x height dimensions to image and video filenames.
```

## Example CLI usage

To quickly organize photos in the source folder, run the following command to output them into the default destination directory.

```bash
media_organizer --source test-media --destination sorted_media
```

While developing with cargo, forward the flags.

```bash
cargo run -p media_organizer -- --source ./test-media --destination ./sorted_media
```

## Features

- Organize photos, videos, and audio in a folder structure based off creation dates.

  - Organize a single media file, or a folder containing unorganized media files.

  - Photos organized based off of EXIF creation dates.

    - Whitelisted photo file types. ("tiff", "TIFF", "heif", "HEIF", "HEIC", "heic", "AVIF", "avif", "jpeg", "jpg", "JPEG",
        "JPG", "HEIC", "heic", "PNG", "png", "webp", "WEBP", "dng", "DNG", "gif", "GIF", "raw", "RAW")

  - Video organized based off of FFMPEG creation dates.

    - Whitelisted video file types. ("mp4", "MP4", "mov", "MOV", "avi", "AVI")

  - Audio organized based off of ID3 recorded dates.

    - Whitelisted audio file types. ("mp3", "MP3", "wav", "WAV", "aiff", "AIFF", "m4a", "M4A", "flac", "FLAC")

  - User provides source folder of unorganized images. Will move photos to a default `photos` directory in the directory where the binary was ran. The outputed folders will be the respective creation dates on the media.

- Allow configurable destination folder.

  - Use the `--destination` flag.

- Organize specific file types. Default to any filetype.

- Option to copy files or move files.

- Creation of file paths from file creation date as an organizer fallback.

- Exposed media_info as a new crate.

## Dependencies

For ffmpeg-next package, [follow this guide](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building)

The version of ffmpeg-next should correlate to the version of ffmpeg installed on the system.

If on windows be sure to install the _shared_ version of FFMPEG!
