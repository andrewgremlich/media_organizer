use std::path::Path;

use chrono::NaiveDate;
use id3::ErrorKind;
use id3::{Tag as ID3Tag, TagLike};

mod dates;

pub mod struct_audio_info;

pub fn read_audio_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let date_recorded = match ID3Tag::read_from_path(path) {
        Ok(tags) => tags.date_recorded(),
        Err(why) => match why.kind {
            ErrorKind::NoTag => dates::make_date_recorded_from_audio_file(path),
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
