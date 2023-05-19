mod handle_media;
mod make_file_destination;

use glob::glob;
use handle_media::handle_media;
use make_file_destination::sort_and_make;
use mkdirp::mkdirp;
use std::env;
use std::path::Path;

pub fn handle_path(path: &str) {
    if Path::new(&path).is_file() {
        match sort_and_make(path) {
            Ok(date) => {
                mkdirp(&date).expect("Could not create directory");
                handle_media(path, &date);
            }
            Err(err) => println!("Error: {}", err),
        }
    } else {
        println!("Path is not a file: {}", path);
    }
}

pub fn organize_dir(dir_str: &str) {
    let mut glob_path: String = String::new();
    let file_type = env::var("FILE_TYPE").expect("FILE_TYPE not set");
    let mut count_filepaths: u32 = 0;

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

        count_filepaths += 1;
    }

    if count_filepaths == 0 {
        println!("No files found in directory: {}", dir_str);
    }
}
