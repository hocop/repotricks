use std::collections::BTreeMap;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rayon::prelude::*;
use crate::utilities::is_text_extension;

pub fn count_lines(paths: &[PathBuf], extensions: Option<&Vec<String>>) -> Result<BTreeMap<String, usize>, Box<dyn std::error::Error>> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    paths.iter()
        .flat_map(|path| WalkBuilder::new(path).build())
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().map_or(false, |ft| ft.is_file()))
        .filter_map(|entry| {
            entry.path().extension()
                .and_then(|os_str| os_str.to_str())
                .map(|extension| extension.to_lowercase())
                .map(|extension| (entry, extension))
        })
        .filter(|(_entry, extension)| {
            if let Some(set) = extensions {
                set.contains(extension)
            } else {
                true
            }
        })
        .filter(|(_entry, extension)| is_text_extension(extension))
        .collect::<Vec<_>>().par_iter()
        .filter_map(|(entry, extension)|
            count_file_lines(entry.path()).map(|count| (extension, count)).ok()
        )
        .collect::<Vec<_>>().iter()
        .for_each(|(ext, count)| {
            *counts.entry(ext.to_string()).or_insert(0) += count;
        });

    Ok(counts)
}

fn count_file_lines(path: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let file = fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let count = reader.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .count();

    Ok(count)
}