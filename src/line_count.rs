use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead; // Import BufRead trait
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rayon::prelude::*;
use crate::utilities::is_text_extension;


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

        walk.build().filter_map(Result::ok)
            .filter(|entry|
                entry.path().is_file() && entry.file_type().map_or(false, |ft| ft.is_file())
            )
            .map(|entry| {
                let extension = entry.path().extension()
                    .and_then(|os_str| os_str.to_str())
                    .map(|s| s.to_lowercase());
                (entry, extension)
            })
            .filter(|(_entry, extension)|
                // Check if extension is allowed
                if let Some(ref ext_set) = ext_set {
                    ext_set.contains(&extension.as_ref().unwrap_or(&"".to_string()))
                } else {
                    true
                }
            )
            .filter(|(_entry, extension)|
                // Skip binary files - only allow known text-based extensions
                if let Some(extension) = extension {
                    is_text_extension(extension)
                } else {
                    true
                }
            )
            .collect::<Vec<_>>().par_iter()
            .filter_map(|(entry, extension)|
                count_file_lines(entry.path()).map(|count| (extension, count)).ok()
            )
            .map(|(extension, count)| (
                match extension.as_ref() {
                    Some(ext) => ext.clone(),
                    None => "no_extension".to_string(),
                },
                count
            ))
            .collect::<Vec<_>>().iter()
            .for_each(|(ext, count)| {
                *counts.entry(ext.clone()).or_insert(0) += count;
            });
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
