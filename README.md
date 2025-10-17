# Repotricks

A CLI tool to analyze and report on code repositories.

## Commands

### lc - Line Count

Counts lines of code in files, grouped by language (file extension).

```
$ repotricks lc
md files: 29 lines
rs files: 206 lines
toml files: 12 lines
```

### context - Merge Codebase

Merges all codebase into a single markdown file with file structure and contents.

```bash
$ repotricks context
Context file generated: context.md
```

Now you can safely upload all your company's codebase into ~~stackoverflow~~ chatgpt!

## Features

- Respects .gitignore
- `lc` groups files by language based on file extension

## Installation

To install the tool, use cargo:

```bash
$ cargo install --path .
```

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.
