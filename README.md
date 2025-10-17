# Repotricks

A CLI tool to analyze and report on code repositories.

## Installation

To install the tool, use cargo:

```bash
cargo install --path .
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

- Respects .gitignore
- Groups files by language based on file extension

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
