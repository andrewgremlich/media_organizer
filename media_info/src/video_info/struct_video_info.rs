use ffmpeg_next as ffmpeg;
use fs_metadata::file_created;
use std::path::Path;

pub struct VideoInfo {
    pub date_time: String,
}

impl VideoInfo {
    pub fn new(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Err(format!("File does not exist: {:?}", path));
        }

        Ok(VideoInfo {
            date_time: "hello world".to_string(),
        })
    }
}
