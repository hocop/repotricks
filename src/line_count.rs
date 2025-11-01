use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rayon::prelude::*;
use crate::utilities::is_text_extension;


pub fn count_lines(paths: &[PathBuf], extensions: Option<&str>) -> Result<BTreeMap<String, usize>, Box<dyn std::error::Error>> {
    // Initialize a BTreeMap to store line counts by file extension
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    // Parse the extensions parameter into a set for efficient lookup
    let ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    paths.iter()
        // For each path create a walk
        .flat_map(|path| WalkBuilder::new(path).build())
        .filter_map(Result::ok)
        // Skip directories
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            // Get file extension and convert to lowercase
            entry.path().extension()
                .and_then(|os_str| os_str.to_str())
                .map(|extension| extension.to_lowercase())
                .map(|extension| (entry, extension))
        })
        .filter(|(_entry, extension)|
            // Check if this extension is in the allowed set (if any)
            if let Some(ref ext_set) = ext_set {
                ext_set.contains(extension)
            } else {
                // If no extensions specified, include all
                true
            }
        )
        // Skip binary files by checking against known text extensions
        .filter(|(_entry, extension)| is_text_extension(extension))
        // Collect all matching files into a vector for parallel processing
        .collect::<Vec<_>>().par_iter()
        // Try to count lines for each file (skip files that can't be read)
        .filter_map(|(entry, extension)|
            count_file_lines(entry.path()).map(|count| (extension, count)).ok()
        )
        // Collect results and update the counts map
        .collect::<Vec<_>>().iter()
        .for_each(|(ext, count)| {
            *counts.entry(ext.to_string()).or_insert(0) += count;
        });

   // Return a clone of the inner BTreeMap
   Ok(counts)
}

pub fn count_file_sizes(paths: &[PathBuf], extensions: Option<&str>) -> Result<BTreeMap<String, u64>, Box<dyn std::error::Error>> {
    // Initialize a BTreeMap to store size counts by file extension
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();

    // Parse the extensions parameter into a set for efficient lookup
    let ext_set = if let Some(exts) = extensions {
        let exts = exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<_>>();
        Some(exts)
    } else {
        None
    };

    paths.iter()
        // For each path create a walk
        .flat_map(|path| WalkBuilder::new(path).build())
        .filter_map(Result::ok)
        // Skip directories
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            // Get file extension and convert to lowercase
            entry.path().extension()
            .and_then(|os_str| os_str.to_str())
            .or(Some("_"))
            .map(|extension| extension.to_lowercase())
            .map(|extension| (entry, extension))
        })
        .filter(|(_entry, extension)|
            // Check if this extension is in the allowed set (if any)
            if let Some(ref ext_set) = ext_set {
                ext_set.contains(extension)
            } else {
                // Include all files regardless of extension (unlike count_lines)
                true
            }
        )
        // Collect all matching files into a vector for parallel processing
        .collect::<Vec<_>>().par_iter()
        // Try to count file size for each file (skip files that can't be read)
        .filter_map(|(entry, extension)|
            count_file_size(entry.path()).map(|size| (extension, size)).ok()
        )
        // Collect results and update the counts map
        .collect::<Vec<_>>().iter()
        .for_each(|(ext, count)| {
            *counts.entry(ext.to_string()).or_insert(0) += count;
        });

   // Return a clone of the inner BTreeMap
   Ok(counts)
}

/// Count size of a single file
fn count_file_size(path: &Path) -> Result<u64, Box<dyn std::error::Error>> {
    // Use metadata to get file size
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

/// Count lines in a single file
fn count_file_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    // Use stdio to read line by line, skip empty lines
    // This is more memory efficient than reading entire file
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let count = reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .count();

    Ok(count)
}
