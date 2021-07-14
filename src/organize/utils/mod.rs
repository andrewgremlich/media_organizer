mod determine_file_type;

use std::env;
use std::fs::rename;
use std::path::PathBuf;

pub use determine_file_type::{get_white_list_file_types, is_photo, is_video};

pub fn move_image(original_file: PathBuf, dest_dir: &str) {
  if let Some(file_name) = original_file.file_name() {
    if let Some(file_name_to_str) = file_name.to_str() {
      let mut owned_dest_string: String = dest_dir.to_owned();

      owned_dest_string.push_str("/");
      owned_dest_string.push_str(file_name_to_str);

      match rename(original_file, owned_dest_string) {
        Ok(_e) => (),
        Err(_) => println!("File not relocated"),
      };
    }
  }
}

pub fn make_dir_string(date_time: &str) -> String {
  let mut split_date_time_spaces = date_time.split_whitespace();

  match split_date_time_spaces.next() {
    Some(e) => {
      let dest_folder: String = env::var("DEST_FOLDER").unwrap();
      let replace_date_hyphens = str::replace(e, "-", "/");

      let mut dir_to_create: String = String::new();

      dir_to_create.push_str("./");
      dir_to_create.push_str(&dest_folder);
      dir_to_create.push_str("/");
      dir_to_create.push_str(&replace_date_hyphens);

      return dir_to_create;
    }
    None => println!("{:?}", "No dates exist."),
  };

  return String::from("There are no directory strings to be made!");
}
