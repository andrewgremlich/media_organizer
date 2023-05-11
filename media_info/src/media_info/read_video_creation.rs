use ffmpeg_next as ffmpeg;
use std::path::PathBuf;

pub fn read_video_creation_date(path_str: &str) -> Result<String, String> {
    match ffmpeg::init() {
        Ok(_) => match ffmpeg::format::input(&PathBuf::from(path_str)) {
            Ok(context) => {
                let mut creation_date: String = String::new();

                for (k, v) in context.metadata().iter() {
                    if k == "creation_time" {
                        creation_date.push_str(v)
                    }
                }

                Ok(creation_date)
            }
            Err(_) => Err(String::from("could-not-read-ffmpeg-date")),
        },
        Err(_) => Err(String::from("could-not-initialize-ffmpeg")),
    }
}
