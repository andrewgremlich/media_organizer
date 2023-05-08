mod determine_file_type;
mod handle_media;
mod make_dir_str;

use chrono::{DateTime, Local, TimeZone};
use determine_file_type::{is_audio, is_photo, is_video};
use glob::glob;
use handle_media::handle_media;
use id3::{ErrorKind, Frame, Timestamp};
use id3::{Tag, TagLike};
use make_dir_str::{make_photo_dir_str, make_video_dir_str};
use mkdirp::mkdirp;
use std::env;
use std::fs;
use std::str::FromStr;

fn handle_path(path: &str) {
    let path_str: &str = path;
    let mut date_data: String = String::new();

    if is_photo(path_str) {
        date_data.push_str(&make_photo_dir_str(path_str));
    }
    if is_video(path_str) {
        date_data.push_str(&make_video_dir_str(path_str));
    }
    if is_audio(path_str) {
        println!("Audio file found: {}", path_str);
        let file_metadata = fs::metadata(path_str).expect("Failed to read file metadata");

        let created = file_metadata
            .created()
            .expect("Failed to read file creation date");
        let datetime: DateTime<Local> = created.into();
        let formatted_date = datetime.format("%Y-%m-%d").to_string();
        let id3_timestamp =
            Timestamp::from_str(&formatted_date).expect("could not write timestamp");

        println!("created: {:?}", formatted_date);

        let existing_tags = match Tag::read_from_path(path_str) {
            Ok(tags) => tags,
            Err(why) => match why.kind {
                ErrorKind::NoTag => {
                    println!("No tag found");
                    Tag::new()
                }
                _ => panic!("An error occurred while reading the file"),
            },
        };

        println!("existing tags: {:?}", existing_tags);

        let mut tag = Tag::new();
        tag.set_date_recorded(id3_timestamp);
        tag.write_to_path(path, id3::Version::Id3v24);

        // if let Some(artist) = tag.artist() {
        //     println!("artist: {}", artist);
        // }
        // if let Some(title) = tag.title() {
        //     println!("title: {}", title);
        // }
        // if let Some(album) = tag.album() {
        //     println!("album: {}", album);
        // }

        // make_audio_dir_str(path_str);
    }

    mkdirp(&date_data).expect("Could not create directory");
    handle_media(path_str, &date_data);
}

pub fn organize_file(file_path: &str) {
    handle_path(file_path);
}

pub fn organize_dir(dir_str: &str) {
    let mut glob_path: String = String::new();
    let file_type = env::var("FILE_TYPE").expect("FILE_TYPE not set");

    glob_path.push_str(dir_str);
    glob_path.push_str("/**/*.");
    glob_path.push_str(&file_type);

    for entry in glob(&glob_path).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => match path.to_str() {
                Some(path_str) => {
                    handle_path(path_str);
                }
                None => println!("Failed to convert path to string"),
            },
            Err(e) => println!("Glop path entry failed: {:?}", e),
        }
    }
}
