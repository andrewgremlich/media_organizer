# Media Organizer

Need an organizer for your photos and videos on the computer? Run this!

## Dependencies

- C++ build tools (or equivalent like build-essentials) (for ffmpeg-next package)
- FFMPEG

## CLI usage

To quickly organize photos in the target folder, run the following command to output them into the default destination directory.

```bash
photo_organizer --target test-media
```

To add a destination folder for all the organized photos, add the `--dest <DEST_FOLDER>` flag to the end.

While developing with cargo, forward the flags.

```bash
cargo run -- --target test-photos
```

Supports organizing JPG (jpeg, jpg, JPEG), HEIC (heic), and PNG (png) image files and their variants.

## Lib usage

```rust
// Take an image path and return the exif date of the image
pub fn read_photo_creation_date(path_str: &str) -> String

// Take a video path and return the FFMPEG creatioon date.
pub fn read_video_creation_date(path_str: &str) -> String

// Sort all the supported media files in a directory.
// Note: will default folder "photos"
pub fn sorter(dir_str: &str)
```

## Features

- Organize photos in a folder structure based off of EXIF dates.

  - User provides target folder of images to be organized. Will move photos to a `photos` directory in the directory where the binary was ran. The outputed folders will be the respective dates EXIF data pulled from the photo.

- Allow configurable destination folder.

  - Use the `--dest` flag.

- Organize Video files in a folder structure based off of FFMPEG creation date.

## Future development

- Make a multi-media organizer binary.
- Organize audio files? https://github.com/pdeljanov/Symphonia
- Export web assembly binary to use in JS/Node/Electron.

  - Watch out for `DEST_FOLDER` environment variable.

- Option to index photos.
- Option to compress image sizes.
