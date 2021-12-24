use clap::ArgMatches;

use std::env;

pub fn set_env(matches: &ArgMatches) {
    if let Some(d) = matches.value_of("destination") {
        env::set_var("DEST_FOLDER", d);
    }

    if let Some(f) = matches.value_of("filetype") {
        env::set_var("FILE_TYPE", f);
    }

    if matches.is_present("copy") {
        env::set_var("COPY", true.to_string());
    } else {
        env::set_var("COPY", false.to_string());
    }
}
