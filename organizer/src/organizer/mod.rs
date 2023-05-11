mod determine_file_type;
mod handle_media;
mod make_dir_str;

use determine_file_type::{is_audio, is_photo, is_video};
use glob::glob;
use handle_media::handle_media;
use make_dir_str::{make_audio_dir_str, make_photo_dir_str, make_video_dir_str};
use mkdirp::mkdirp;
use std::env;

fn handle_path(path: &str) {
    let path_str: &str = path;
    let mut date_data: String = String::new();

    if is_photo(path_str) {
        date_data.push_str(&make_photo_dir_str(path_str));
    }
    if is_video(path_str) {
        date_data.push_str(&make_video_dir_str(path_str));
    }
    if is_audio(path_str) {
        println!("{:?} is audio", make_audio_dir_str(path_str));
    }

    mkdirp(&date_data).expect("Could not create directory");
    handle_media(path_str, &date_data);
}

pub fn organize_file(file_path: &str) {
    handle_path(file_path);
}

pub fn organize_dir(dir_str: &str) {
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
            Err(e) => println!("Glop path entry failed: {:?}", e),
        }
    }
}
