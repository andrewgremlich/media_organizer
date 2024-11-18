mod video_info;
mod audio_info;
mod photo_info;

#[cfg(feature = "audio")]
pub use audio_info::read_audio_creation_date;

#[cfg(feature = "photo")]
pub use photo_info::{read_photo_creation_date, PhotoInfo};

#[cfg(feature = "video")]
pub use video_info::{read_video_creation_date, VideoInfo};
