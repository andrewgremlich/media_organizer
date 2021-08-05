extern crate glob;
extern crate mkdirp;

mod utils;
mod media_info;

use self::glob::glob;
use media_info::date_data::{read_photo_creation_date, read_video_creation_date};
use mkdirp::mkdirp;
use utils::{get_white_list_file_types, is_photo, is_video, make_dir_string, move_image};

pub fn sorter(dir_str: &str) {
    let white_list_file_types: Vec<&str> = get_white_list_file_types();

    for file_type in &white_list_file_types {
        let mut glob_path: String = String::new();

        glob_path.push_str(dir_str);
        glob_path.push_str("/*.");
        glob_path.push_str(file_type);

        for entry in glob(&glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let path_str: &str = path.to_str().unwrap();
                    let mut date_data: String = String::new();

                    if is_photo(path_str) {
                        let date_of_photo: String = read_photo_creation_date(&path_str);
                        let photo_dir_str: String =
                            make_dir_string(date_of_photo.split_whitespace().next());

                        date_data.push_str(&photo_dir_str);
                    }
                    if is_video(path_str) {
                        let date_of_video: String = read_video_creation_date(&path_str);
                        let video_dir_str: String = make_dir_string(date_of_video.split("T").next());
                        date_data.push_str(&video_dir_str);
                    }

                    mkdirp(&date_data).unwrap();
                    move_image(path_str, &date_data);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
