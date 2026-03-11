mod compare_files;
mod counter;
mod handle_media;
mod make_file_destination;
#[cfg(target_os = "windows")]
mod set_creation_time_windows;

use crate::organizer::counter::{
    add_unsupported_type, get_identical_file_counter, get_ignored_file_counter,
    get_same_name_diff_content_counter, get_saved_file_counter,
    get_successfully_compared_file_counter, get_unsupported_types, increment_ignored_file_counter,
};
use crate::organizer::make_file_destination::MakeFileDestinationError;
use glob::glob;
use handle_media::handle_media;
use log::{error, info, warn};
use make_file_destination::make_file_destination_str;
use media_info::get_fallback_counter;
use mkdirp::mkdirp;
use std::env;
use std::path::Path;

pub fn organize_file(file_str: &str) {
    if Path::new(&file_str).is_file() {
        match make_file_destination_str(file_str) {
            Ok(destination_dir) => {
                mkdirp(&destination_dir).expect(&format!(
                    "Could not create destination directory {:?} for the source file {}",
                    destination_dir, file_str
                ));
                handle_media(file_str, &destination_dir);
            }
            Err(MakeFileDestinationError::UnsupportedType(unsupported_type)) => {
                add_unsupported_type(unsupported_type);
                increment_ignored_file_counter()
            }
            Err(MakeFileDestinationError::Error(err)) => error!("Error: {}", err),
        }
    } else {
        error!("Provided path is not a file: {}", file_str);
    }
}

pub fn organize_dir(dir_str: &str) {
    let mut glob_path: String = String::new();
    let file_type = env::var("FILE_TYPE").expect("FILE_TYPE not set");
    let mut count_filepaths: u32 = 0;

    glob_path.push_str(dir_str);
    glob_path.push_str("/**/*.");
    glob_path.push_str(&file_type);

    for entry in glob(&glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if path.is_file() {
                    path.to_str().map(|file_str| {
                        organize_file(file_str);
                        count_filepaths += 1
                    });
                }
            }
            Err(e) => error!("Glob path entry failed: {:?}", e),
        }
    }

    if count_filepaths == 0 {
        warn!("No files found in directory: {}", dir_str);
    } else {
        info!(
            "Processed {} source files in directory: {}",
            count_filepaths, dir_str
        );

        let num_saved_files = get_saved_file_counter();
        if num_saved_files > 0 {
            info!("Saved {} files in destination directory", num_saved_files);
            info!(
                "Successfully compared {} files",
                get_successfully_compared_file_counter()
            );
        }

        let num_identical_files = get_identical_file_counter();
        if num_identical_files > 0 {
            info!("{} files already existed, were identical with source and thus were skipped", get_identical_file_counter());
        }

        let num_same_name_diff_content = get_same_name_diff_content_counter();
        if num_same_name_diff_content > 0 {
            info!("{} files had same name but different content and thus were saved as duplicate (dup. prefix)", get_same_name_diff_content_counter());
        }

        let num_fallback_files = get_fallback_counter();
        if num_fallback_files > 0 {
            info!("{} files didn't have exif infos and were sorted by file modification date", get_fallback_counter());
        }

        let num_ignored_files = get_ignored_file_counter();
        if num_ignored_files > 0 {
            info!("Ignored {} files because of incompatible types", num_ignored_files);
            info!("Following incompatible file types were encountered: {}", get_unsupported_types());
        }
    }
}
