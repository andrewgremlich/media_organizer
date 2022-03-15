extern crate clap;
extern crate media_organizer_lib;

mod sorter;

use clap::Parser;
use media_organizer_lib::env::{set_env, Args};
use sorter::{sort_dir, sort_file};
use std::path::Path;

fn main() {
    let matches: Args = Args::parse();

    set_env(&matches);

    let path = Path::new(&matches.target);

    if path.exists() && path.is_dir() {
        sort_dir(&matches.target);
    } else {
        sort_file(&matches.target);
    }
}
