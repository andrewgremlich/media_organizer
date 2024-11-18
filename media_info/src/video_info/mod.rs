use ffmpeg_next as ffmpeg;
use fs_metadata::file_created;
use std::path::Path;

mod struct_video_info;

pub use struct_video_info::VideoInfo;

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
                .unwrap_or(&"".to_string())
                .to_string();

            Ok(creation_date)
        }
        Err(_) => {
            println!("Error reading video creation date: {:?}", path);
            println!("Falling back to file creation date");
            // TODO: edit creation date with file creation date?

            let formatted_date = file_created(path).unwrap();

            Ok(formatted_date)
        }
    }
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
}
