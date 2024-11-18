use ffmpeg_next::{self as ffmpeg};
use std::path::Path;

#[derive(Debug)]
pub struct VideoInfo {
    pub creation_date: String,
    pub major_brand: String,
    pub minor_version: String,
    pub compatible_brands: String,
    pub encoder: String,
    pub comment: String,
    pub duration_in_secs: i64,
    pub bit_rate: i64,

    pub location: String,
    pub make: String,
    pub model: String,
    pub software: String,
}

impl VideoInfo {
    pub fn new(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("File does not exist: {:?}", path));
        }

        ffmpeg::init().expect("could not initialize ffmpeg");

        let input_context = ffmpeg::format::input(path).unwrap();
        let video_metadata = input_context.metadata().to_owned();

        macro_rules! get_video_metadata {
            ($tag:expr) => {
                video_metadata
                    .get($tag)
                    .unwrap_or(&"".to_string())
                    .to_string()
            };
        }

        let creation_date = if get_video_metadata!("com.apple.quicktime.creationdate").len() > 0
        {
            get_video_metadata!("com.apple.quicktime.creationdate")
        } else {
            get_video_metadata!("creation_time")
        };
        let location = if get_video_metadata!("com.apple.quicktime.location").len() > 0 {
            get_video_metadata!("com.apple.quicktime.location")
        } else {
            get_video_metadata!("location")
        };
        let make = if get_video_metadata!("com.apple.quicktime.make").len() > 0 {
            get_video_metadata!("com.apple.quicktime.make")
        } else {
            get_video_metadata!("make")
        };
        let model = if get_video_metadata!("com.apple.quicktime.model").len() > 0 {
            get_video_metadata!("com.apple.quicktime.model")
        } else {
            get_video_metadata!("model")
        };
        let software = if get_video_metadata!("com.apple.quicktime.software").len() > 0 {
            get_video_metadata!("com.apple.quicktime.software")
        } else {
            get_video_metadata!("software")
        };

        Ok(VideoInfo {
            creation_date: creation_date,
            major_brand: get_video_metadata!("major_brand"),
            minor_version: get_video_metadata!("minor_version"),
            compatible_brands: get_video_metadata!("compatible_brands"),
            encoder: get_video_metadata!("encoder"),
            comment: get_video_metadata!("comment"),
            duration_in_secs: input_context.duration() / 1_000_000,
            bit_rate: input_context.bit_rate(),
            location: location,
            make: make,
            model: model,
            software: software,
        })
    }
}

#[cfg(test)]
mod video_info_struct {
    use super::*;

    #[test]
    fn can_read_video_info() {
        let raw_path_str = "../test-media/corgi_race.mp4";
        let path = Path::new(raw_path_str);
        let video_info = VideoInfo::new(path).unwrap();

        println!("{:?}", video_info);

        assert_eq!(video_info.creation_date.contains("2024-10-20"), true);
        assert_eq!(video_info.major_brand.contains("isom"), true);
        assert_eq!(video_info.minor_version.contains("512"), true);
        assert_eq!(
            video_info.compatible_brands.contains("isomiso2avc1mp41"),
            true
        );
        assert_eq!(video_info.encoder.contains(""), true);
        assert_eq!(video_info.comment.contains(""), true);
        assert_eq!(video_info.duration_in_secs > 0, true);
    }
}
