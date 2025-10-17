use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use crate::utilities::is_text_extension;

/// Counts lines of code in files, grouped by language (file extension)
///
/// # Arguments
///
/// * `paths` - Vector of paths to search
/// * `extensions` - Optional vector of file extensions to include
pub fn count_lines(paths: &[PathBuf], extensions: Option<&str>) -> Result<HashMap<String, usize>, Box<dyn std::error::Error>> {
    let mut counts = HashMap::new();
    let ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    for path in paths {
        let walk = WalkBuilder::new(path);

        // .gitignore handling is built into the WalkBuilder
        for entry in walk.build().filter_map(Result::ok) {
            let path = entry.path();

            // Skip directories and binary files
            if path.is_dir() || !entry.file_type().map_or(false, |ft| ft.is_file()) {
                continue;
            }

            let extension = path.extension()
                .and_then(|os_str| os_str.to_str())
                .map(|s| s.to_lowercase());

            // Check if extension is allowed
            if let Some(ref ext_set) = ext_set {
                if !ext_set.contains(&extension.as_ref().unwrap_or(&"".to_string())) {
                    continue;
                }
            }

            // Skip binary files - only allow known text-based extensions
            if let Some(extension) = extension.as_ref() {
                if !is_text_extension(extension) {
                    continue;
                }
            }

            // Count lines in the file
            if let Ok(lines) = count_file_lines(path) {
                let counter = counts.entry(extension.unwrap_or_default()).or_insert(0);
                *counter += lines;
            }
        }
    }

   Ok(counts)
}

/// Count lines in a single file
fn count_file_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    // Skip empty lines (lines with only whitespace)
    let non_empty_lines = content.lines().filter(|line| !line.trim().is_empty()).count();
    Ok(non_empty_lines)
}
