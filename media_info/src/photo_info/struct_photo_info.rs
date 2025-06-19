use exif::Reader;
use std::fs::File;
use std::path::Path;

pub struct PhotoInfo {
    pub make: String,
    pub model: String,
    pub date_time: String,
    pub exposure_time: String,
    pub f_number: String,
    pub exposure_program: String,
    pub photographic_sensitivity: String,
    pub date_time_original: String,
    pub date_time_digitized: String,
    pub exposure_bias_value: String,
    pub focal_length: String,
    pub color_space: String,
    pub pixel_x_dimension: String,
    pub pixel_y_dimension: String,
    pub lens_make: String,
    pub lens_model: String,
    pub gps_latitude_ref: String,
    pub gps_latitude: String,
    pub gps_longitude_ref: String,
    pub gps_longitude: String,
    pub gps_altitude_ref: String,
    pub gps_altitude: String,
    pub gps_timestamp: String,
    pub gps_speed_ref: String,
    pub gps_speed: String,
    pub gps_img_direction_ref: String,
    pub gps_img_direction: String,
    pub gps_dest_bearing_ref: String,
    pub gps_dest_bearing: String,
    pub gps_date_stamp: String,
    pub gps_h_positioning_error: String,
}

/// Creates a new `PhotoInfo` instance by extracting EXIF metadata from the specified file path.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` representing the location of the photo file.
///
/// # Returns
///
/// * `Ok(PhotoInfo)` containing the extracted metadata if successful.
/// * `Err(String)` with an error message if the file does not exist or cannot be read.
///
/// # Errors
///
/// Returns an error if the file at the given path does not exist. Panics if the file cannot be opened
/// or if EXIF data cannot be read from the file.
///
/// # Example
///
/// ```rust
/// use media_info::photo_info::struct_photo_info::PhotoInfo;
/// use std::path::Path;
/// let photo_info = PhotoInfo::new(Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg")).unwrap();
/// ```
impl PhotoInfo {
    pub fn new(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("File does not exist: {:?}", path));
        }

        let file = File::open(path).expect("could not open photo");
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        macro_rules! get_exif_field {
            ($tag:expr) => {
                exif.get_field($tag, exif::In::PRIMARY)
                    .map_or(String::new(), |f| {
                        f.display_value().with_unit(&exif).to_string()
                    })
            };
        }

        Ok(PhotoInfo {
            make: get_exif_field!(exif::Tag::Make),
            model: get_exif_field!(exif::Tag::Model),
            date_time: get_exif_field!(exif::Tag::DateTime),
            exposure_time: get_exif_field!(exif::Tag::ExposureTime),
            f_number: get_exif_field!(exif::Tag::FNumber),
            exposure_program: get_exif_field!(exif::Tag::ExposureProgram),
            photographic_sensitivity: get_exif_field!(exif::Tag::PhotographicSensitivity),
            date_time_original: get_exif_field!(exif::Tag::DateTimeOriginal),
            date_time_digitized: get_exif_field!(exif::Tag::DateTimeDigitized),
            exposure_bias_value: get_exif_field!(exif::Tag::ExposureBiasValue),
            focal_length: get_exif_field!(exif::Tag::FocalLength),
            color_space: get_exif_field!(exif::Tag::ColorSpace),
            pixel_x_dimension: get_exif_field!(exif::Tag::PixelXDimension),
            pixel_y_dimension: get_exif_field!(exif::Tag::PixelYDimension),
            lens_make: get_exif_field!(exif::Tag::LensMake),
            lens_model: get_exif_field!(exif::Tag::LensModel),
            gps_latitude_ref: get_exif_field!(exif::Tag::GPSLatitudeRef),
            gps_latitude: get_exif_field!(exif::Tag::GPSLatitude),
            gps_longitude_ref: get_exif_field!(exif::Tag::GPSLongitudeRef),
            gps_longitude: get_exif_field!(exif::Tag::GPSLongitude),
            gps_altitude_ref: get_exif_field!(exif::Tag::GPSAltitudeRef),
            gps_altitude: get_exif_field!(exif::Tag::GPSAltitude),
            gps_timestamp: get_exif_field!(exif::Tag::GPSTimeStamp),
            gps_speed_ref: get_exif_field!(exif::Tag::GPSSpeedRef),
            gps_speed: get_exif_field!(exif::Tag::GPSSpeed),
            gps_img_direction_ref: get_exif_field!(exif::Tag::GPSImgDirectionRef),
            gps_img_direction: get_exif_field!(exif::Tag::GPSImgDirection),
            gps_dest_bearing_ref: get_exif_field!(exif::Tag::GPSDestBearingRef),
            gps_dest_bearing: get_exif_field!(exif::Tag::GPSDestBearing),
            gps_date_stamp: get_exif_field!(exif::Tag::GPSDateStamp),
            gps_h_positioning_error: get_exif_field!(exif::Tag::GPSHPositioningError),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_photo_info() {
        let raw_path_str = "../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg";
        let path = Path::new(raw_path_str);
        let photo_info = PhotoInfo::new(path).unwrap();

        assert_eq!(photo_info.date_time.contains("2024-10-22"), true);
        assert_eq!(photo_info.make.contains("Apple"), true);
        assert_eq!(photo_info.model.contains("iPhone 15"), true);
        assert_eq!(photo_info.exposure_time.contains("1/60"), true);
        assert_eq!(photo_info.f_number.contains("f/2.2"), true);
        assert_eq!(photo_info.exposure_program.contains("normal"), true);
        assert_eq!(photo_info.photographic_sensitivity.contains("320"), true);
        assert_eq!(photo_info.date_time_original.contains("2024-10-22"), true);
        assert_eq!(photo_info.date_time_digitized.contains("2024-10-22"), true);
        assert_eq!(photo_info.exposure_bias_value.contains("0"), true);
        assert_eq!(photo_info.focal_length.contains("2.22"), true);
        assert_eq!(photo_info.color_space.contains("sRGB"), true);
        assert_eq!(photo_info.pixel_x_dimension.contains("3022"), true);
        assert_eq!(photo_info.pixel_y_dimension.contains("4030"), true);
        assert_eq!(photo_info.lens_make.contains("Apple"), true);
        assert_eq!(photo_info.lens_model.contains("iPhone 15"), true);
        assert_eq!(photo_info.gps_latitude_ref.contains("N"), true);
        assert_eq!(photo_info.gps_latitude.contains("40 deg 42 min 52.24 sec N"), true);
        assert_eq!(photo_info.gps_longitude_ref.contains("W"), true);
        assert_eq!(photo_info.gps_longitude.contains("73 deg 59 min 55.74 sec W"), true);
        assert_eq!(photo_info.gps_altitude_ref.contains("above sea level"), true);
        assert_eq!(photo_info.gps_altitude.contains("9.9"), true);
        assert_eq!(photo_info.gps_timestamp.contains("19:09:16"), true);
        assert_eq!(photo_info.gps_speed_ref.contains("km/h"), true);
        assert_eq!(photo_info.gps_speed.contains("0 km/h"), true);
        assert_eq!(photo_info.gps_img_direction_ref.contains("true direction"), true);
        assert_eq!(photo_info.gps_img_direction.contains("72.17"), true);
        assert_eq!(photo_info.gps_dest_bearing_ref.contains("true direction"), true);
        assert_eq!(photo_info.gps_dest_bearing.contains("72.17"), true);
        assert_eq!(photo_info.gps_date_stamp.contains("2024-10-22"), true);
        assert_eq!(photo_info.gps_h_positioning_error.contains("20.83"), true);
    }
}
