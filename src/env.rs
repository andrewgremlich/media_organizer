use std::env;
extern crate clap;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        value_name = "TARGET_MEDIA",
        help = "The target media to sort.",
        takes_value = true,
        required = true
    )]
    pub target: String,

    #[clap(
        short,
        long,
        value_name = "DESTINATION_PATH",
        help = "The destination path",
        help = "The destination path of sorted media.",
        takes_value = true
    )]
    destination: String,

    #[clap(
        short,
        long,
        value_name = "FILE_TYPE",
        default_value = "*",
        help = "The file type to sort.",
        takes_value = true
    )]
    file_type: String,

    #[clap(
        short,
        long,
        value_name = "COPY",
        takes_value = false,
        help = "Copy the files instead of moving them."
    )]
    copy: bool,
}

pub fn set_env(matches: &Args) {
    env::set_var("DEST_FOLDER", &matches.destination);
    env::set_var("FILE_TYPE", &matches.file_type);
    env::set_var("COPY", matches.copy.to_string());
}
