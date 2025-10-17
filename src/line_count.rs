use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;

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

            // Skip images and binary files
            if let Some(extension) = extension.as_ref() {
                if is_binary_extension(extension) {
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
    let lines = content.lines().count();
    Ok(lines)
}

/// Check if an extension is for a binary file
fn is_binary_extension(extension: &str) -> bool {
    match extension {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tiff" |
        "svg" | "psd" | "ico" |
        "mp3" | "mp4" | "wav" | "ogg" | "flac" |
        "avi" | "mov" | "mkv" | "mpg" | "mpeg" |
        "pdf" | "doc" | "docx" | "xls" | "xlsx" |
        "ppt" | "pptx" | "zip" | "rar" | "7z" |
        "exe" | "dll" | "so" | "dylib" | "jar" |
        "class" | "swf" | "apk" | "deb" | "rpm" |
        "iso" | "img" | "bin" => true,
        _ => false,
    }
}
