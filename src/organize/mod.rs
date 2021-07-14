extern crate glob;
extern crate mkdirp;

mod utils;

use self::glob::glob;
use crate::exif_wrapper::date_data::read_creation_date;
use mkdirp::mkdirp;
use std::path::Display;
use utils::{get_white_list_file_types, is_photo, is_video, make_dir_string, move_image};

fn make_photo_library(photos_dir_str: &str) {
    for entry in glob(&photos_dir_str).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let image_path: Display = path.display();
                let path_str: &str = &image_path.to_string();
                let date_data: String = read_creation_date(path_str);
                let made_dir: String = make_dir_string(&date_data);

                println!("{:?}", date_data);
                println!("{:?}", made_dir);

                mkdirp(&made_dir).unwrap();
                move_image(path, &made_dir);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

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

                    if is_photo(path_str) {
                        make_photo_library(&path_str);
                    }

                    if is_video(path_str) {
                        println!("IS VIDEO {:?} {:?}", path_str, is_video(path_str));
                        // Do something with video
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
