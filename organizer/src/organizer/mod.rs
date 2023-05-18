mod handle_media;
mod make_file_destination;

use glob::glob;
use handle_media::handle_media;
use make_file_destination::sort_and_make;
use mkdirp::mkdirp;
use std::env;

pub fn handle_path(path: &str) {
    let path_str: &str = path;
    let mut date_data: String = String::new();

    match sort_and_make(path_str) {
        Ok(date) => date_data.push_str(&date),
        Err(err) => {
            println!("Error: {}", err);
            date_data.push_str("filetypenotsupported");
        }
    }

    mkdirp(&date_data).expect("Could not create directory");
    handle_media(path_str, &date_data);
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
