# Media Info

A tool to extract media information from a media source.

This is a supplemental library crate for
[Media Organizer](https://crates.io/crates/media_organizer).

## API

```rust
read_audio_creation_date(path_str: &Path) -> Result<String, String>

read_photo_creation_date(path_str: &Path) -> Result<String, String>

read_video_creation_date(path_str: &Path) -> Result<String, String>
```

```rust
struct AudioInfo {
  pub creation_date: String,
  pub artist: String,
  pub title: String,
  pub album: String,
  pub duration: String,
  pub released: String,
  pub genre: String,
}

struct VideoInfo {
  pub creation_date: String,
  pub major_brand: String,
  pub minor_version: String,
  pub compatible_brands: String,
  pub encoder: String,
  pub comment: String,
  pub duration_in_secs: i64,
  pub bit_rate: i64,

  pub location: String,
  pub make: String,
  pub model: String,
  pub software: String,
}

pub struct PhotoInfo {
  pub make: String,
  pub model: String,
  pub date_time: String,
  pub exposure_time: String,
  pub f_number: String,
  pub exposure_program: String,
  pub photographic_sensitivity: String,
  pub date_time_original: String,
  pub date_time_digitized: String,
  pub exposure_bias_value: String,
  pub focal_length: String,
  pub color_space: String,
  pub pixel_x_dimension: String,
  pub pixel_y_dimension: String,
  pub lens_make: String,
  pub lens_model: String,
  pub gps_latitude_ref: String,
  pub gps_latitude: String,
  pub gps_longitude_ref: String,
  pub gps_longitude: String,
  pub gps_altitude_ref: String,
  pub gps_altitude: String,
  pub gps_timestamp: String,
  pub gps_speed_ref: String,
  pub gps_speed: String,
  pub gps_img_direction_ref: String,
  pub gps_img_direction: String,
  pub gps_dest_bearing_ref: String,
  pub gps_dest_bearing: String,
  pub gps_date_stamp: String,
  pub gps_h_positioning_error: String,
}
```

### Examples

```rust
let raw_path_str = "../test-media/400a861d-014a-4dfb-9143-1a914212fd4d.jpg";
let path = Path::new(raw_path_str);
let photo_info = PhotoInfo::new(path).unwrap();

assert_eq!(photo_info.date_time.contains("2024-10-22"), true);

let raw_path_str = "../test-media/corgi_race.mp4";
let path = Path::new(raw_path_str);
let video_info = VideoInfo::new(path).unwrap();

assert_eq!(video_info.creation_date.contains("2024-10-20"), true);

let raw_path_str = "../test-media/Recording.m4a";
let path = Path::new(raw_path_str);
let audio_info = AudioInfo::new(path).unwrap();

assert_eq!(audio_info.creation_date.contains("2024-11-11"), true);
```
