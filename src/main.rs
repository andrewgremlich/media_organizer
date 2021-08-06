extern crate clap;

mod env;
mod sorter;

use clap::{App, Arg, ArgMatches};
use env::set_env;
use sorter::{sort_dir, sort_file};
use std::path::Path;

fn main() {
    // TODO: Change target argument name? It's confusing me.
    let matches: ArgMatches = App::new("Photo Organizer")
        .version("0.2.0")
        .author("Andrew Gremlich")
        .about("Organize photos in one folder into date-centric folder structure.")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET_MEDIA")
                .help("Target media. Can be a folder with unorganized media or a single file.")
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
        let path = Path::new(targ);
        let path_exists = path.exists();
        let path_is_dir = path.is_dir();

        if path_exists && path_is_dir {
            sort_dir(targ);
        } else {
            sort_file(targ);
        }
    }
}
