extern crate glob;
extern crate mkdirp;

mod utils;

use self::glob::glob;
use crate::exif_wrapper::date_data::read_exif_date_data;
use mkdirp::mkdirp;
use std::path::Display;
use utils::{make_dir_string, move_image};

pub fn make_photo_library(photos_dir_str: &str) {
    let white_list_file_types: Vec<&str> = vec!["jpeg", "jpg", "JPEG", "JPG", "HEIC", "heic", "PNG", "png"];

    for file_type in &white_list_file_types {
        let mut glob_path: String = photos_dir_str.to_owned();

        glob_path.push_str("/*.");
        glob_path.push_str(file_type);

        for entry in glob(&glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let image_path: Display = path.display();
                    let image_path_str: &str = &image_path.to_string();
                    let date_data: String = read_exif_date_data(image_path_str);

                    let made_dir: String = make_dir_string(&date_data);
                    mkdirp(&made_dir).unwrap();
                    move_image(path, &made_dir);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
