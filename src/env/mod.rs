use clap::ArgMatches;

use std::env;

pub fn set_env(matches: &ArgMatches) {
    if let Some(d) = matches.value_of("destination") {
        env::set_var("DEST_FOLDER", d);
    }
}
