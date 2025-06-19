use epub::doc::EpubDoc;
use mobi::Mobi;
use std::path::Path;

pub fn read_epub_info(path: &Path) {
    let doc = EpubDoc::new(path).unwrap();
    let modified = doc.mdata("dcterms:modified").unwrap();

    println!("Modified: {:?}", modified);
}

pub fn read_mobi_info(path: &Path) {
    let mobi = Mobi::from_path(path).unwrap_or_default();
    let modified = mobi.publish_date().unwrap_or_default();

    println!("Modified: {:?}", modified);
}
