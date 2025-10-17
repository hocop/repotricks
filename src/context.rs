use std::fs;
use std::path::PathBuf;
use ignore::WalkBuilder;

/// Merges the entire codebase into a single markdown file
///
/// # Arguments
///
/// * `paths` - Vector of paths to search
/// * `output` - Path to the output file
pub fn generate_context(paths: &[PathBuf], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut content = String::new();

    // Add file tree structure
    content.push_str("# File Structure\n\n");
    for path in paths {
        let walk = WalkBuilder::new(path);
        for entry in walk.build().filter_map(Result::ok) {
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                continue;
            }
            let rel_path = entry.path().strip_prefix(path).unwrap_or(entry.path());
            content.push_str(&format!("- {}\n", rel_path.display()));
        }
    }

    content.push_str("\n\n# File Contents\n\n");

    // Add file contents
    for path in paths {
        let walk = WalkBuilder::new(path);
        for entry in walk.build().filter_map(Result::ok) {
            let path = entry.path();

            // Skip binary files
            if path.extension().and_then(|ext| ext.to_str()) == Some("png") ||
               path.extension().and_then(|ext| ext.to_str()) == Some("jpg") ||
               path.extension().and_then(|ext| ext.to_str()) == Some("gif") ||
               path.extension().and_then(|ext| ext.to_str()) == Some("pdf") ||
               path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
                continue;
            }

            // Only process files, not directories
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let rel_path = path.strip_prefix(path).unwrap_or(path);
                content.push_str(&format!("## {}\n\n", rel_path.display()));

                if let Ok(file_content) = fs::read_to_string(path) {
                    content.push_str("```\n");
                    content.push_str(&file_content);
                    content.push_str("```\n\n");
                }
            }
        }
    }

    // Write output file
    fs::write(output, &content)?;

    Ok(())
}
