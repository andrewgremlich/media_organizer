# Photo Organizer

Need an organizer for your photos on the computer? Run this!

## CLI usage

```bash
photo_organizer --target test-photos
```

While developing with cargo, forward the flags.

```bash
cargo run -- --target test-photos
```

## Lib usage

```rust
//Take an image path and return the exif date of the image
pub fn read_exif_date_data(image_path_str: &str) -> String

//organize photos in a folder.
pub fn make_photo_library(photos_dir_str: &str)
```

## Features

Organize photos in a folder structure based off of EXIF dates.

- User provides target folder of images to be organized. Will move photos to a `photos` directory in the directory where the binary was ran. The outputed folders will be the respective dates EXIF data pulled from the photo.

## Future development

- Allow configurable destination folder.
- Organize videos.
- Export web assembly binary to use in JS/Node/Electron.
- Index photos.
- Compress image sizes.
