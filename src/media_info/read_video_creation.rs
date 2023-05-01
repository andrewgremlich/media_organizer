use ffmpeg_next as ffmpeg;
use std::path::PathBuf;

pub enum VideoReaderHandle {
  VideoDate(String),
  Err(String),
}

pub fn read_video_creation_date(path_str: &str) -> VideoReaderHandle {
  let ffmpeg_init = match ffmpeg::init() {
    Ok(_) => VideoReaderHandle::VideoDate(String::from("success-ffmpeg-init")),
    Err(_) => VideoReaderHandle::Err(String::from("could-not-initialize-ffmpeg")),
  };

  match ffmpeg_init {
    VideoReaderHandle::VideoDate(_) => {
      let date_info = match ffmpeg::format::input(&PathBuf::from(path_str)) {
        Ok(context) => {
          let mut creation_date: String = String::new();
          for (k, v) in context.metadata().iter() {
            if k == "creation_time" {
              creation_date.push_str(v)
            }
          }
          return VideoReaderHandle::VideoDate(creation_date);
        }
        Err(_) => VideoReaderHandle::Err(String::from("could-not-read-ffmpeg-date")),
      };

      return date_info;
    }
    VideoReaderHandle::Err(message) => VideoReaderHandle::Err(String::from(message)),
  }
}
