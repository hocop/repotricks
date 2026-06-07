use clap::Parser;
use std::path::PathBuf;
use std::fs;
use utilities::is_in_gitignore;

/// A CLI tool to analyze code repositories
#[derive(Parser)]
#[command(name = "rcontext")]
#[command(about = "Merge codebase into context or count lines", long_about = None)]
struct Cli {
    /// One or more paths to search (default is current directory)
    #[arg(default_value = ".", short = 'p')]
    paths: Vec<PathBuf>,

    /// One or more paths to exclude (applied after paths; effectively paths_to_include = set(paths) - set(exclude))
    #[arg(long, short = 'x', value_name = "PATHS")]
    exclude: Vec<PathBuf>,

    /// Filter by file extensions (comma-separated, e.g. rs,py,js)
    #[arg(long, value_name = "EXTENSIONS")]
    exts: Option<String>,

    /// Count lines of code grouped by language instead of generating context
    #[arg(long, default_value = "false")]
    lc: bool,

    /// Print context to stdout instead of writing to a file
    #[arg(long, default_value = "false", short = 's')]
    stdout: bool,

    /// Output file path for context mode (default: context.md)
    #[arg(long, default_value = "context.md", value_name = "FILE")]
    output: String,
}

mod line_count;
mod context;
mod utilities;

fn main() {
    let cli = Cli::parse();

    let ext_filter = if let Some(ref exts) = cli.exts {
        let parsed: Vec<String> = exts
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .collect();
        Some(parsed)
    } else {
        None
    };

    if cli.lc {
        match line_count::count_lines(&cli.paths, ext_filter.as_ref(), &cli.exclude) {
            Ok(counts) => {
                for (ext, count) in counts {
                    println!("{} files: {} lines", ext, count);
                }
            }
            Err(e) => eprintln!("Error counting lines: {}", e),
        }
    } else {
        let content = context::generate_context(&cli.paths, ext_filter.as_ref(), Some(&cli.output), &cli.exclude);
        if cli.stdout {
            println!("{}", content);
        } else {
            match fs::write(&cli.output, &content) {
                Ok(()) => {
                    println!("Context saved to {}", cli.output);
                    if !is_in_gitignore(&cli.output) {
                        println!("\nNote: {} is not in your .gitignore.", cli.output);
                        println!("To avoid committing it, run:");
                        println!("echo {} >> .gitignore", cli.output);
                    }
                }
                Err(e) => eprintln!("Error writing context: {}", e),
            }
        }
    }
}
