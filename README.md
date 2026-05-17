# rcontext

A CLI tool to merge a codebase into a single context file.

## Usage

### Context merging

Merges all codebase into a single markdown file with file structure and contents.

```bash
$ rcontext
$ rcontext src/ tests/
$ rcontext --stdout
$ rcontext --stdout --exts rs,toml
```

Now you can safely upload all your company's codebase into ~~stackoverflow~~ chatgpt!

### Line count

Counts non-blank lines of code, grouped by language (file extension).

```bash
$ rcontext --lc
md files: 29 lines
rs files: 206 lines
toml files: 12 lines

$ rcontext --lc --exts rs,toml
rs files: 206 lines
toml files: 12 lines
```

## Options

| Flag       | Description                                               |
|------------|-----------------------------------------------------------|
| `--exts`   | Filter by file extensions (comma-separated, e.g. `rs,py`)|
| `--lc`     | Count lines instead of generating context                 |
| `--stdout` | Print context to stdout                                   |
| `--output` | Output file for context mode (default: `context.md`)     |

## Installation

```bash
$ cargo install --git https://github.com/hocop/repotricks
```

## Features

- Respects .gitignore
- Blazing fast
