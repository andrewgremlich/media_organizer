extern crate exif;
extern crate ffmpeg_next as ffmpeg;

use exif::{In, Reader, Tag};
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn read_photo_creation_date(path_str: &str) -> String {
  let path = Path::new(path_str);

  let file = File::open(path);
  let file = match file {
    Ok(file) => file,
    Err(_) => panic!("Could not open photo file: {:?}", path_str),
  };

  let reader = Reader::new().read_from_container(&mut std::io::BufReader::new(&file));
  let reader = match reader {
    Ok(reader) => reader,
    Err(_) => panic!("Could not read photo file: {:?}", path_str),
  };

  let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
    Some(data) => data.value.display_as(data.tag).to_string(),
    None => panic!("Could not read photo creation date: {:?}", path_str),
  };

  return date_data;
}

pub fn read_video_creation_date(path_str: &str) -> String {
  match ffmpeg::init() {
    Ok(_) => (),
    Err(_) => panic!("Could not initialize ffmpeg"),
  };

  let path = PathBuf::from(path_str);
  let mut creation_date: String = String::new();

  match ffmpeg::format::input(&path) {
    Ok(context) => {
      for (k, v) in context.metadata().iter() {
        if k == "creation_time" {
          creation_date.push_str(v)
        }
      }
    }

    Err(error) => println!("Could not read ffmpeg data: {}", error),
  }

  return creation_date;
}
