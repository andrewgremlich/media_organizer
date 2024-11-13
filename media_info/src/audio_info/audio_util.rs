use id3::Timestamp;
use id3::{Tag as ID3Tag, TagLike};
use std::path::Path;
use std::str::FromStr;
use fs_metadata::file_created;

pub fn make_date_recorded_from_audio_file(path: &Path) -> Option<Timestamp> {
    if !path.exists() {
        panic!("File does not exist: {:?}", path);
    }

    let formatted_date = file_created(path).unwrap();
    let id3_timestamp = Timestamp::from_str(&formatted_date).expect("could not write timestamp");

    let mut tag = ID3Tag::new();
    tag.set_date_recorded(id3_timestamp);
    tag.write_to_path(path, id3::Version::Id3v24)
        .expect("could not write tag");

    Some(id3_timestamp)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn can_read_audio_creation_date() {
//         let raw_path_str = "../test-media/Recording.m4a";
//         let path = Path::new(raw_path_str);
//         let creation_date = make_date_recorded_from_audio_file(path);

//         assert_eq!(creation_date.unwrap().contains("2024-10-22"), true);
//     }
// }