use exif::{In, Reader, Tag as ExifTag};
use fs_metadata::file_created;
use std::fs::File;
use std::path::Path;

/// Reads the creation date of the photo from the metadata.
/// If it can't read the photo's creation date, it will fall back to the file's creation date.
///
/// # Examples
/// ```
/// use media_info::read_photo_creation_date;
///
/// let photo_path = "tests/test_data/photo.jpg";
/// let creation_date = read_photo_creation_date(photo_path).unwrap();
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn read_photo_creation_date(path_str: &str) -> String {
    let file = File::open(Path::new(path_str)).expect("could not open photo");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();

    match exifreader.read_from_container(&mut bufreader) {
        Ok(reader) => {
            let date_data: String = match reader.get_field(ExifTag::DateTime, In::PRIMARY) {
                Some(data) => data.value.display_as(data.tag).to_string(),
                None => todo!(),
            };
            date_data
        }
        Err(_) => {
            println!("Error reading exif: {:?}", path_str);
            println!("Falling back to file creation date");
            // TODO: edit exif date with file creation date?

            let formatted_date = file_created(path_str).unwrap();
            formatted_date
        }
    }
}
