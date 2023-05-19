use ffmpeg_next as ffmpeg;
use mo_file_metadata::file_created;
use std::path::PathBuf;

pub fn read_video_creation_date(path_str: &str) -> Result<String, String> {
    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(&PathBuf::from(path_str)) {
        Ok(context) => {
            let mut creation_date: String = String::new();

            for (name, value) in context.metadata().iter() {
                if name == "creation_time" {
                    creation_date.push_str(value)
                }
            }

            Ok(creation_date)
        }
        Err(_) => {
            println!("Error reading video creation date: {:?}", path_str);
            println!("Falling back to file creation date");

            let formatted_date = file_created(path_str);
            return Ok(formatted_date);
        }
    }
}
