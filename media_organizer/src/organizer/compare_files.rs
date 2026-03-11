use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

/// Byte-compares two provided file `Path`s.
///
/// Function is only crate-public and returns `true` if both files have same length and are byte-identical.
///
/// # Errors
///
/// Returns `io:Error` if opening or reading any of the files fails.
pub(crate) fn compare_files<P: AsRef<Path>>(file1_path: P, file2_path: P) -> Result<bool, io::Error> {
    // Get metadata for both files
    let metadata1 = std::fs::metadata(&file1_path)?;
    let metadata2 = std::fs::metadata(&file2_path)?;

    // Compare file sizes
    if metadata1.len() != metadata2.len() {
        return Ok(false);
    }

    // Compare file contents
    let mut file1 = File::open(file1_path)?;
    let mut file2 = File::open(file2_path)?;
    let mut buffer1 = [0; 8192];
    let mut buffer2 = [0; 8192];

    loop {
        let bytes_read1 = file1.read(&mut buffer1)?;
        let bytes_read2 = file2.read(&mut buffer2)?;

        if bytes_read1 != bytes_read2 {
            return Ok(false); // Different file sizes
        }

        if bytes_read1 == 0 {
            break; // Reached the end of both files
        }

        if buffer1[..bytes_read1] != buffer2[..bytes_read2] {
            return Ok(false); // File contents differ
        }
    }

    Ok(true)
}
