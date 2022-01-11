# Media Organizer

Need an organizer for your media content on the computer? Run this!

## CLI Usage

This is the output from `--help`.

```txt
Photo Organizer 0.3.6
Andrew Gremlich
Organize media in one folder into date-centric folder structure.

USAGE:
    media_organizer [FLAGS] [OPTIONS] --target <TARGET_MEDIA>

FLAGS:
    -c, --copy       Copy files instead of moving them.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dest <DEST_FOLDER>       Name of the folder in the current directory where organized media will be put.
                                   [default: media]
    -f, --filetype <FILE_TYPE>     File type to sort. Defaults to all file types. [default: *]
    -t, --target <TARGET_MEDIA>    Target media. Can be a folder with unorganized media or a single file.
```

## Example CLI usage

To quickly organize photos in the target folder, run the following command to output them into the default destination directory.

```bash
media_organizer --target test-media
```

To add a destination folder for all the organized photos, add the `--dest <DEST_FOLDER>` flag to the end.

While developing with cargo, forward the flags.

```bash
cargo run -- --target test-photos
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

- C++ build tools (or equivalent like build-essentials) (for ffmpeg-next package)
- FFMPEG

## Future development

- [Organize audio files?](https://github.com/pdeljanov/Symphonia)
- Contribute to Kamadak-exif?
- Give option to change permission of parent folder?

  - This will require mapping out the directory structure and checking file permissions.

    - Make structs mapping out the file directory?

- Export web assembly binary to use in JS/Node/Electron.

  - Watch out for `DEST_FOLDER` environment variable.
  - OR! Just use Tauri!?

- Option to index photos.
- Option to compress image sizes.
