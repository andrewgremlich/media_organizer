mod organizer;

use clap::{Args, Parser, Subcommand};
use organizer::{organize_file, organize_dir};
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use log::error;
use structured_logger::Builder;
use structured_logger::json::new_writer;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(flatten)]
    args: OrganizeArgs,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args, Debug)]
pub struct OrganizeArgs {
    #[clap(
        short,
        long,
        value_name = "SOURCE_FOLDER",
        help = "The absolute path to the source folder of the media to be sorted."
    )]
    source: Option<String>,

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

    #[clap(long, help = "Separate media into category subfolders (photos, video, audio, documents).")]
    categorize: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Find media files matching a glob pattern
    Find {
        /// Glob pattern to search for (e.g. "*.jpg", "vacation*")
        pattern: String,

        /// Directory to search in (defaults to current directory)
        #[clap(short, long, default_value = ".")]
        path: String,
    },
}

fn set_env(args: &OrganizeArgs) {
    unsafe {
        env::set_var("DEST_FOLDER", &args.destination);
        env::set_var("FILE_TYPE", &args.file_type);
        env::set_var("COPY", args.copy.to_string());
        env::set_var("DRY_RUN", args.dry_run.to_string());
        env::set_var("LOG_SAVED", args.log_saved.to_string());
        env::set_var("DIMENSIONS", args.dimensions.to_string());
        env::set_var("CATEGORIZE", args.categorize.to_string());
    }
}

fn run_find(pattern: &str, path: &str) {
    let full_pattern = format!("{}/**/{}", path, pattern);
    match glob::glob(&full_pattern) {
        Ok(entries) => {
            let mut found = false;
            for entry in entries.flatten() {
                if entry.is_file() {
                    println!("{}", entry.display());
                    found = true;
                }
            }
            if !found {
                println!("No files found matching '{}'", pattern);
            }
        }
        Err(e) => eprintln!("Invalid pattern: {}", e),
    }
}

fn run_organize(args: &OrganizeArgs) {
    let source = match &args.source {
        Some(s) => s,
        None => {
            eprintln!("error: the following required argument was not provided: --source <SOURCE_FOLDER>");
            std::process::exit(2);
        }
    };

    let mut builder = Builder::with_level("debug");

    if !args.dry_run {
        let same_file_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("same_file.log")
            .expect("Unable to create or open log file for same_file logging");
        builder = builder.with_target_writer("same_file", new_writer(same_file_log_file));
    }

    if args.log_saved {
        let saved_file_log_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("saved_file.log")
            .expect("Unable to create or open log file for saved_file logging");
        builder = builder.with_target_writer("saved_file", new_writer(saved_file_log_file));
    }

    if !args.verbose && !args.dry_run {
        builder = builder.with_default_writer(new_writer(io::sink()));
    }

    builder.init();

    set_env(args);

    let path = Path::new(source);

    if !path.exists() {
        error!("Path to source folder does not exist: {}", source);
        return;
    }

    if path.is_dir() {
        organize_dir(source);
    } else if path.is_file() {
        organize_file(source);
    } else {
        error!("Path is not a file or directory: {}", source);
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Find { pattern, path }) => {
            run_find(pattern, path);
        }
        None => {
            run_organize(&cli.args);
        }
    }
}
