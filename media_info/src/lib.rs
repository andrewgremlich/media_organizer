#[cfg(feature = "audio")]
pub mod audio_info;
#[cfg(feature = "photo")]
pub mod photo_info;
#[cfg(feature = "video")]
pub mod video_info;

#[cfg(feature = "audio")]
pub use audio_info::*;
#[cfg(feature = "photo")]
pub use photo_info::*;
#[cfg(feature = "video")]
pub use video_info::*;