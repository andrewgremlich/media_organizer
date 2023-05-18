mod organizer;

use clap::Parser;
use env::{set_env, Args};
use organizer::{handle_path, organize_dir};
use std::path::Path;

fn main() {
    let matches: Args = Args::parse();

    set_env(&matches);

    let path = Path::new(&matches.target);

    if !path.exists() {
        println!("Path does not exist: {}", &matches.target);
        return;
    }

    if path.is_dir() {
        organize_dir(&matches.target);
    } else if path.is_file() {
        handle_path(&matches.target);
    } else {
        println!("Path is not a file or directory: {}", &matches.target);
    }
}
