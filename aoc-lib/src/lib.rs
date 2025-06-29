#[macro_use]
pub mod macros;

use std::fs;
use std::path::Path;

/// Loads the contents of a UTF-8 data file into a `String`.
///
/// Load data files at runtime.
/// Exits with an error if the file can't be read.
///
pub fn read_text_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        panic!("Failed to read file at {}: {}", path.display(), e);
    })
}
