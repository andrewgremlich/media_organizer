extern crate clap;

mod env;

use clap::{App, Arg, ArgMatches};
use env::set_env;
use photo_organizer::sorter;
use std::path::Path;

fn main() {
    let matches: ArgMatches = App::new("Photo Organizer")
        .version("0.4.0")
        .author("Andrew Gremlich")
        .about("Organize photos in one folder into date-centric folder structure.")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET_FOLDER")
                .help("Target folder where all the unorganized photos are.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("destination")
                .short("d")
                .long("dest")
                .value_name("DEST_FOLDER")
                .default_value("photos")
                .help("Name of the folder in the current directory where organized photos will be put.")
                .takes_value(true),
        )
        .get_matches();

    set_env(&matches);

    if let Some(targ) = matches.value_of("target") {
        let dir_path = Path::new(targ);

        if dir_path.is_dir() {
            sorter(targ);
        }
    }
}
