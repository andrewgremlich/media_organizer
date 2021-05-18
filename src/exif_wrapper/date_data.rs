extern crate exif;

use exif::{In, Reader, Tag};
use std::fs::File;
use std::path::Path;

pub fn read_exif_date_data(image_path_str: &str) -> String {
  let path = Path::new(image_path_str);

  let file = File::open(path).unwrap();
  let reader = Reader::new()
    .read_from_container(&mut std::io::BufReader::new(&file))
    .unwrap();

  let date_data: String = match reader.get_field(Tag::DateTime, In::PRIMARY) {
    Some(data) => data.value.display_as(data.tag).to_string(),
    None => String::from("false"),
  };

  return date_data;
}
