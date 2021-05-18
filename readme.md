# Photo Organizer

Need an organizer for your photos on the computer? Run this!

## Usage

```bash
photo_organizer --target test-photos
```

While developing with cargo, forward the flags.

```bash
cargo run -- --target test-photos
```

## Features

Organize photos in a folder structure based off of EXIF dates.

- User provides target folder of images to be organized. Will move photos to a `photos` directory in the directory where the binary was ran. The outputed folders will be the respective dates EXIF data pulled from the photo.

## Future development

Allow configurable destination folder.
Export web assembly binary to use in JS/Node/Electron.
Index photos in a file structure.
Compress image sizes.
