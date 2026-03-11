mod make_dir_str;

use crate::organizer::make_file_destination::make_dir_str::{
    make_audio_dir_str, make_doc_dir_str, make_photo_dir_str, make_video_dir_str,
};
use std::path::Path;

fn get_white_list_video_types<'a>() -> Vec<&'a str> {
    vec!["mp4", "MP4", "mov", "MOV", "avi", "AVI"]
}

fn get_white_list_photo_types<'a>() -> Vec<&'a str> {
    vec![
        "tiff", "TIFF", "heif", "HEIF", "HEIC", "heic", "AVIF", "avif", "jpeg", "jpg", "JPEG",
        "JPG", "HEIC", "heic", "PNG", "png", "webp", "WEBP", "dng", "DNG", "gif", "GIF", "raw", "RAW",
    ]
}

fn get_white_list_audio_types<'a>() -> Vec<&'a str> {
    vec!["mp3", "MP3", "wav", "WAV", "aiff", "AIFF", "m4a", "M4A", "flac", "FLAC"]
}

fn get_white_list_doc_types<'a>() -> Vec<&'a str> {
    vec![
        "docx", "DOCX", "doc", "DOC", "pdf", "PDF", "epub", "EPUB", "mobi", "MOBI",
        "odt", "ODT", "txt", "TXT", "md", "MD", "rtf", "RTF",
    ]
}

fn ends_with_type(types: Vec<&str>, name: &str) -> bool {
    for file_type in types {
        if name.ends_with(file_type) {
            return true;
        }
    }

    false
}

fn is_video(file_name: &str) -> bool {
    ends_with_type(get_white_list_video_types(), file_name)
}

fn is_photo(file_name: &str) -> bool {
    ends_with_type(get_white_list_photo_types(), file_name)
}

fn is_audio(file_name: &str) -> bool {
    ends_with_type(get_white_list_audio_types(), file_name)
}

fn is_document(file_name: &str) -> bool {
    ends_with_type(get_white_list_doc_types(), file_name)
}

#[derive(Debug)]
pub enum MakeFileDestinationError {
    UnsupportedType(String),
    Error(String),
}

pub fn make_file_destination_str(file_name: &str) -> Result<String, MakeFileDestinationError> {
    if is_video(file_name) {
        return make_video_dir_str(file_name);
    }
    if is_photo(file_name) {
        return make_photo_dir_str(file_name);
    }
    if is_audio(file_name) {
        return make_audio_dir_str(file_name);
    }
    if is_document(file_name) {
        return Ok(make_doc_dir_str(file_name));
    }

    let path = Path::new(file_name);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();
    Err(MakeFileDestinationError::UnsupportedType(ext))
}

#[cfg(test)]
mod tests {
    use super::*;

    // make_file_destination_str type detection tests (file may be missing → Err(Error) is ok, UnsupportedType is not)
    #[test]
    fn sort_and_make_detects_video_mp4() {
        let result = make_file_destination_str("video.mp4");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "MP4 should be detected as video, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_video_mov() {
        let result = make_file_destination_str("video.MOV");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "MOV should be detected as video, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_photo_jpg() {
        let result = make_file_destination_str("photo.jpg");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "JPG should be detected as photo, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_photo_heic() {
        let result = make_file_destination_str("photo.HEIC");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "HEIC should be detected as photo, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_audio_mp3() {
        let result = make_file_destination_str("song.mp3");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "MP3 should be detected as audio, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_audio_flac() {
        // Detection test: FLAC must be recognized as audio (no UnsupportedType).
        // Err(Error(_)) is acceptable when the path does not point to an existing file.
        let result = make_file_destination_str("song.FLAC");
        assert!(
            result.is_ok() || !matches!(&result, Err(MakeFileDestinationError::UnsupportedType(_))),
            "FLAC should be detected as audio, got: {:?}",
            result
        );
    }

    #[test]
    fn sort_and_make_detects_doc_pdf() {
        let result = make_file_destination_str("file.pdf");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_docx() {
        let result = make_file_destination_str("file.DOCX");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_epub() {
        let result = make_file_destination_str("book.epub");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_txt() {
        let result = make_file_destination_str("notes.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_md() {
        let result = make_file_destination_str("readme.md");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_rtf() {
        let result = make_file_destination_str("doc.RTF");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_odt() {
        let result = make_file_destination_str("doc.odt");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_rejects_unsupported_type() {
        let result = make_file_destination_str("file.xyz");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MakeFileDestinationError::UnsupportedType(_)
        ));
    }

    #[test]
    fn sort_and_make_rejects_no_extension() {
        let result = make_file_destination_str("justfilename");
        assert!(result.is_err());
    }

    // is_* function edge case tests
    #[test]
    fn is_video_false_for_non_video() {
        assert!(!is_video("photo.jpg"));
    }

    #[test]
    fn is_photo_false_for_non_photo() {
        assert!(!is_photo("video.mp4"));
    }

    #[test]
    fn is_audio_false_for_non_audio() {
        assert!(!is_audio("photo.jpg"));
    }

    #[test]
    fn is_document_false_for_non_doc() {
        assert!(!is_document("video.mp4"));
    }

    #[test]
    fn contains_type_with_path() {
        // sort_and_make receives full paths, ensure extension matching works with paths
        assert!(is_photo("/some/path/to/photo.jpg"));
        assert!(is_video("/some/path/to/video.mp4"));
        assert!(is_audio("/some/path/to/song.m4a"));
        assert!(is_document("/some/path/to/doc.pdf"));
    }
}

