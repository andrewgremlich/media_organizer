extern crate clap;

mod env;
mod sorter;

use clap::{App, Arg, ArgMatches};
use env::set_env;
use sorter::{sort_dir, sort_file};
use std::path::Path;

fn main() {
    let matches: ArgMatches = App::new("Photo Organizer")
        .version("0.3.6")
        .author("Andrew Gremlich")
        .about("Organize media in one folder into date-centric folder structure.")
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
                .default_value("media")
                .help("Name of the folder in the current directory where organized media will be put.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filetype")
                .short("f")
                .long("filetype")
                .value_name("FILE_TYPE")
                .default_value("*")
                .help("File type to sort. Defaults to all file types.")
                .takes_value(true)
        )
        .get_matches();

    set_env(&matches);

    if let Some(targ) = matches.value_of("target") {
        let path = Path::new(targ);

        if path.exists() && path.is_dir() {
            sort_dir(targ);
        } else {
            sort_file(targ);
        }
    }
}
