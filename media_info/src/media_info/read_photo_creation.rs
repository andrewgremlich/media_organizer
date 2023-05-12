use exif::{Exif, In, Reader, Tag};
use file_metadata::file_created;
use std::fs::File;
use std::path::Path;

enum ExifReader {
    Exif(Exif),
    Err(String),
}

fn match_read_exif(file: File) -> ExifReader {
    match Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
        Ok(reader) => ExifReader::Exif(reader),
        Err(_) => ExifReader::Err(String::from("noexiffound")),
    }
}

pub fn read_photo_creation_date(path_str: &str) -> Result<String, String> {
    let file = File::open(Path::new(path_str)).expect("could not open photo file");

    match match_read_exif(file) {
        ExifReader::Exif(reader) => {
            let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
                Some(data) => data.value.display_as(data.tag).to_string(),
                None => return Err(String::from("couldnotreadphotocreationdate")),
            };
            return Ok(date_data);
        }
        ExifReader::Err(_) => {
            println!("Error reading exif: {:?}", path_str);
            println!("Falling back to file creation date");

            let formatted_date = file_created(path_str);
            return Ok(formatted_date);
        }
    }
}
