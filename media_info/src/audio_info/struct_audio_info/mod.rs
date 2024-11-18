use std::path::Path;

use id3::Tag as ID3Tag;

mod id3_tags;

#[derive(Debug)]
pub struct AudioInfo {
  pub creation_date: String,
  pub artist: String,
  pub title: String,
  pub album: String,
  pub duration: String,
  pub released: String,
  pub genre: String,
}

impl AudioInfo {
  pub fn new (path: &Path) -> Result<Self, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let tag = ID3Tag::read_from_path(path).unwrap();

    let date_recorded = id3_tags::get_date_recorded(&tag);
    let artist = id3_tags::get_artist(&tag);
    let title = id3_tags::get_title(&tag);
    let album = id3_tags::get_album(&tag);
    let duration = id3_tags::get_duration(&tag);
    let released = id3_tags::get_release_date(&tag);
    let genre = id3_tags::get_genre(&tag);

    Ok(AudioInfo {
      creation_date: date_recorded,
      artist,
      title,
      album,
      duration,
      released,
      genre,
    })
  }
}

#[cfg(test)]
mod photo_info_struct {
  use super::*;

  #[test]
  fn can_read_audio_creation_date() {
    let raw_path_str = "../test-media/Recording.m4a";
    let path = Path::new(raw_path_str);
    let audio_info = AudioInfo::new(path).unwrap();

    assert_eq!(audio_info.creation_date.contains("2024-11-11"), true);
  }
}