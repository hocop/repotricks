# Repotricks

A CLI tool to analyze and report on code repositories.

## Installation

To install the tool, use cargo:

```bash
cargo install --path /home/ruslan/code/small_progs/repo_tricks
```

## Commands

### lc - Line Count

Counts lines of code in files, grouped by language (file extension).

```bash
# Count lines in the current directory
repotricks lc

# Count lines with specific extensions
repotricks lc --extensions rs,py,js
```

### context - Merge Codebase

Merges all codebase into a single markdown file with file structure and contents.

```bash
# Generate context file in current directory
repotricks context

# Generate context file with custom output path
repotricks context --output my-context.md
```

## Features

- **Respects .gitignore**: Both commands ignore files specified in .gitignore
- **Language Detection**: Automatically groups files by language based on file extension
- **Image Handling**: Lists images in file tree but doesn't include them in line counts
- **Markdown Output**: Context command generates a comprehensive markdown file with file structure and contents

## Dependencies

The tool uses the following dependencies:

- [clap](https://github.com/clap-rs/clap) for command-line parsing
- [ignore](https://github.com/BurntSushi/ignore) for handling .gitignore patterns
- [walkdir](https://github.com/BurntSushi/walkdir) for directory traversal

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
