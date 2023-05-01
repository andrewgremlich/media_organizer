fn get_white_list_video_types<'a>() -> Vec<&'a str> {
    vec!["mp4", "MP4", "mov", "MOV"]
}

fn get_white_list_photo_types<'a>() -> Vec<&'a str> {
    vec!["jpeg", "jpg", "JPEG", "JPG", "HEIC", "heic", "PNG", "png"]
}

fn get_white_list_audio_types<'a>() -> Vec<&'a str> {
    vec!["mp3", "MP3", "wav", "WAV"]
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

pub fn is_audio(file_name: &str) -> bool {
    contains_type(get_white_list_audio_types(), file_name)
}
