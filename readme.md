# Photo Organizer

Need an organizer for your photos on the computer? Run this!

## CLI usage

To quickly organize photos in the `test-photos` folder, run the follow command to output them into the default `photos` directory.

```bash
photo_organizer --target test-photos
```

To add a destination folder for all the organized photos, add the `--dest <DEST_FOLDER>` flag to the end.

While developing with cargo, forward the flags.

```bash
cargo run -- --target test-photos
```

## Lib usage

```rust
//Take an image path and return the exif date of the image
pub fn read_exif_date_data(image_path_str: &str) -> String

//organize photos in a folder.
//will require environment variable `DEST_FOLDER`
pub fn make_photo_library(photos_dir_str: &str)
```

## Features

- Organize photos in a folder structure based off of EXIF dates.

  - User provides target folder of images to be organized. Will move photos to a `photos` directory in the directory where the binary was ran. The outputed folders will be the respective dates EXIF data pulled from the photo.

- Allow configurable destination folder.

  - Use the `--dest` flag.

## Future development

- Organize videos.
- Export web assembly binary to use in JS/Node/Electron.
- Index photos.
- Compress image sizes.
