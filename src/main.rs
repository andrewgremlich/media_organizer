mod photo_library;

use std::{env, process};
use photo_library::make_photo_library;
use std::path::{Path};

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
