use std::path::Path;

mod doc_info;

pub fn read_doc_creation_date(path: &Path) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("File does not exist: {:?}", path));
    }

    match path.extension() {
        Some(ext) => {
            let ext = ext.to_str().unwrap();

            match ext {
                "doc" => Ok("doc".to_string()),
                "docx" => Ok("docx".to_string()),
                "pdf" => Ok("pdf".to_string()),
                "mobi" => {
                    doc_info::read_mobi_info(path);
                    return Ok("mobi".to_string());
                }
                "epub" => {
                    doc_info::read_epub_info(path);
                    return Ok("epub".to_string());
                }
                "txt" => Ok("txt".to_string()),
                "md" => Ok("md".to_string()),
                "odt" => Ok("odt".to_string()),
                "rtf" => Ok("rtf".to_string()),
                _ => {
                    return Err(format!("Unsupported file type: {:?}", path));
                }
            }
        }
        None => {
            println!("Error: {:?}", format!("File has no extension: {:?}", path));
            return Err(format!("File has no extension: {:?}", path));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_doc_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.docx";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_pdf_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.pdf";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_epub_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.epub";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_txt_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.txt";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_md_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.md";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_odt_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.odt";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }

    #[test]
    fn can_read_rtf_creation_date() {
        let raw_path_str = "../test-media/TESTDOCUMENT.rtf";
        let path = Path::new(raw_path_str);
        let creation_date = read_doc_creation_date(&path);

        println!("Creation date: {:?}", creation_date.unwrap());
    }
}
