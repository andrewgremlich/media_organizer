use exif::{Exif, In, Reader, Tag};
use std::fs::File;
use std::path::Path;

enum ExifReader {
    Exif(Exif),
    Err(String),
}

enum ImageReaderHandle {
    ImageData(File),
    Err(String),
}

fn match_open_photo_file(path_str: &str) -> ImageReaderHandle {
    match File::open(Path::new(path_str)) {
        Ok(file) => ImageReaderHandle::ImageData(file),
        Err(_) => ImageReaderHandle::Err(String::from("couldnotopenphotofile")),
    }
}

fn match_read_exif(file: File) -> ExifReader {
    match Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
        Ok(reader) => ExifReader::Exif(reader),
        Err(_) => ExifReader::Err(String::from("nodatefound")),
    }
}

pub fn read_photo_creation_date(path_str: &str) -> Result<String, String> {
    let file: ImageReaderHandle = match_open_photo_file(path_str);

    match file {
        ImageReaderHandle::ImageData(file) => match match_read_exif(file) {
            ExifReader::Exif(reader) => {
                let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
                    Some(data) => data.value.display_as(data.tag).to_string(),
                    None => return Err(String::from("couldnotreadphotocreationdate")),
                };
                return Ok(date_data);
            }
            ExifReader::Err(message) => Err(message),
        },
        ImageReaderHandle::Err(message) => Err(message),
    }
}
