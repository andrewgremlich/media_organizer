use ffmpeg_next as ffmpeg;
use fs_metadata::file_created;
use std::path::PathBuf;

/// Reads the creation date of the video from the metadata.
/// If it can't read the video's creation date, it will fall back to the file's creation date.
///
/// # Examples
/// ```
/// use media_info::read_video_creation_date;
///
/// let video_path = "tests/test_data/video.mp4";
/// let creation_date = read_video_creation_date(video_path).unwrap();
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn read_video_creation_date(path_str: &str) -> String {
    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(&PathBuf::from(path_str)) {
        Ok(context) => {
            let mut creation_date: String = String::new();

            for (name, value) in context.metadata().iter() {
                if name == "creation_time" {
                    creation_date.push_str(value)
                }
            }

            creation_date
        }
        Err(_) => {
            println!("Error reading video creation date: {:?}", path_str);
            println!("Falling back to file creation date");
            // TODO: edit creation date with file creation date?

            let formatted_date = file_created(path_str).unwrap();
            formatted_date
        }
    }
}
