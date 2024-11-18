use chrono::NaiveDate;
use id3::{Tag as ID3Tag, TagLike, Timestamp};

fn format_date(date: Timestamp) -> String {
    let year = date.year;
    let month = date.month.unwrap_or(1);
    let day = date.day.unwrap_or(1);

    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);
    let date_str = assembled_date.unwrap().format("%Y-%m-%d").to_string();

    return date_str;
}

pub fn get_date_recorded(tag: &ID3Tag) -> String {
    let date_recorded = tag.date_recorded().unwrap();
    format_date(date_recorded)
}

pub fn get_artist(tag: &ID3Tag) -> String {
    tag.artist().unwrap_or("Unknown Artist").to_string()
}

pub fn get_title(tag: &ID3Tag) -> String {
    tag.title().unwrap_or("Unknown Title").to_string()
}

pub fn get_album(tag: &ID3Tag) -> String {
    tag.album().unwrap_or("Unknown Album").to_string()
}

pub fn get_duration(tag: &ID3Tag) -> String {
    let duration = tag.duration().unwrap_or(0);
    let duration_str = format!("{} seconds", duration);

    duration_str
}

pub fn get_release_date(tag: &ID3Tag) -> String {
    let release_date = tag.date_released().unwrap_or_default();

    format_date(release_date)
}

pub fn get_genre(tag: &ID3Tag) -> String {
    tag.genre().unwrap_or("Unknown Genre").to_string()
}