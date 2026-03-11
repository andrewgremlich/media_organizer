use chrono::NaiveDate;
use epub::doc::EpubDoc;
use fs_metadata::file_created;
use mobi::Mobi;
use std::io::Read;
use std::path::Path;

fn parse_date_to_ymd(date_str: &str) -> Option<String> {
    // Try ISO 8601: 2024-01-15T10:30:00Z or 2024-01-15
    if let Ok(date) = NaiveDate::parse_from_str(&date_str[..10], "%Y-%m-%d") {
        return Some(date.format("%Y-%m-%d").to_string());
    }
    // Try YYYY/MM/DD
    if let Ok(date) = NaiveDate::parse_from_str(&date_str[..10], "%Y/%m/%d") {
        return Some(date.format("%Y-%m-%d").to_string());
    }
    // Try just a year like "2024"
    if date_str.len() == 4
        && let Ok(year) = date_str.parse::<i32>()
        && let Some(date) = NaiveDate::from_ymd_opt(year, 1, 1)
    {
        return Some(date.format("%Y-%m-%d").to_string());
    }
    None
}

pub fn read_epub_date(path: &Path) -> Result<String, String> {
    let doc = EpubDoc::new(path).map_err(|e| format!("Failed to open EPUB: {}", e))?;

    // Try multiple metadata keys in priority order
    let keys = ["dcterms:created", "dcterms:modified", "date"];
    for key in &keys {
        if let Some(value) = doc.mdata(key) {
            let s = value.value.as_str();
            if let Some(date) = parse_date_to_ymd(s) {
                return Ok(date);
            }
        }
    }

    file_created(path)
}

pub fn read_mobi_date(path: &Path) -> Result<String, String> {
    let mobi = Mobi::from_path(path).map_err(|e| format!("Failed to open MOBI: {}", e))?;

    if let Some(date_str) = mobi.publish_date()
        && let Some(date) = parse_date_to_ymd(&date_str)
    {
        return Ok(date);
    }

    file_created(path)
}

pub fn read_pdf_date(path: &Path) -> Result<String, String> {
    let file = pdf::file::FileOptions::cached()
        .open(path)
        .map_err(|e| format!("Failed to open PDF: {}", e))?;

    if let Some(ref info) = file.trailer.info_dict {
        // Try CreationDate first, then ModDate
        let dates = [&info.creation_date, &info.mod_date];
        for date in dates.iter().copied().flatten() {
            if let Some(naive) =
                NaiveDate::from_ymd_opt(date.year as i32, date.month as u32, date.day as u32)
            {
                return Ok(naive.format("%Y-%m-%d").to_string());
            }
        }
    }

    file_created(path)
}

pub fn read_docx_date(path: &Path) -> Result<String, String> {
    let file =
        std::fs::File::open(path).map_err(|e| format!("Failed to open DOCX: {}", e))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read DOCX as ZIP: {}", e))?;

    if let Ok(mut core_xml) = archive.by_name("docProps/core.xml") {
        let mut contents = String::new();
        core_xml
            .read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read core.xml: {}", e))?;

        // Look for dcterms:created or dcterms:modified
        if let Some(date) = extract_xml_date(&contents, "dcterms:created")
            .or_else(|| extract_xml_date(&contents, "dcterms:modified"))
        {
            return Ok(date);
        }
    }

    file_created(path)
}

pub fn read_odt_date(path: &Path) -> Result<String, String> {
    let file =
        std::fs::File::open(path).map_err(|e| format!("Failed to open ODT: {}", e))?;
    let mut archive =
        zip::ZipArchive::new(file).map_err(|e| format!("Failed to read ODT as ZIP: {}", e))?;

    if let Ok(mut meta_xml) = archive.by_name("meta.xml") {
        let mut contents = String::new();
        meta_xml
            .read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read meta.xml: {}", e))?;

        if let Some(date) = extract_xml_date(&contents, "meta:creation-date")
            .or_else(|| extract_xml_date(&contents, "dc:date"))
        {
            return Ok(date);
        }
    }

    file_created(path)
}

fn extract_xml_date(xml: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);

    let start = xml.find(&open_tag)?;
    let after_open = xml[start..].find('>')?;
    let content_start = start + after_open + 1;
    let end = xml[content_start..].find(&close_tag)?;
    let value = &xml[content_start..content_start + end];

    parse_date_to_ymd(value.trim())
}
