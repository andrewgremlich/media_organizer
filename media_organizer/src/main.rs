mod organizer;

use clap::Parser;
use organizer::{organize_file, organize_dir};
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use log::error;
use structured_logger::Builder;
use structured_logger::json::new_writer;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(
        short,
        long,
        value_name = "SOURCE_FOLDER",
        help = "The absolute path to the source folder of the media to be sorted.",
        required = true
    )]
    source: String,

    #[clap(
        short,
        long,
        value_name = "DESTINATION_FOLDER",
        default_value = "sorted",
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

    #[clap(short, long, help = "Copy the files instead of moving them.")]
    copy: bool,

    #[clap(short = 'y', long, help = "Dry-run with statistics but without actually copying or moving.")]
    dry_run: bool,

    #[clap(short = 'l', long, help = "Log each saved file in a log-file.")]
    log_saved: bool,

    #[clap(long, help = "Append width x height dimensions to image and video filenames.")]
    dimensions: bool,

    #[clap(short, long, help = "Print log output to the terminal.")]
    verbose: bool,
}

fn set_env(matches: &Args) {
    unsafe {
        env::set_var("DEST_FOLDER", &matches.destination);
        env::set_var("FILE_TYPE", &matches.file_type);
        env::set_var("COPY", matches.copy.to_string());
        env::set_var("DRY_RUN", matches.dry_run.to_string());
        env::set_var("LOG_SAVED", matches.log_saved.to_string());
        env::set_var("DIMENSIONS", matches.dimensions.to_string());
    }
}

fn main() {
    let matches: Args = Args::parse();

    let mut builder = Builder::with_level("debug");

    if !matches.dry_run {
        let same_file_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("same_file.log")
            .expect("Unable to create or open log file for same_file logging");
        builder = builder.with_target_writer("same_file", new_writer(same_file_log_file));
    }

    if matches.log_saved {
        let saved_file_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("saved_file.log")
            .expect("Unable to create or open log file for saved_file logging");
        builder = builder.with_target_writer("saved_file", new_writer(saved_file_log_file));
    }

    if !matches.verbose {
        builder = builder.with_default_writer(new_writer(io::sink()));
    }

    builder.init();

    set_env(&matches);

    let path = Path::new(&matches.source);

    if !path.exists() {
        error!("Path to source folder does not exist: {}", &matches.source);
        return;
    }

    if path.is_dir() {
        organize_dir(&matches.source);
    } else if path.is_file() {
        organize_file(&matches.source);
    } else {
        error!("Path is not a file or directory: {}", &matches.source);
    }
}
