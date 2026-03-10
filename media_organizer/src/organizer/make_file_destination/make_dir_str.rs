use crate::organizer::make_file_destination::MakeFileDestinationError;
use media_info::{
    read_audio_creation_date, read_doc_creation_date, read_photo_creation_date,
    read_video_creation_date,
};
use std::env;
use std::path::Path;

fn make_dir_string(date: &str) -> String {
    let replace_date_hyphens = str::replace(date, "-", &std::path::MAIN_SEPARATOR.to_string());
    let dest_folder = env::var("DEST_FOLDER").expect("DEST_FOLDER not set");
    // Normalize so output uses platform separator even if DEST_FOLDER was set with / or \.
    let dest_folder = str::replace(
        &dest_folder,
        if std::path::MAIN_SEPARATOR == '\\' { "/" } else { "\\" },
        &std::path::MAIN_SEPARATOR.to_string(),
    );
    let mut regular_date_folder: String = String::new();

    regular_date_folder.push_str(&dest_folder);
    regular_date_folder.push(std::path::MAIN_SEPARATOR);
    regular_date_folder.push_str(&replace_date_hyphens);

    regular_date_folder
}

pub(crate) fn make_photo_dir_str(date: &str) -> Result<String, MakeFileDestinationError> {
    match read_photo_creation_date(Path::new(date)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

pub(crate) fn make_video_dir_str(date: &str) -> Result<String, MakeFileDestinationError> {
    match read_video_creation_date(Path::new(date)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

pub(crate) fn make_audio_dir_str(dir_str: &str) -> Result<String, MakeFileDestinationError> {
    match read_audio_creation_date(Path::new(dir_str)) {
        Ok(date) => Ok(make_dir_string(date.as_str())),
        Err(error) => Err(MakeFileDestinationError::Error(error)),
    }
}

pub(crate) fn make_doc_dir_str(dir_str: &str) -> String {
    let doc_date =
        read_doc_creation_date(Path::new(dir_str)).unwrap_or("no_date_found".to_string());

    make_dir_string(&doc_date)
}

#[cfg(test)]
pub mod date_read_tests {
    use super::*;

    #[test]
    fn can_read_photo_creation_date() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let path_str = &format!(
            "..{}test-media{0}400a861d-014a-4dfb-9143-1a914212fd4d.jpg",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_photo_creation_date(Path::new(path_str)) {
            Ok(date_of_photo) => make_dir_string(date_of_photo.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}10{0}22", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_video_creation_date() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let path_str = &format!(
            "..{}test-media{0}corgi_race.mp4",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_video_creation_date(Path::new(path_str)) {
            Ok(date_of_video) => make_dir_string(date_of_video.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}10{0}20", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_audio_creation_date() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let path_str = &format!(
            "..{}test-media{0}Recording.m4a",
            std::path::MAIN_SEPARATOR
        );

        let date_info = match read_audio_creation_date(Path::new(path_str)) {
            Ok(date_of_audio) => make_dir_string(date_of_audio.as_str()),
            Err(err) => panic!("Test failed because of error: {}", err),
        };

        assert_eq!(
            format!("tests{}test_files{0}2024{0}11{0}11", std::path::MAIN_SEPARATOR),
            date_info
        );
    }

    #[test]
    fn can_read_doc_creation_date() {
        let sep = std::path::MAIN_SEPARATOR;
        unsafe {
            env::set_var("DEST_FOLDER", &format!("tests{}test_files", sep));
        }

        let doc_date = read_doc_creation_date(Path::new("../test-media/TESTDOCUMENT.docx"))
            .unwrap_or("no_date_found".to_string());
        let date_info = make_dir_string(&doc_date);

        let expected_prefix = format!("tests{}test_files{}", sep, sep);
        assert!(
            date_info.starts_with(&expected_prefix),
            "Expected path to start with dest folder, got: {}",
            date_info
        );
        assert_ne!(
            date_info,
            format!("tests{}test_files{}no_date_found", sep, sep),
            "Should extract a real date, not fallback"
        );
    }

    #[test]
    fn make_dir_string_with_no_date_found() {
        let sep = std::path::MAIN_SEPARATOR;
        unsafe {
            env::set_var("DEST_FOLDER", &format!("tests{}test_files", sep));
        }

        let date_info = make_dir_string("no_date_found");
        assert_eq!(
            format!("tests{}test_files{}no_date_found", sep, sep),
            date_info
        );
    }

    #[test]
    fn make_doc_dir_str_with_nonexistent_file() {
        let sep = std::path::MAIN_SEPARATOR;
        unsafe {
            env::set_var("DEST_FOLDER", &format!("tests{}test_files", sep));
        }

        let result = make_doc_dir_str("nonexistent.pdf");
        assert_eq!(
            format!("tests{}test_files{}no_date_found", sep, sep),
            result
        );
    }

    #[test]
    fn make_photo_dir_str_with_nonexistent_file() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let result = make_photo_dir_str("nonexistent.jpg");
        assert!(result.is_err());
    }

    #[test]
    fn make_video_dir_str_with_nonexistent_file() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let result = make_video_dir_str("nonexistent.mp4");
        assert!(result.is_err());
    }

    #[test]
    fn make_audio_dir_str_with_nonexistent_file() {
        unsafe {
            env::set_var(
                "DEST_FOLDER",
                &format!("tests{}test_files", std::path::MAIN_SEPARATOR),
            );
        }

        let result = make_audio_dir_str("nonexistent.mp3");
        assert!(result.is_err());
    }
}
