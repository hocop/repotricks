use std::fs;
use std::path::PathBuf;
use ignore::WalkBuilder;
use crate::utilities::is_text_extension;

fn is_excluded(entry_path: &std::path::Path, excludes: &[PathBuf]) -> bool {
    if excludes.is_empty() {
        return false;
    }
    if let Ok(canonical) = fs::canonicalize(entry_path) {
        for excl in excludes {
            if let Ok(excl_canonical) = fs::canonicalize(excl) {
                if canonical == excl_canonical || canonical.starts_with(&excl_canonical) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn generate_context(paths: &[PathBuf], extensions: Option<&Vec<String>>, _exclude_file: Option<&str>, excludes: &[PathBuf]) -> String {
    let mut content = String::new();

    content.push_str("# File Structure\n\n");
    for path in paths {
        let walk = WalkBuilder::new(path);
        for entry in walk.build().filter_map(Result::ok) {
            let entry_path = entry.path();

            if is_excluded(entry_path, excludes) {
                continue;
            }

            let depth = entry_path.components().count();
            let indent = "  ".repeat(depth - 1);
            content.push_str(&format!("{}- {}\n", indent, entry_path.display()));
        }
    }

    content.push_str("\n\n# File Contents\n\n");

    for path in paths {
        let walk = WalkBuilder::new(path);
        for entry in walk.build().filter_map(Result::ok) {
            let entry_path = entry.path();

            if is_excluded(entry_path, excludes) {
                continue;
            }

            let extension = entry_path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
            let extension_lower = extension.to_lowercase();

            if !is_text_extension(&extension_lower) {
                continue;
            }

            if let Some(set) = extensions {
                if !set.contains(&extension_lower) {
                    continue;
                }
            }

            if entry.file_type().map_or(false, |ft| ft.is_file()) {
                let rel_path = entry_path.strip_prefix(path).unwrap_or(entry_path);
                content.push_str(&format!("{}\n", rel_path.display()));

                if let Ok(file_content) = fs::read_to_string(entry_path) {
                    content.push_str("```\n");
                    content.push_str(&file_content);
                    content.push_str("\n```\n\n");
                }
            }
        }
    }

    content
}
