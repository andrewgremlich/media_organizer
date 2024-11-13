use id3::Timestamp;
use id3::{Tag as ID3Tag, TagLike};
use std::path::Path;
use std::str::FromStr;
use fs_metadata::file_created;

/// Writes the creation date of the audio to the metadata.
///
/// # Examples
/// ```
/// let audio_path = "tests/test_data/audio.mp3";
/// let creation_date = "2021-01-01";
/// make_date_recorded_from_audio_file(audio_path, creation_date).unwrap();
/// ```
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
