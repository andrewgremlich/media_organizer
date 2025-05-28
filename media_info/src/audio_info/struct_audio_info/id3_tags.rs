use chrono::NaiveDate;
use id3::{Tag as ID3Tag, TagLike, Timestamp};

/// Formats a `Timestamp` into a `String` in the format "YYYY-MM-DD".
/// If the month or day is missing, defaults to 1.
///
/// # Arguments
///
/// * `date` - The `Timestamp` to format.
///
/// # Returns
///
/// A `String` representing the formatted date.
fn format_date(date: Timestamp) -> String {
    let year = date.year;
    let month = date.month.unwrap_or(1);
    let day = date.day.unwrap_or(1);

    let assembled_date = NaiveDate::from_ymd_opt(year, month as u32, day as u32);
    let date_str = assembled_date.unwrap().format("%Y-%m-%d").to_string();

    return date_str;
}

/// Retrieves the recorded date from an ID3 tag and formats it as a string.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
/// # Returns
/// A `String` representing the recorded date in "YYYY-MM-DD" format.
/// Panics if the date is not present in the tag.
pub fn get_date_recorded(tag: &ID3Tag) -> String {
    let date_recorded = tag.date_recorded().unwrap();
    format_date(date_recorded)
}

/// Retrieves the artist from an ID3 tag.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` containing the artist name, or "Unknown Artist" if not present.
pub fn get_artist(tag: &ID3Tag) -> String {
    tag.artist().unwrap_or("Unknown Artist").to_string()
}

/// Retrieves the title from an ID3 tag.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` containing the title, or "Unknown Title" if not present.
pub fn get_title(tag: &ID3Tag) -> String {
    tag.title().unwrap_or("Unknown Title").to_string()
}

/// Retrieves the album name from an ID3 tag.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` containing the album name, or "Unknown Album" if not present.
pub fn get_album(tag: &ID3Tag) -> String {
    tag.album().unwrap_or("Unknown Album").to_string()
}

/// Retrieves the duration from an ID3 tag and formats it as a string.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` representing the duration in seconds (e.g., "123 seconds").
/// Returns "0 seconds" if duration is not present.
pub fn get_duration(tag: &ID3Tag) -> String {
    let duration = tag.duration().unwrap_or(0);
    let duration_str = format!("{} seconds", duration);

    duration_str
}

/// Retrieves the release date from an ID3 tag and formats it as a string.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` representing the release date in "YYYY-MM-DD" format.
/// Returns "0000-01-01" if the release date is not present.
pub fn get_release_date(tag: &ID3Tag) -> String {
    let release_date = tag.date_released().unwrap_or_default();

    format_date(release_date)
}

/// Retrieves the genre from an ID3 tag.
///
/// # Arguments
///
/// * `tag` - Reference to the ID3 tag.
///
/// # Returns
///
/// A `String` containing the genre, or "Unknown Genre" if not present.
pub fn get_genre(tag: &ID3Tag) -> String {
    tag.genre().unwrap_or("Unknown Genre").to_string()
}