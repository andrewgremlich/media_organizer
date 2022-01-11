use permissions::is_removable;
use std::env;
use std::fs::{copy, rename};
use std::path::PathBuf;

fn handle_if_removable(file: &str) {
  if let Ok(removable) = is_removable(file) {
    if !removable {
      println!(
        "{} is not removable. Check file permissions of parent folder?",
        file
      );
    }
  }
}

pub fn handle_media(original_file: &str, dest_dir: &str) {
  let mut original_file_path_buf: PathBuf = PathBuf::new();

  original_file_path_buf.push(original_file);

  if let Some(file_name) = original_file_path_buf.file_name() {
    if let Some(file_name_to_str) = file_name.to_str() {
      let mut owned_dest_string: String = dest_dir.to_owned();
      let copy_env = env::var("COPY").expect("COPY not set");

      owned_dest_string.push_str("/");
      owned_dest_string.push_str(file_name_to_str);

      if copy_env == "true" {
        println!("{} => {}", original_file, owned_dest_string);
        match copy(original_file, owned_dest_string) {
          Ok(_e) => (),
          Err(_) => handle_if_removable(original_file),
        };
      } else {
        match rename(original_file, owned_dest_string) {
          Ok(_e) => (),
          Err(_) => handle_if_removable(original_file),
        };
      }
    }
  }
}
