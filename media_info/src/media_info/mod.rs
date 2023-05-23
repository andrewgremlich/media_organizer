use ffmpeg_next as ffmpeg;
use std::path::PathBuf;

mod file_metadata;

use chrono::NaiveDate;
use exif::{In, Reader, Tag as ExifTag};
use file_metadata::file_created;
use id3::{ErrorKind, Timestamp};
use id3::{Tag as ID3Tag, TagLike};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

/// Writes the creation date of the audio to the metadata.
/// 
/// # Examples
/// ```
/// let audio_path = "tests/test_data/audio.mp3";
/// let creation_date = "2021-01-01";
/// make_date_recorded_from_file(audio_path, creation_date).unwrap();
/// ```
fn make_date_recorded_from_file(path_str: &str) -> Option<Timestamp> {
    let formatted_date = file_created(path_str);
    let id3_timestamp = Timestamp::from_str(&formatted_date).expect("could not write timestamp");

    let mut tag = ID3Tag::new();
    tag.set_date_recorded(id3_timestamp);
    tag.write_to_path(path_str, id3::Version::Id3v24)
        .expect("could not write tag");

    Some(id3_timestamp)
}

/// Reads the creation date of the audio from the metadata.
/// If it can't read the audio's creation date, it will fall back to the file's creation date.
/// 
/// # Examples
/// ```
/// use media_info::read_audio_creation_date;
/// 
/// let audio_path = "tests/test_data/audio.mp3";
/// let creation_date = read_audio_creation_date(audio_path).unwrap();
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn read_audio_creation_date(path_str: &str) -> Result<String, String> {
    let date_recorded = match ID3Tag::read_from_path(path_str) {
        Ok(tags) => tags.date_recorded(),
        Err(why) => match why.kind {
            ErrorKind::NoTag => make_date_recorded_from_file(path_str),
            _ => None,
        },
    };
    let year = date_recorded.map(|t| t.year).expect("No year found");
    let month = date_recorded
        .map(|t| t.month.unwrap())
        .expect("No month found");
    let day = date_recorded.map(|t| t.day.unwrap()).expect("No day found");

    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);
    let date_str = assembled_date.unwrap().format("%Y-%m-%d").to_string();

    Ok(date_str)
}

/// Reads the creation date of the photo from the metadata.
/// If it can't read the photo's creation date, it will fall back to the file's creation date.
/// 
/// # Examples
/// ```
/// use media_info::read_photo_creation_date;
/// 
/// let photo_path = "tests/test_data/photo.jpg";
/// let creation_date = read_photo_creation_date(photo_path).unwrap();
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn read_photo_creation_date(path_str: &str) -> Result<String, String> {
    let file = File::open(Path::new(path_str)).expect("could not open photo file");

    match Reader::new().read_from_container(&mut std::io::BufReader::new(&file)) {
        Ok(reader) => {
            let date_data: String = match reader.get_field(ExifTag::DateTime, In::PRIMARY) {
                Some(data) => data.value.display_as(data.tag).to_string(),
                None => return Err(String::from("couldnotreadphotocreationdate")),
            };
            Ok(date_data)
        }
        Err(_) => {
            println!("Error reading exif: {:?}", path_str);
            println!("Falling back to file creation date");
            // TODO: edit exif date with file creation date?

            let formatted_date = file_created(path_str);
            Ok(formatted_date)
        }
    }
}


/// Reads the creation date of the video from the metadata.
/// If it can't read the video's creation date, it will fall back to the file's creation date.
/// 
/// # Examples
/// ```
/// use media_info::read_video_creation_date;
/// 
/// let video_path = "tests/test_data/video.mp4";
/// let creation_date = read_video_creation_date(video_path).unwrap();
/// assert_eq!(creation_date, "2021-01-01");
/// ```
pub fn read_video_creation_date(path_str: &str) -> Result<String, String> {
    ffmpeg::init().expect("could not initialize ffmpeg");

    match ffmpeg::format::input(&PathBuf::from(path_str)) {
        Ok(context) => {
            let mut creation_date: String = String::new();

            for (name, value) in context.metadata().iter() {
                if name == "creation_time" {
                    creation_date.push_str(value)
                }
            }

            Ok(creation_date)
        }
        Err(_) => {
            println!("Error reading video creation date: {:?}", path_str);
            println!("Falling back to file creation date");
            // TODO: edit creation date with file creation date?

            let formatted_date = file_created(path_str);
            Ok(formatted_date)
        }
    }
}