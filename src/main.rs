extern crate clap;

use clap::{App, Arg, ArgMatches};
use photo_organizer::organize::make_photo_library;
use std::path::Path;

fn main() {
    let matches: ArgMatches = App::new("Photo Organizer")
        .version("0.2.0")
        .author("Andrew Gremlich")
        .about("Organize photos in a date structure")
        .arg(
            Arg::with_name("Target")
                .short("t")
                .long("target")
                .value_name("TARGET_FOLDER")
                .help("Target folder where all the unorganized photos are.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    if let Some(tar) = matches.value_of("Target") {
        let photos_dir_path = Path::new(tar);

        if photos_dir_path.is_dir() {
            make_photo_library(tar);
        }
    }
}
