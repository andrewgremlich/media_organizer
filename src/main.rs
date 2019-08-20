extern crate exif;
extern crate glob;

use self::glob::glob;
use std::fs::{create_dir_all, rename, File};
use std::path::{Display, Path};
use std::{env, process};

fn make_dirs(date_time: &str) {
    let mut split_date_time_spaces = date_time.split_whitespace();

    match split_date_time_spaces.next() {
        Some(e) => {
            let replace_date_hyphens = str::replace(e, "-", "/");
            let dir_to_create = "./photos/".to_owned() + &replace_date_hyphens;

            match create_dir_all(&dir_to_create) {
                Ok(_e) => println!("{:?} Created!", &dir_to_create),
                Err(_) => println!("{:?} could not be created!", &dir_to_create)
            }
        }
        None => println!("{:?}", "No dates exist."),
    };
}

fn read_exif_date_data(image_path: &Display) -> String {
    let image_path_str: &str = &image_path.to_string();
    let path = Path::new(image_path_str);

    let file = File::open(path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();

    let date_data: String = match reader.get_field(exif::Tag::DateTime, false) {
        Some(data) => data.value.display_as(data.tag).to_string(),
        None => String::from("false"),
    };

    return date_data;
}

fn make_photo_library(photos_dir_str: &str) {
    let white_list_file_types: Vec<&str> = vec!["jpeg", "jpg", "JPEG", "JPG"];

    for file_type in &white_list_file_types {
        let glob_path = photos_dir_str.to_owned() + "**/*." + file_type.to_owned();
        for entry in glob(&glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let image_path: Display = path.display();
                    let date_data: String = read_exif_date_data(&image_path);

                    make_dirs(&date_data);
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let photos_dir_str: &str = &args[1];

    if args.len() != 2 {
        println!("Did not input the right amount of arguments!  Just two please.");
        process::exit(1);
    }

    let photos_dir_path = Path::new(photos_dir_str);

    if photos_dir_path.is_dir() {
        make_photo_library(photos_dir_str);
    }
}
