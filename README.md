# rcontext

A CLI tool to merge a codebase into a single context file.

## Usage

### Context merging

Merges all codebase into a single markdown file with file structure and contents.

```bash
$ rcontext
Context saved to my_context.md
```

> [!TIP]
> Now you can safely upload all your company's codebase into ~~stackoverflow~~ chatgpt!

List specific files and/or directories:

```bash
rcontext src/ tests/ README.md
```

Filter by extensions:

```bash
rcontext --exts rs,toml
```

Print to stdout instead of `my_context.md`.

```bash
rcontext --stdout
```

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
$ cargo install --git https://github.com/hocop/rcontext
```

## Features

- Respects .gitignore
- Blazing fast
