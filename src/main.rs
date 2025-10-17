use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI tool to analyze code repositories
#[derive(Parser)]
#[command(name = "repotricks")]
#[command(author = "Your Name <you@example.com>")]
#[command(version = "0.1.0")]
#[command(about = "Analyze and report on code repositories", long_about = None)]
struct Cli {
    /// One or more paths to search (default is current directory)
    #[arg(default_value = ".", global = true)]
    paths: Vec<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Count lines of code grouped by language
    Lc {
        /// Show only these extensions (comma-separated, e.g. rs,py,js)
        #[arg(long, value_name = "EXTENSIONS")]
        extensions: Option<String>,
    },
    /// Merge all codebase into a single markdown file
    Context {
        /// Output file path (default: context.md)
        #[arg(long, default_value = "context.md", value_name = "FILE")]
        output: String,
    },
}

mod line_count;
mod context;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Lc { extensions } => {
            match line_count::count_lines(&cli.paths, extensions.as_deref()) {
                Ok(counts) => {
                    for (ext, count) in counts {
                        println!("{} files: {} lines", ext, count);
                    }
                }
                Err(e) => eprintln!("Error counting lines: {}", e),
            }
        }
        Commands::Context { output } => {
            if let Err(e) = context::generate_context(&cli.paths, output) {
                eprintln!("Error generating context: {}", e);
            } else {
                println!("Context file generated: {}", output);
            }
        }
    }
}
