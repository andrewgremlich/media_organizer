extern crate glob;
extern crate mkdirp;

use std::env;
mod media_info;
mod utils;

use self::glob::glob;
use media_info::date_data::{read_photo_creation_date, read_video_creation_date};
use mkdirp::mkdirp;
use utils::{is_photo, is_video, make_dir_string, move_image};

fn make_photo_dir_str(dir_str: &str) -> String {
    let date_of_photo: String = read_photo_creation_date(dir_str);
    let photo_dir_str: String = make_dir_string(date_of_photo.split_whitespace().next());
    return photo_dir_str;
}

fn make_video_dir_str(dir_str: &str) -> String {
    let date_of_video: String = read_video_creation_date(dir_str);
    let video_dir_str: String = make_dir_string(date_of_video.split("T").next());
    return video_dir_str;
}

fn handle_path(path: &str) {
    let path_str: &str = path;
    let mut date_data: String = String::new();

    if is_photo(path_str) {
        let photo_dir_str: String = make_photo_dir_str(path_str);
        date_data.push_str(&photo_dir_str);
    }
    if is_video(path_str) {
        let video_dir_str: String = make_video_dir_str(path_str);
        date_data.push_str(&video_dir_str);
    }

    mkdirp(&date_data).unwrap();
    move_image(path_str, &date_data);
}

pub fn sort_file(file_path: &str) {
    handle_path(file_path);
}

pub fn sort_dir(dir_str: &str) {
    let mut glob_path: String = String::new();
    let file_type: String = env::var("FILE_TYPE").unwrap();

    glob_path.push_str(dir_str);
    glob_path.push_str("/**/*.");
    glob_path.push_str(&file_type);

    for entry in glob(&glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                handle_path(path.to_str().unwrap());
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
