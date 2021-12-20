extern crate exif;
extern crate ffmpeg_next as ffmpeg;

use exif::{Exif, In, Reader, Tag};
use std::fs::File;
use std::path::{Path, PathBuf};

enum ExifReaderHandle {
  Exif(Exif),
  Err(String),
}

pub enum VideoReaderHandle {
  VideoDate(String),
  Err(String),
}

pub fn read_photo_creation_date(path_str: &str) -> String {
  let path = Path::new(path_str);

  let file = File::open(path);

  // TODO: if a photo can't be opened don't error. Just skip it.
  let file = match file {
    Ok(file) => file,
    Err(_) => panic!("Could not open photo file: {:?}", path_str),
  };

  let handled_reader: ExifReaderHandle =
    match Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
      Ok(reader) => ExifReaderHandle::Exif(reader),
      Err(_) => ExifReaderHandle::Err(String::from("nodatefound")),
    };

  match handled_reader {
    ExifReaderHandle::Exif(reader) => {
      // TODO: don't panic! just skip it.
      let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
        Some(data) => data.value.display_as(data.tag).to_string(),
        None => panic!("Could not read photo creation date: {:?}", path_str),
      };
      return date_data;
    }
    ExifReaderHandle::Err(message) => message,
  }
}

pub fn read_video_creation_date(path_str: &str) -> VideoReaderHandle {
  // definitely panic if there are videos. or skip videos?
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
