use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead; // Import BufRead trait
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
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

    // Process each path in parallel using Rayon
    for path in paths {
        let walk = WalkBuilder::new(path);

        // Create a stream of file entries with proper filtering:
        // 1. Keep only files (not directories)
        // 2. Extract file extension for each file
        walk.build().filter_map(Result::ok)
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
            .filter(|(_entry, extension)|
                // Skip binary files by checking against known text extensions
                is_text_extension(extension)
            )
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
    }

   // Return a clone of the inner BTreeMap
   Ok(counts)
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
