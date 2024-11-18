use media_info::{read_audio_creation_date, read_photo_creation_date, read_video_creation_date};
use std::env;
use std::path::Path;

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
    let photo_date = read_photo_creation_date(Path::new(dir_str)).unwrap_or("no_date_found".to_string());

    make_dir_string(photo_date.split_whitespace().next().unwrap_or_default())
}

pub fn make_video_dir_str(dir_str: &str) -> String {
    let date = read_video_creation_date(Path::new(dir_str)).unwrap_or("no_date_found".to_string());

    make_dir_string(date.split('T').next().unwrap_or_default())
}

pub fn make_audio_dir_str(dir_str: &str) -> String {
    let audio_date = read_audio_creation_date(Path::new(dir_str)).unwrap_or("no_date_found".to_string());

    make_dir_string(&audio_date)
}

#[cfg(test)]
pub mod date_read_tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        env::set_var("DEST_FOLDER", &"tests/test_files");

        let photo_date = read_photo_creation_date(Path::new("../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg")).unwrap_or("no_date_found".to_string());
        let date_info = make_dir_string(photo_date.split_whitespace().next().unwrap_or_default());

        assert_eq!("./tests/test_files/2024/10/22", date_info);
    }

    #[test]
    fn can_read_video_creation_date() {
        env::set_var("DEST_FOLDER", &"tests/test_files");

        let video_date = read_video_creation_date(Path::new("../test-media/corgi_race.mp4")).unwrap_or("no_date_found".to_string());
        let date_info = make_dir_string(video_date.split('T').next().unwrap_or_default());

        assert_eq!("./tests/test_files/2024/10/20", date_info);
    }

    #[test]
    fn can_read_audio_creation_date() {
        env::set_var("DEST_FOLDER", &"tests/test_files");

        let audio_date = read_audio_creation_date(Path::new("../test-media/Recording.m4a")).unwrap_or("no_date_found".to_string());
        let date_info = make_dir_string(&audio_date);

        assert_eq!("./tests/test_files/2024/11/11", date_info);
    }
}
