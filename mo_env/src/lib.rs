use clap::Parser;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        value_name = "TARGET_MEDIA",
        help = "The target media to sort.",
        required = true
    )]
    pub target: String,

    #[clap(
        short,
        long,
        value_name = "DESTINATION_FOLDER",
        default_value = "sorted_media",
        help = "The destination folder of sorted media."
    )]
    destination: String,

    #[clap(
        short,
        long,
        value_name = "FILE_TYPE",
        default_value = "*",
        help = "The file type to sort."
    )]
    file_type: String,

    #[clap(
        short,
        long,
        value_name = "COPY",
        help = "Copy the files instead of moving them.",
        default_value = "false"
    )]
    copy: bool,
}

pub fn set_env(matches: &Args) {
    env::set_var("DEST_FOLDER", &matches.destination);
    env::set_var("FILE_TYPE", &matches.file_type);
    env::set_var("COPY", matches.copy.to_string());
}
