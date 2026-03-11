use crate::counter::increment_fallback_counter;
use ffmpeg_next as ffmpeg;
use fs_metadata::file_modified;
use log::warn;
use std::path::Path;

mod struct_video_info;

pub use struct_video_info::VideoInfo;

/// Reads the creation date of the video from the metadata.
///
/// If it can't read the video's creation date, it will fall back to the file's modification date.
/// Function indicates that fallback took place by adding '.fallback' at the end of the date string
/// or by returning 'no.date' in case if no date could be determined.
/// # Examples
/// ```
/// use media_info::read_video_creation_date;
///
/// let video_path = std::path::Path::new("../test-media/corgi_race.mp4");
/// let creation_date = read_video_creation_date(video_path).unwrap();
/// assert_eq!(creation_date, "2024-10-20");
/// ```
pub fn read_video_dimensions(path: &Path) -> Result<(u32, u32), String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(path) {
        Ok(context) => {
            for stream in context.streams() {
                let codec_params = stream.parameters();
                if let Ok(decoder) = ffmpeg::codec::context::Context::from_parameters(codec_params)
                    && let Ok(video) = decoder.decoder().video()
                {
                    return Ok((video.width(), video.height()));
                }
            }
            Err("No video stream found".to_string())
        }
        Err(e) => Err(format!("Error reading video: {}", e)),
    }
}

pub fn read_video_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(path) {
        Ok(context) => {
            let video_metadata = context.metadata().to_owned();
            let creation_date: String = video_metadata
                .get("creation_time")
                .unwrap_or("")
                .to_string();

            if creation_date.len() == 0 {
                return fallback_to_file_modified_date(path);
            }

            if creation_date.split('T').count() > 1 {
                Ok(creation_date.split('T').next().unwrap().to_string())
            } else {
                Ok(creation_date.split_whitespace().next().unwrap().to_string())
            }
        }
        Err(_) => fallback_to_file_modified_date(path),
    }
}

fn fallback_to_file_modified_date(path_str: &Path) -> Result<String, String> {
    let file_modified_date_string = file_modified(path_str)?;
    let final_file_modified_date = if file_modified_date_string.split('T').count() > 1 {
        match file_modified_date_string.split('T').next() {
            Some(date) => format!("{date}.fallback"),
            None => "no.date".to_string(),
        }
    } else {
        match file_modified_date_string.split_whitespace().next() {
            Some(date) => format!("{date}.fallback"),
            None => "no.date".to_string(),
        }
    };
    increment_fallback_counter();
    warn!(
        "Error reading exif: {:?}, falling back to file modification date {:?}",
        path_str, final_file_modified_date
    );
    Ok(final_file_modified_date)
}

#[cfg(test)]
pub mod video_date_read {
    use super::*;

    #[test]
    fn can_read_video_creation_date() {
        let raw_path_str = "../test-media/corgi_race.mp4";
        let path = Path::new(raw_path_str);
        let creation_date = read_video_creation_date(path);

        assert_eq!(creation_date.unwrap().contains("2024-10-20"), true);
    }

    #[test]
    fn can_read_video_dimensions() {
        let path = Path::new("../test-media/corgi_race.mp4");
        let (width, height) = read_video_dimensions(path).unwrap();

        assert!(width > 0);
        assert!(height > 0);
    }

    #[test]
    fn video_dimensions_nonexistent_file() {
        let path = Path::new("nonexistent.mp4");
        assert!(read_video_dimensions(path).is_err());
    }
}
