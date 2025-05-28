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

/// Creates a new `AudioInfo` instance by reading metadata from the specified file path.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` representing the location of the audio file.
///
/// # Returns
///
/// * `Ok(AudioInfo)` if the file exists and metadata is successfully read.
/// * `Err(String)` if the file does not exist or if there is an error reading metadata.
///
/// # Errors
///
/// Returns an error if the file does not exist at the given path or if metadata extraction fails.
///
/// # Example
///
/// ```rust
/// let audio_info = AudioInfo::new(Path::new("song.mp3"))?;
/// ```
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