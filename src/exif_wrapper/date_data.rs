extern crate exif;

use exif::{In, Reader, Tag};
use std::fs::{metadata, File};
use std::io::Result;
use std::path::Path;

fn get_meta_data(path: &str) -> Result<()> {
  let metadata = metadata(path)?;

  if let Ok(time) = metadata.created() {
    println!("{:?}", time);
    return Ok(());
  } else {
    return Ok(());
    // return Err(());
  }
}

pub fn read_creation_date(path_str: &str) -> String {
  let path = Path::new(path_str);
  get_meta_data(path_str);

  println!("{:?}", path);

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
