mod determine_file_type;

use std::env;
use std::fs::rename;
use std::path::PathBuf;

pub use determine_file_type::{is_photo, is_video};

pub fn move_image(original_file: &str, dest_dir: &str) {
  let mut original_file_path_buf: PathBuf = PathBuf::new();

  original_file_path_buf.push(original_file);

  if let Some(file_name) = original_file_path_buf.file_name() {
    if let Some(file_name_to_str) = file_name.to_str() {
      let mut owned_dest_string: String = dest_dir.to_owned();

      owned_dest_string.push_str("/");
      owned_dest_string.push_str(file_name_to_str);

      // this is to relocate instead of rename file.
      // TODO: this will error if the folder doesn't have write permissions
      match rename(original_file, owned_dest_string) {
        Ok(_e) => (),
        Err(_) => println!("File not relocated: {:?}", original_file),
      };
    }
  }
}

pub fn make_dir_string(date_time: Option<&str>) -> String {
  match date_time {
    Some(e) => {
      let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
      let replace_date_hyphens = str::replace(e, "-", "/");
      let mut dir_to_create: String = String::new();

      dir_to_create.push_str("./");
      dir_to_create.push_str(&dest_folder);
      dir_to_create.push_str("/");
      dir_to_create.push_str(&replace_date_hyphens);

      return dir_to_create;
    }
    None => println!("No dates exist. {:?}", date_time),
  };

  return String::from("There are no directory strings to be made!");
}
