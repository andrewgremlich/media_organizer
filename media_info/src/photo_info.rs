use exif::{In, Reader, Tag as ExifTag};
use fs_metadata::file_created;
use std::fs::File;
use std::path::Path;

pub fn read_photo_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let file = File::open(path).expect("could not open photo");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();

    match exifreader.read_from_container(&mut bufreader) {
        Ok(reader) => {
            let date_data: String = match reader.get_field(ExifTag::DateTime, In::PRIMARY) {
                Some(data) => data.value.display_as(data.tag).to_string(),
                None => todo!(),
            };
            Ok(date_data)
        }
        Err(_) => {
            println!("Error reading exif: {:?}", path);
            println!("Falling back to file creation date");
            // TODO: edit exif date with file creation date?

            let formatted_date = file_created(path).unwrap();
            Err(formatted_date)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        let raw_path_str = "../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg";
        let path = Path::new(raw_path_str);
        let creation_date = read_photo_creation_date(path);

        assert_eq!(creation_date.unwrap().contains("2024-10-22"), true);
    }
}
