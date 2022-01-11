extern crate media_organizer_lib;

// use media_organizer_lib::media_info::read_audio_creation::read_audio_creation_date;
use media_organizer_lib::media_info::read_photo_creation::{
  read_photo_creation_date, PhotoCreationDateReader,
};
use media_organizer_lib::media_info::read_video_creation::{
  read_video_creation_date, VideoReaderHandle,
};
use std::env;

enum DirString<'a> {
  DateBreakdown(Option<&'a str>),
  RegularStr(String),
}

fn finally_make_date_str(appender: String) -> String {
  let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
  let mut regular_date_folder: String = String::new();

  regular_date_folder.push_str("./");
  regular_date_folder.push_str(&dest_folder);
  regular_date_folder.push_str("/");
  regular_date_folder.push_str(&appender);

  regular_date_folder
}

fn make_dir_string(date_time: DirString) -> String {
  match date_time {
    DirString::DateBreakdown(breakdown) => {
      if let Some(breakdown) = breakdown {
        let replace_date_hyphens = str::replace(breakdown, "-", "/");
        finally_make_date_str(replace_date_hyphens)
      } else {
        finally_make_date_str(String::from("nodatesexist"))
      }
    }
    DirString::RegularStr(reg_string) => finally_make_date_str(String::from(reg_string)),
  }
}

pub fn make_photo_dir_str(dir_str: &str) -> String {
  match read_photo_creation_date(dir_str) {
    PhotoCreationDateReader::CreationDate(date_of_photo) => make_dir_string(
      DirString::DateBreakdown(date_of_photo.split_whitespace().next()),
    ),
    PhotoCreationDateReader::Err(err) => make_dir_string(DirString::RegularStr(String::from(err))),
  }
}

pub fn make_video_dir_str(dir_str: &str) -> String {
  let date_of_video: VideoReaderHandle = read_video_creation_date(dir_str);

  match date_of_video {
    VideoReaderHandle::VideoDate(date) => {
      make_dir_string(DirString::DateBreakdown(date.split("T").next()))
    }
    VideoReaderHandle::Err(err) => make_dir_string(DirString::RegularStr(String::from(err))),
  }
}

// pub fn make_audio_dir_str(dir_str: &str) {
//   read_audio_creation_date(dir_str);
// }
