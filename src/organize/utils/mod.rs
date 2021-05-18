
use std::fs::rename;
use std::path::PathBuf;

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
      let replace_date_hyphens = str::replace(e, "-", "/");
      let dir_to_create = "./photos/".to_owned() + &replace_date_hyphens;

      return dir_to_create;
    }
    None => println!("{:?}", "No dates exist."),
  };

  return String::from("There are no directory strings to be made!");
}
