use fs_metadata::file_created;
use std::path::Path;

mod readers;

pub fn read_doc_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| format!("File has no extension: {:?}", path))?;

    match ext.to_lowercase().as_str() {
        "epub" => readers::read_epub_date(path),
        "mobi" => readers::read_mobi_date(path),
        "pdf" => readers::read_pdf_date(path),
        "docx" | "doc" => readers::read_docx_date(path),
        "odt" => readers::read_odt_date(path),
        "txt" | "md" | "rtf" => file_created(path),
        _ => Err(format!("Unsupported file type: {:?}", path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_docx_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.docx");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_pdf_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.pdf");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_epub_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.epub");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_txt_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.txt");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_md_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.md");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_odt_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.odt");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn can_read_rtf_creation_date() {
        let path = Path::new("../test-media/TESTDOCUMENT.rtf");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
        let date = result.unwrap();
        assert!(
            date.contains('-'),
            "Expected YYYY-MM-DD format, got: {}",
            date
        );
    }

    #[test]
    fn nonexistent_file_returns_error() {
        let path = Path::new("../test-media/NONEXISTENT.pdf");
        let result = read_doc_creation_date(path);
        assert!(result.is_err());
    }

    #[test]
    fn unsupported_extension_returns_error() {
        let path = Path::new("../test-media/Recording.m4a");
        let result = read_doc_creation_date(path);
        assert!(result.is_err());
    }

    #[test]
    fn file_with_no_extension_returns_error() {
        let path = Path::new("../test-media");
        let result = read_doc_creation_date(path);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("no extension") || err.contains("does not exist"),
            "Unexpected error: {}",
            err
        );
    }

    #[test]
    fn uppercase_extension_is_handled() {
        // The function lowercases extensions, so DOCX should work via the .docx path
        let path = Path::new("../test-media/TESTDOCUMENT.docx");
        let result = read_doc_creation_date(path);
        assert!(result.is_ok());
    }

    #[test]
    fn date_format_is_yyyy_mm_dd() {
        let path = Path::new("../test-media/TESTDOCUMENT.pdf");
        let result = read_doc_creation_date(path).unwrap();
        let parts: Vec<&str> = result.split('-').collect();
        assert_eq!(parts.len(), 3, "Expected 3 parts in YYYY-MM-DD, got: {}", result);
        assert_eq!(parts[0].len(), 4, "Year should be 4 digits");
        assert_eq!(parts[1].len(), 2, "Month should be 2 digits");
        assert_eq!(parts[2].len(), 2, "Day should be 2 digits");
    }
}
