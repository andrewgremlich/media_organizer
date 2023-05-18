mod make_dir_str;
use make_dir_str::{make_audio_dir_str, make_photo_dir_str, make_video_dir_str};

fn get_white_list_video_types<'a>() -> Vec<&'a str> {
    vec!["mp4", "MP4", "mov", "MOV"]
}

fn get_white_list_photo_types<'a>() -> Vec<&'a str> {
    vec![
        "tiff", "TIFF", "heif", "HEIF", "HEIC", "heic", "AVIF", "avif", "jpeg", "jpg", "JPEG",
        "JPG", "HEIC", "heic", "PNG", "png", "webp", "WEBP",
    ]
}

fn get_white_list_audio_types<'a>() -> Vec<&'a str> {
    vec!["mp3", "MP3", "wav", "WAV", "aiff", "AIFF"]
}

fn contains_type(types: Vec<&str>, name: &str) -> bool {
    for file_type in types {
        if name.contains(file_type) {
            return true;
        }
    }

    return false;
}

fn is_video(file_name: &str) -> bool {
    contains_type(get_white_list_video_types(), file_name)
}

fn is_photo(file_name: &str) -> bool {
    contains_type(get_white_list_photo_types(), file_name)
}

fn is_audio(file_name: &str) -> bool {
    contains_type(get_white_list_audio_types(), file_name)
}

pub fn sort_and_make(file_name: &str) -> Result<String, String> {
    if is_video(file_name) {
        return Ok(make_video_dir_str(file_name));
    }
    if is_photo(file_name) {
        return Ok(make_photo_dir_str(file_name));
    }
    if is_audio(file_name) {
        return Ok(make_audio_dir_str(file_name));
    }

    return Err(String::from("File type not supported"));
}
