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

fn media_action(file_name: &str, dest_dir: &str, original_file: &str) {
  let mut owned_dest_string: String = dest_dir.to_owned();
  let copy_env = env::var("COPY").expect("COPY not set");
  owned_dest_string.push_str("/");
  owned_dest_string.push_str(file_name);

  if copy_env == "true" {
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

pub fn handle_media(original_file: &str, dest_dir: &str) {
  let mut original_file_path_buf: PathBuf = PathBuf::new();

  original_file_path_buf.push(original_file);

  match original_file_path_buf.file_name() {
    Some(file_name) => {
      media_action(
        file_name
          .to_str()
          .expect("could not read filename from path buffer."),
        dest_dir,
        original_file,
      );
    }
    None => {
      println!("Could not get file name from path: {}", original_file);
      return;
    }
  }
}
