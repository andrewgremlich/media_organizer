# Media Organizer

Need an organizer for your media content on the computer? Run this!

## CLI Usage

This is the output from `--help`.

```txt
media_organizer 0.5.0
Andrew Gremlich <andrew.gremlich@gmail.com>
Organize media into one folder with a date-centric folder structure.

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
cargo run -- --target test-media --destination sorted_media
```

## Features

- Organize photos and videos in a folder structure based off creation dates.

  - Organize a single media file, or a folder containing unorganized media files.

  - Photos organized based off of EXIF creation dates.

    - Whitelisted photo file types. ("jpeg", "jpg", "JPEG", "JPG", "HEIC", "heic", "PNG", "png")

  - Video organized based off of FFMPEG creation dates.

    - Whitelisted video file types. ("mp4", "MP4", "mov", "MOV")

  - User provides target folder of unorganized images. Will move photos to a default `photos` directory in the directory where the binary was ran. The outputed folders will be the respective creation dates on the media.

- Allow configurable destination folder.

  - Use the `--dest` flag.

- Organize specific file types. Default to any filetype.

- Option to copy files or move files.

- Exposed creation date reading functions.

## Dependencies

- For ffmpeg-next package
  - C++ build tools (or equivalent like build-essentials)
  - FFMPEG

## Future development

- Write tests for bin and lib?
- Export web assembly binary to use in JS/Node/Electron.

  - Watch out for `DEST_FOLDER` environment variable.
  - OR! Just use Tauri!?

- Contribute to Kamadak-exif
- [Organize audio files?](https://github.com/pdeljanov/Symphonia)
- Option to index photos.
- Option to compress image sizes.
