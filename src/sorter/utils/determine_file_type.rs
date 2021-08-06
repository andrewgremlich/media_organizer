fn get_white_list_video_types<'a>() -> Vec<&'a str> {
  vec!["mp4", "MP4", "mov", "MOV"]
}

fn get_white_list_photo_types<'a>() -> Vec<&'a str> {
  vec!["jpeg", "jpg", "JPEG", "JPG", "HEIC", "heic", "PNG", "png"]
}

pub fn get_white_list_file_types<'a>() -> Vec<&'a str> {
  let mut combined_file_types: Vec<&str> = vec![];
  let mut video_types: Vec<&str> = get_white_list_video_types();
  let mut photo_types: Vec<&str> = get_white_list_photo_types();

  combined_file_types.append(&mut video_types);
  combined_file_types.append(&mut photo_types);

  return combined_file_types;
}

fn contains_type(types: Vec<&str>, name: &str) -> bool {
  let mut boolean: bool = false;

  for file_type in types {
    if name.contains(file_type) {
      boolean = true;
      break;
    }
  }

  return boolean;
}

pub fn is_video(file_name: &str) -> bool {
  contains_type(get_white_list_video_types(), file_name)
}

pub fn is_photo(file_name: &str) -> bool {
  contains_type(get_white_list_photo_types(), file_name)
}
