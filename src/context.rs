use std::fs;
use std::path::PathBuf;
use ignore::WalkBuilder;
use crate::utilities::is_text_extension;

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
            let entry_path = entry.path();

            // Get relative path for display
            let rel_path = entry_path.strip_prefix(path).unwrap_or(entry_path);
            // Add indentation based on depth
            let depth = rel_path.components().count();
            let indent = "  ".repeat(depth);
            content.push_str(&format!("{}- {}\n", indent, rel_path.display()));
        }
    }

    content.push_str("\n\n# File Contents\n\n");

    // Add file contents
    for path in paths {
        let walk = WalkBuilder::new(path);
        for entry in walk.build().filter_map(Result::ok) {
            let entry_path = entry.path();

            // Skip binary files - only allow known text-based extensions
            let extension = entry_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            if !is_text_extension(extension) {
                continue;
            }

            // Only process files, not directories
            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let rel_path = entry_path.strip_prefix(path).unwrap_or(entry_path);
                content.push_str(&format!("## {}\n\n", rel_path.display()));

                if let Ok(file_content) = fs::read_to_string(entry_path) {
                    // Handle markdown files (add code fences for non-code content)
                    if extension == "md" {
                        content.push_str("```\n");
                        content.push_str(&file_content);
                        content.push_str("```\n\n");
                    } else {
                        content.push_str("```\n");
                        content.push_str(&file_content);
                        content.push_str("```\n\n");
                    }
                }
            }
        }
    }

    // Write output file
    fs::write(output, &content)?;

    Ok(())
}
