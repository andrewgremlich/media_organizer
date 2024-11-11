use media_info::{read_audio_creation_date, read_photo_creation_date, read_video_creation_date};
use std::env;

fn make_dir_string(date_time: &str) -> String {
    let replace_date_hyphens = str::replace(date_time, "-", "/");
    let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
    let mut regular_date_folder: String = String::new();

    regular_date_folder.push_str("./");
    regular_date_folder.push_str(&dest_folder);
    regular_date_folder.push('/');
    regular_date_folder.push_str(&replace_date_hyphens);

    regular_date_folder
}

pub fn make_photo_dir_str(dir_str: &str) -> String {
    let photo_date = read_photo_creation_date(dir_str);

    make_dir_string(photo_date.split_whitespace().next().unwrap_or_default())
}

pub fn make_video_dir_str(dir_str: &str) -> String {
    let date = read_video_creation_date(dir_str);

    make_dir_string(date.split('T').next().unwrap_or_default())
}

pub fn make_audio_dir_str(dir_str: &str) -> String {
    let audio_date = read_audio_creation_date(dir_str);

    make_dir_string(&audio_date)
}

#[cfg(test)]
pub mod date_read_tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        env::set_var("DEST_FOLDER", &"tests/test_files");

        let path_str = "tests/test_files/test_photo.JPG";
        let photo_date = read_photo_creation_date(path_str);
        let date_info = make_dir_string(photo_date.split_whitespace().next().unwrap_or_default());

        assert_eq!("./tests/test_files/2020/02/01", date_info);
    }

    #[test]
    fn can_read_video_creation_date() {
        env::set_var("DEST_FOLDER", &"tests/test_files");

        let path_str = "tests/test_files/test_video.mp4";
        let video_date = read_video_creation_date(path_str);
        let date_info = make_dir_string(video_date.split('T').next().unwrap_or_default());

        assert_eq!("./tests/test_files/2021/05/21", date_info);
    }
}
