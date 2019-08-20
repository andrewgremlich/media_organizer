extern crate exif;
extern crate glob;

use self::glob::glob;
use std::fs::{create_dir_all, rename, File};
use std::path::{Display, Path};

fn move_image(file_name: Option<&str>, made_dir: String, image_path_str: &str) {
    match file_name {
        Some(file_name) => {
            let new_file_name = made_dir + "/" + file_name;

            println!("{:?}", new_file_name);
            println!("{:?}", image_path_str);

            match rename(image_path_str, new_file_name) {
                Ok(_e) => println!("File relocated!"),
                Err(_) => println!("File not relocated"),
            };
        },
        _ => ()
    }
}

fn make_dir(date_time: &str) -> String {
    let mut split_date_time_spaces = date_time.split_whitespace();

    match split_date_time_spaces.next() {
        Some(e) => {
            let replace_date_hyphens = str::replace(e, "-", "/");
            let dir_to_create = "./photos/".to_owned() + &replace_date_hyphens;

            match create_dir_all(&dir_to_create) {
                Ok(_e) => (),
                Err(_) => println!("{:?} could not be created!", &dir_to_create),
            }

            return dir_to_create;
        }
        None => println!("{:?}", "No dates exist."),
    };

    return String::from("Somehow no directories were made!");
}

fn read_exif_date_data(image_path_str: &str) -> String {
    let path = Path::new(image_path_str);

    let file = File::open(path).unwrap();
    let reader = exif::Reader::new(&mut std::io::BufReader::new(&file)).unwrap();

    let date_data: String = match reader.get_field(exif::Tag::DateTime, false) {
        Some(data) => data.value.display_as(data.tag).to_string(),
        None => String::from("false"),
    };

    return date_data;
}

pub fn make_photo_library(photos_dir_str: &str) {
    let white_list_file_types: Vec<&str> = vec!["jpeg", "jpg", "JPEG", "JPG"];

    for file_type in &white_list_file_types {
        let glob_path = photos_dir_str.to_owned() + "/*." + file_type.to_owned();

        for entry in glob(&glob_path).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let image_path: Display = path.display();
                    let image_path_str: &str = &image_path.to_string();

                    let date_data: String = read_exif_date_data(image_path_str);
                    let made_dir: String = make_dir(&date_data);
                    
                    match path.file_name() {
                        Some(data) => move_image(data.to_str(), made_dir, image_path_str),
                        _ => (),
                    }
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}