use std::fs;
use std::io;
use std::path::Path;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Storage::FileSystem::{CreateFileW, SetFileTime, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL};
use windows::Win32::Foundation::{CloseHandle, FILETIME};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use windows::core::PCWSTR;

// Helper function to convert SystemTime to FILETIME (Windows format)
fn system_time_to_filetime(system_time: SystemTime) -> FILETIME {
    // Calculate the duration since the Unix epoch (January 1, 1970)
    let duration_since_unix_epoch = system_time.duration_since(UNIX_EPOCH).expect("SystemTime is before the Unix epoch");
    // Add the offset between Unix epoch and FILETIME epoch (January 1, 1601)
    let unix_epoch_offset = Duration::from_secs(11644473600);
    let duration_since_filetime_epoch = duration_since_unix_epoch + unix_epoch_offset;

    // Convert to 100-nanosecond intervals
    let intervals = duration_since_filetime_epoch.as_nanos() / 100;

    // Split the 64-bit interval value into high and low parts for FILETIME
    FILETIME {
        dwLowDateTime: intervals as u32,
        dwHighDateTime: (intervals >> 32) as u32,
    }
}

#[allow(unused)]
fn system_time_to_human_readable(system_time: SystemTime) -> String {
    // Get the duration since the Unix epoch
    let duration = system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // Convert the duration to seconds and nanoseconds
    let datetime = DateTime::<Utc>::from(UNIX_EPOCH + duration);

    // Format the datetime in a readable form (ISO 8601 format, e.g., "2024-09-12T14:12:37Z")
    datetime.to_rfc3339()
}

// Helper function to convert FILETIME to SystemTime
fn filetime_to_system_time(filetime: FILETIME) -> SystemTime {
    // FILETIME represents the number of 100-nanosecond intervals since January 1, 1601 (UTC).
    let intervals = ((filetime.dwHighDateTime as u64) << 32) | filetime.dwLowDateTime as u64;

    // Convert 100-nanosecond intervals to seconds and nanoseconds
    let nanos_since_1601 = intervals * 100;
    let duration_since_1601 = Duration::from_nanos(nanos_since_1601);

    // January 1, 1601 is 11644473600 seconds before Unix epoch (January 1, 1970).
    let duration_since_unix_epoch = duration_since_1601
        .checked_sub(Duration::from_secs(11644473600))
        .expect("Overflow occurred");

    // Return the corresponding SystemTime
    UNIX_EPOCH + duration_since_unix_epoch
}

// Function to convert FILETIME to a human-readable string
#[allow(unused)]
fn filetime_to_human_readable(filetime: FILETIME) -> String {
    let system_time = filetime_to_system_time(filetime);

    // Convert SystemTime to a human-readable format using chrono
    let duration = system_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let datetime = DateTime::<Utc>::from(UNIX_EPOCH + duration);

    datetime.to_rfc3339()
}

// Helper function to set the file creation, access, and modification times for a file on Windows
fn set_file_times_windows(path: &Path, creation_time: SystemTime, access_time: SystemTime, modification_time: SystemTime) -> io::Result<()> {
    // Convert `Path` to wide string for Windows API
    let wide_path: Vec<u16> = OsStr::new(path).encode_wide().chain(Some(0)).collect();

    // Open the file
    let handle: HANDLE = unsafe {
        CreateFileW(
            PCWSTR::from_raw(wide_path.as_ptr()),
            FILE_GENERIC_WRITE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            None,
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            HANDLE(0u32 as _),
        )
    }?;

    // Convert SystemTime to FILETIME
    let creation_time_ft = system_time_to_filetime(creation_time);
    let access_time_ft = system_time_to_filetime(access_time);
    let modification_time_ft = system_time_to_filetime(modification_time);

    // Set the file times
    unsafe {
        SetFileTime(
            handle,
            Some(&creation_time_ft),
            Some(&access_time_ft),
            Some(&modification_time_ft),
        )?;
    }

    // Close the file handle
    unsafe {
        CloseHandle(handle)?;
    }

    Ok(())
}

pub(crate) fn copy_file_metadata<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    // Step 2: Get metadata from the source file
    let metadata = fs::metadata(src)?;

    // Step 3: Retrieve the creation, access, and modification times
    let creation_time = metadata.modified()?;
    let access_time = metadata.accessed()?;
    let modification_time = metadata.modified()?;

    // Step 4: Set the metadata on the destination file (Windows only)
    set_file_times_windows(dst.as_ref(), creation_time, access_time, modification_time)?;

    Ok(())
}
