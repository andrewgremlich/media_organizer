extern crate exif;
extern crate ffmpeg_next as ffmpeg;

use exif::{Exif, In, Reader, Tag};
use std::fs::File;
use std::path::{Path, PathBuf};

enum ImageReaderHandle {
  ImageData(File),
  Err(String),
}

enum ExifReader {
  Exif(Exif),
  Err(String),
}

pub enum PhotoCreationDateReader {
  CreationDate(String),
  Err(String),
}

pub enum VideoReaderHandle {
  VideoDate(String),
  Err(String),
}

fn match_open_photo_file(path_str: &str) -> ImageReaderHandle {
  match File::open(Path::new(path_str)) {
    Ok(file) => ImageReaderHandle::ImageData(file),
    Err(_) => {
      let mut message: String = String::new();

      message.push_str("Could not open photo file: ");
      message.push_str(path_str);

      ImageReaderHandle::Err(message)
    }
  }
}

fn match_read_exif(file: File) -> ExifReader {
  match Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
    Ok(reader) => ExifReader::Exif(reader),
    Err(_) => ExifReader::Err(String::from("nodatefound")),
  }
}

pub fn read_photo_creation_date(path_str: &str) -> PhotoCreationDateReader {
  let file: ImageReaderHandle = match_open_photo_file(path_str);

  match file {
    ImageReaderHandle::ImageData(file) => match match_read_exif(file) {
      ExifReader::Exif(reader) => {
        let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
          Some(data) => data.value.display_as(data.tag).to_string(),
          None => {
            let mut error_message: String = String::new();

            error_message.push_str("Could not read photo creation date: ");
            error_message.push_str(path_str);

            return PhotoCreationDateReader::Err(error_message);
          }
        };
        return PhotoCreationDateReader::CreationDate(date_data);
      }
      ExifReader::Err(message) => PhotoCreationDateReader::Err(message),
    },
    ImageReaderHandle::Err(message) => PhotoCreationDateReader::Err(String::from(message)),
  }
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
