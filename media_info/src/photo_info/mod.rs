use exif::{In, Reader, Tag as ExifTag};
use fs_metadata::file_modified;
use std::fs::File;
use std::path::Path;
use log::warn;
use crate::counter::increment_fallback_counter;

pub mod struct_photo_info;

/// Reads the creation date of the photo from the metadata.
///
/// If it can't read the photo's creation date, it will fall back to the file's modification date.
/// Function indicates that fallback took place by adding '.fallback' at the end of the date string
/// or by returning 'no.date' in case if no date could be determined.
///
/// # Examples
/// ```
/// use media_info::read_photo_creation_date;
///
/// let photo_path = std::path::Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
/// let creation_date = read_photo_creation_date(photo_path).unwrap();
/// assert_eq!(creation_date, "2024-10-22");
/// ```
pub fn read_photo_dimensions(path: &Path) -> Result<(u32, u32), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let file = File::open(path).expect("could not open photo");
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = Reader::new();

    match exifreader.read_from_container(&mut bufreader) {
        Ok(reader) => {
            let width: u32 = reader
                .get_field(ExifTag::PixelXDimension, In::PRIMARY)
                .and_then(|f| f.value.get_uint(0))
                .unwrap_or(0);
            let height: u32 = reader
                .get_field(ExifTag::PixelYDimension, In::PRIMARY)
                .and_then(|f| f.value.get_uint(0))
                .unwrap_or(0);

            if width > 0 && height > 0 {
                Ok((width, height))
            } else {
                Err("Could not read photo dimensions from EXIF".to_string())
            }
        }
        Err(_) => Err(format!("Could not read EXIF data from {:?}", path)),
    }
}

pub fn read_photo_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let file = File::open(path).expect("could not open photo");
    let mut bufreader = std::io::BufReader::new(&file);
    let exif_reader = Reader::new();

    match exif_reader.read_from_container(&mut bufreader) {
        Ok(reader) => match reader.get_field(ExifTag::DateTime, In::PRIMARY) {
            Some(data) => Ok(data.value.display_as(data.tag).to_string().split_whitespace().next().unwrap().to_string()),
            None => fallback_to_file_modified_date(path),
        }
        Err(_) => fallback_to_file_modified_date(path)
    }
}

fn fallback_to_file_modified_date(path: &Path) -> Result<String, String> {
    let file_modified_date_string = file_modified(path)?;
    // indicate that given date is a fallback or that there is no date at all
    let final_file_modified_date = match file_modified_date_string.split_whitespace().next() {
        Some(date) => format!("{date}.fallback"),
        None => "no.date".to_string(),
    };
    increment_fallback_counter();
    warn!("Error reading exif: {:?}, falling back to file modification date {:?}", path, final_file_modified_date);
    Ok(final_file_modified_date.to_string())
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

    #[test]
    fn can_read_photo_dimensions() {
        let path = Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg");
        let (width, height) = read_photo_dimensions(path).unwrap();

        assert_eq!(width, 3022);
        assert_eq!(height, 4030);
    }

    #[test]
    fn photo_dimensions_nonexistent_file() {
        let path = Path::new("nonexistent.jpg");
        assert!(read_photo_dimensions(path).is_err());
    }
}
