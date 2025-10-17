use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead; // Import BufRead trait
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use crate::utilities::is_text_extension;

/// Counts lines of code in files, grouped by language (file extension)
///
/// # Arguments
///
/// * `paths` - Vector of paths to search
/// * `extensions` - Optional vector of file extensions to include
pub fn count_lines(paths: &[PathBuf], extensions: Option<&str>) -> Result<BTreeMap<String, usize>, Box<dyn std::error::Error>> {
    let mut counts = BTreeMap::new();
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
                let counter = counts.entry(match extension.as_ref() {
                    Some(ext) => ext.clone(),
                    None => "no_extension".to_string(),
                }).or_insert(0);
                *counter += lines;
            }
        }
    }

   Ok(counts)
}

/// Count lines in a single file
fn count_file_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    // Use stdio to read line by line, skip empty lines
    // This is more memory efficient than reading entire file
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        if !line.trim().is_empty() {
            count += 1;
        }
    }

    Ok(count)
}
