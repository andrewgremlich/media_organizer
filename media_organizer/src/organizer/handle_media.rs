use faccess::{AccessMode, PathExt};
use media_info::{read_photo_dimensions, read_video_dimensions};
use std::env;
use std::fs::{copy, rename};
use std::path::Path;
use std::path::PathBuf;

fn handle_if_removable(file: &str) {
    let file_path = Path::new(file);

    if !file_path.exists() {
        println!("The {} file does not exist.", file);
        return;
    }

    match file_path.parent().unwrap().access(AccessMode::WRITE) {
        Ok(_) => println!("The file is likely deletable."),
        Err(e) => println!("The file might not be deletable. Error: {}", e),
    }
}

fn get_dimensions_suffix(original_file: &str) -> Option<String> {
    let path = Path::new(original_file);
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let ext_lower = ext.to_lowercase();

    let dims = match ext_lower.as_str() {
        "jpg" | "jpeg" | "png" | "tiff" | "heif" | "heic" | "avif" | "webp" => {
            read_photo_dimensions(path).ok()
        }
        "mp4" | "mov" => read_video_dimensions(path).ok(),
        _ => None,
    };

    dims.map(|(w, h)| format!("_{}x{}", w, h))
}

fn apply_dimensions_to_filename(file_name: &str, original_file: &str) -> String {
    let dimensions_env = env::var("DIMENSIONS").unwrap_or("false".to_string());
    if dimensions_env != "true" {
        return file_name.to_string();
    }

    if let Some(suffix) = get_dimensions_suffix(original_file) {
        let path = Path::new(file_name);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(file_name);
        let ext = path.extension().and_then(|e| e.to_str());

        match ext {
            Some(e) => format!("{}{}.{}", stem, suffix, e),
            None => format!("{}{}", stem, suffix),
        }
    } else {
        file_name.to_string()
    }
}

fn media_action(file_name: &str, dest_dir: &str, original_file: &str) {
    let mut owned_dest_string: String = dest_dir.to_owned();
    let copy_env = env::var("COPY").expect("COPY not set");
    owned_dest_string.push('/');
    owned_dest_string.push_str(file_name);

    if copy_env == "true" {
        match copy(original_file, owned_dest_string) {
            Ok(_e) => (),
            Err(_) => handle_if_removable(original_file),
        };
    } else {
        match rename(original_file, owned_dest_string) {
            Ok(_e) => (),
            Err(_) => handle_if_removable(original_file),
        };
    }
}

pub fn handle_media(original_file: &str, dest_dir: &str) {
    let mut original_file_path_buf: PathBuf = PathBuf::new();

    original_file_path_buf.push(original_file);

    match original_file_path_buf.file_name() {
        Some(file_name) => {
            let file_name_str = file_name
                .to_str()
                .expect("could not read filename from path buffer.");
            let dest_file_name = apply_dimensions_to_filename(file_name_str, original_file);
            media_action(&dest_file_name, dest_dir, original_file);
        }
        None => {
            println!("Could not get file name from path: {}", original_file);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_dimensions_disabled_returns_original() {
        unsafe {
            env::remove_var("DIMENSIONS");
        }
        // Use a non-image file so result is deterministic regardless of env race
        let result = apply_dimensions_to_filename("notes.txt", "../test-media/test.txt");
        assert_eq!(result, "notes.txt");
    }

    #[test]
    fn apply_dimensions_to_photo() {
        unsafe {
            env::set_var("DIMENSIONS", "true");
        }
        let result = apply_dimensions_to_filename(
            "400a861d-014a-4dfb-9143-1a914212fd4d.jpg",
            "../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg",
        );
        assert_eq!(result, "400a861d-014a-4dfb-9143-1a914212fd4d_3022x4030.jpg");
    }

    #[test]
    fn apply_dimensions_to_video() {
        unsafe {
            env::set_var("DIMENSIONS", "true");
        }
        let result = apply_dimensions_to_filename("corgi_race.mp4", "../test-media/corgi_race.mp4");
        assert!(result.contains("x"));
        assert!(result.starts_with("corgi_race_"));
        assert!(result.ends_with(".mp4"));
    }

    #[test]
    fn apply_dimensions_skips_audio() {
        unsafe {
            env::set_var("DIMENSIONS", "true");
        }
        let result = apply_dimensions_to_filename("Recording.m4a", "../test-media/Recording.m4a");
        assert_eq!(result, "Recording.m4a");
    }

    #[test]
    fn apply_dimensions_nonexistent_file_returns_original() {
        unsafe {
            env::set_var("DIMENSIONS", "true");
        }
        let result = apply_dimensions_to_filename("missing.jpg", "nonexistent.jpg");
        assert_eq!(result, "missing.jpg");
    }
}
