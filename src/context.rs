use std::fs::File;
use std::io::{BufWriter, Write, BufRead, BufReader};
use std::path::PathBuf;
use ignore::WalkBuilder;
use crate::utilities::is_text_extension;

/// Efficiently merges the codebase into one markdown file.
pub fn generate_context(paths: &[PathBuf], output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);

    // --- File tree header ---
    writeln!(writer, "# File Structure\n")?;

    // Single walk to collect all entries
    let mut text_files = Vec::new();

    for path in paths {
        for entry in WalkBuilder::new(path).build().filter_map(Result::ok) {
            let entry_path = entry.path();

            let rel_path = entry_path.strip_prefix(path).unwrap_or(entry_path);
            let depth = rel_path.components().count();
            if depth >= 1 {
                let indent = "  ".repeat(depth - 1);
                writeln!(writer, "{}- {}", indent, rel_path.file_name().unwrap().display())?;
            }

            // Collect text files for later reading
            if let Some(ext) = entry_path.extension().and_then(|x| x.to_str()) {
                if is_text_extension(ext) {
                    text_files.push(entry_path.to_path_buf());
                }
            }
        }
    }

    writeln!(writer, "\n\n# File Contents\n")?;

    // --- Stream file contents ---
    for entry_path in text_files {
        writeln!(writer, "{}\n```", entry_path.display())?;

        if let Ok(file) = File::open(&entry_path) {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            while reader.read_line(&mut line)? > 0 {
                writer.write_all(line.as_bytes())?;
                line.clear();
            }
        }

        writeln!(writer, "```\n")?;
    }

    writer.flush()?;
    Ok(())
}
