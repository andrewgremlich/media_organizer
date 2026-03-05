mod make_dir_str;
use make_dir_str::{make_audio_dir_str, make_doc_dir_str, make_photo_dir_str, make_video_dir_str};

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
    vec!["mp3", "MP3", "wav", "WAV", "aiff", "AIFF", "m4a", "M4A", "flac", "FLAC"]
}

fn get_white_list_doc_types<'a>() -> Vec<&'a str> {
    vec![
        "docx", "DOCX", "doc", "DOC", "pdf", "PDF", "epub", "EPUB", "mobi", "MOBI",
        "odt", "ODT", "txt", "TXT", "md", "MD", "rtf", "RTF",
    ]
}

fn contains_type(types: Vec<&str>, name: &str) -> bool {
    for file_type in types {
        if name.contains(file_type) {
            return true;
        }
    }

    false
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

fn is_document(file_name: &str) -> bool {
    contains_type(get_white_list_doc_types(), file_name)
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
    if is_document(file_name) {
        return Ok(make_doc_dir_str(file_name));
    }

    Err(format!("'{}', File type not supported", file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    // sort_and_make type detection tests
    #[test]
    fn sort_and_make_detects_video_mp4() {
        let result = sort_and_make("video.mp4");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_video_mov() {
        let result = sort_and_make("video.MOV");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_photo_jpg() {
        let result = sort_and_make("photo.jpg");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_photo_heic() {
        let result = sort_and_make("photo.HEIC");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_audio_mp3() {
        let result = sort_and_make("song.mp3");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_audio_flac() {
        let result = sort_and_make("song.FLAC");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_pdf() {
        let result = sort_and_make("file.pdf");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_docx() {
        let result = sort_and_make("file.DOCX");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_epub() {
        let result = sort_and_make("book.epub");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_txt() {
        let result = sort_and_make("notes.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_md() {
        let result = sort_and_make("readme.md");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_rtf() {
        let result = sort_and_make("doc.RTF");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_detects_doc_odt() {
        let result = sort_and_make("doc.odt");
        assert!(result.is_ok());
    }

    #[test]
    fn sort_and_make_rejects_unsupported_type() {
        let result = sort_and_make("file.xyz");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("File type not supported"));
    }

    #[test]
    fn sort_and_make_rejects_no_extension() {
        let result = sort_and_make("justfilename");
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
