mod determine_file_type;

use std::env;
use std::fs::rename;
use std::path::PathBuf;

pub use determine_file_type::{is_photo, is_video};

pub enum DirString<'a> {
  DateBreakdown(Option<&'a str>),
  RegularStr(String),
}

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

fn finally_make_date_str(appender: String) -> String {
  let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
  let mut regular_date_folder: String = String::new();

  regular_date_folder.push_str("./");
  regular_date_folder.push_str(&dest_folder);
  regular_date_folder.push_str("/");
  regular_date_folder.push_str(&appender);

  regular_date_folder
}

fn handle_date_string_breakdown(breakdown: Option<&str>) -> String {
  match breakdown {
    Some(e) => {
      let replace_date_hyphens = str::replace(e, "-", "/");
      finally_make_date_str(replace_date_hyphens)
    }
    None => String::from("No dates exist"),
  }
}

pub fn make_dir_string(date_time: DirString) -> String {
  match date_time {
    DirString::DateBreakdown(breakdown) => handle_date_string_breakdown(breakdown),
    DirString::RegularStr(reg_string) => finally_make_date_str(String::from(reg_string)),
  }
}
