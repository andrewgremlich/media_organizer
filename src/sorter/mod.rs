extern crate glob;
extern crate mkdirp;

use std::env;
mod media_info;
mod utils;

use self::glob::glob;
use media_info::date_data::{
    read_photo_creation_date, read_video_creation_date, PhotoCreationDateReader, VideoReaderHandle,
};
use mkdirp::mkdirp;
use utils::{is_photo, is_video, make_dir_string, move_image, DirString};

fn make_photo_dir_str(dir_str: &str) -> String {
    match read_photo_creation_date(dir_str) {
        PhotoCreationDateReader::CreationDate(date_of_photo) => make_dir_string(
            DirString::DateBreakdown(date_of_photo.split_whitespace().next()),
        ),
        PhotoCreationDateReader::Err(err) => {
            make_dir_string(DirString::RegularStr(String::from(err)))
        }
    }
}

fn make_video_dir_str(dir_str: &str) -> String {
    let date_of_video: VideoReaderHandle = read_video_creation_date(dir_str);

    match date_of_video {
        VideoReaderHandle::VideoDate(date) => {
            make_dir_string(DirString::DateBreakdown(date.split("T").next()))
        }
        VideoReaderHandle::Err(err) => make_dir_string(DirString::RegularStr(String::from(err))),
    }
}

fn handle_path(path: &str) {
    let path_str: &str = path;
    let mut date_data: String = String::new();

    if is_photo(path_str) {
        date_data.push_str(&make_photo_dir_str(path_str));
    }
    if is_video(path_str) {
        date_data.push_str(&make_video_dir_str(path_str));
    }

    mkdirp(&date_data).expect("Could not create directory");
    move_image(path_str, &date_data);
}

pub fn sort_file(file_path: &str) {
    handle_path(file_path);
}

pub fn sort_dir(dir_str: &str) {
    let mut glob_path: String = String::new();
    let file_type = env::var("FILE_TYPE").expect("FILE_TYPE not set");

    glob_path.push_str(dir_str);
    glob_path.push_str("/**/*.");
    glob_path.push_str(&file_type);

    for entry in glob(&glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match path.to_str() {
                Some(path_str) => {
                    handle_path(path_str);
                }
                None => println!("Failed to convert path to string"),
            },
            // TODO: I don't think this should fail anymore.
            Err(e) => panic!("Glop path entry failed: {:?}", e),
        }
    }
}
