use chrono::NaiveDate;
use fs_metadata::file_modified;
use id3::{ErrorKind, Tag as ID3Tag, TagLike, Timestamp};
use std::path::Path;
use std::str::FromStr;

pub mod struct_audio_info;

/// Reads the creation date of the audio from the metadata.
///
/// If it can't read the audio's creation date, it will fall back to the file's modification date.
///
/// TODO This function invokes `make_date_recorded_from_audio_file` that has side-effect of writing
/// ID3 information back to provided file path. Don't think it is good practice to change source files.
///
/// # Examples
/// ```
/// use media_info::read_audio_creation_date;
///
/// let audio_path = std::path::Path::new("../test-media/Recording.m4a");
/// let creation_date = read_audio_creation_date(audio_path).unwrap();
/// assert_eq!(creation_date, "2024-11-11");
/// ```
pub fn read_audio_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let date_recorded = match ID3Tag::read_from_path(path) {
        Ok(tags) => tags.date_recorded(),
        Err(why) => match why.kind {
            ErrorKind::NoTag => make_date_recorded_from_audio_file(path),
            _ => None,
        },
    };
    let year = date_recorded.map(|t| t.year).expect("No year found");
    let month = date_recorded
        .map(|t| t.month.unwrap())
        .expect("No month found");
    let day = date_recorded.map(|t| t.day.unwrap()).expect("No day found");

    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);
    let date_str = match assembled_date {
        Some(date) => date.format("%Y-%m-%d").to_string(),
        None => "no.date".to_string(),
    };

    Ok(date_str)
}

/// Writes the modification date of the audio to the metadata.
fn make_date_recorded_from_audio_file(path_str: &Path) -> Option<Timestamp> {
    let formatted_date = file_modified(path_str).unwrap();
    let id3_timestamp = Timestamp::from_str(&formatted_date).expect("could not write timestamp");

    let mut tag = ID3Tag::new();
    tag.set_date_recorded(id3_timestamp);
    tag.write_to_path(path_str, id3::Version::Id3v24)
        .expect("could not write tag");

    Some(id3_timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_audio_creation_date() {
        let raw_path_str = "../test-media/Recording.m4a";
        let path = Path::new(raw_path_str);
        let creation_date = read_audio_creation_date(path);

        assert_eq!(creation_date.unwrap().contains("2024-11-11"), true);
    }
}
