use chrono::NaiveDate;
use id3::ErrorKind;
use id3::{Tag as ID3Tag, TagLike};

mod audio_util;

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
pub fn read_audio_creation_date(path_str: &str) -> String {
    let date_recorded = match ID3Tag::read_from_path(path_str) {
        Ok(tags) => tags.date_recorded(),
        Err(why) => match why.kind {
            ErrorKind::NoTag => audio_util::make_date_recorded_from_audio_file(path_str),
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

    date_str
}
