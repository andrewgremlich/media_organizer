extern crate clap;
extern crate media_organizer_lib;

mod organizer;

use clap::Parser;
use media_organizer_lib::env::{set_env, Args};
use organizer::{organizer_dir, organize_file};
use std::path::Path;

fn main() {
    let matches: Args = Args::parse();

    set_env(&matches);

    let path = Path::new(&matches.target);

    if path.exists() && path.is_dir() {
        organizer_dir(&matches.target);
    } else {
        organize_file(&matches.target);
    }
}
