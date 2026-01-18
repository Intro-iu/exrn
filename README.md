# exrn 🔄

**A Safe & Powerful Batch File Renamer with Regex Magic** ✨

![Rust](https://img.shields.io/badge/rust-%23e57373.svg?style=flat&logo=rust&logoColor=white)
![Build](https://github.com/Intro-iu/exrn/actions/workflows/ci.yml/badge.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

*Transform your files like a wizard! 🧙*

## Features 🌟

- 🎯 **Precision Matching**: Supports both standard GLOB patterns and direct Shell-expanded paths.
- 🔄 **Smart Replacement**: Use standard Regex Capture Groups (e.g., `$1`, `$2`) for complex renaming.
- ⛓️ **Chain Renaming**: Safely handles dependency chains (e.g., `A -> B`, `B -> C`) using topological sorting.
- 🎨 **Visual Feedback**: **Colorized** diff previews (Red -> Green) to verify changes instantly.
- 🛡️ **Safety First**:
    - **Dry Run** mode to preview without touching files.
    - Collision detection.
    - Interactive confirmation.
- 🚀 **Blazing Fast**: Built with Rust.

## Installation ⚡

### Via Cargo (Recommended)
```bash
cargo install --git https://github.com/Intro-iu/exrn.git
```

### From Source
```bash
git clone https://github.com/Intro-iu/exrn.git
cd exrn
cargo install --path .
```

## Usage 🛠️

### Basic Syntax
```bash
exrn -s [SOURCES] -r 'REGEX' 'REPLACEMENT' [OPTIONS]
```
> **Tip**: It is recommended to use **single quotes** `'...'` for regex patterns to prevent Shell from incorrectly expanding `$1` or `*`.

### Options
| Option      | Short | Description                                                                       |
| ----------- | ----- | --------------------------------------------------------------------------------- |
| `--sources` | `-s`  | File glob patterns or file paths (e.g. `'*.txt'` or `file.txt`)                   |
| `--rule`    | `-r`  | **Required**. Regex pattern and replacement string (e.g. `'(.*)\.txt'` `'$1.md'`) |
| `--dry-run` | `-d`  | Print the planned changes and exit without modifying anything                     |
| `--sort`    |       | Sort the output list by source filename (Default: true)                           |
| `--yes`     | `-y`  | Auto-confirm all prompts (Non-interactive mode)                                   |

## Examples 🧪

### 1. Change Extensions (Dry Run first!)
Preview what would happen:
```bash
exrn -s '*.txt' -r '(.*)\.txt' '$1.md' --dry-run
```

### 2. Rename with Smart Chain Sorting
If you have files `part1.txt`, `part2.txt`, `part3.txt` and want to shift them naming-wise:
```bash
# Safely handles renaming part2 -> part3 even if part3 exists (but is also being moved)
exrn -s 'part*' -r 'part(\d+)' 'part${1}_next'
```

### 3. Date Formatting
Convert `2023-01-01_log.txt` to `log_2023-01-01.txt`:
```bash
exrn -s '*.txt' -r '(\d{4}-\d{2}-\d{2})_(.*)\.txt' '$2_$1.txt'
```

### 4. Direct Files (Shell Expansion)
You can let your shell handle the globbing (useful for zsh/bash users):
```bash
exrn -s *.png -r 'Img_(.*)' 'Picture_$1'
```

## Safety Measures 🔒

`exrn` includes robust protection layers:
- **Cycle Detection**: Prevents circular renaming (e.g. `A->B` and `B->A`) which would require temporary files.
- **Topological Sorting**: Automatically orders operations so you don't overwrite files that are themselves being moved.
- **Path Validation**: Prevents accidental file movement to other directories if not intended.

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

💡 **Pro Tip**: Always use `--dry-run` (`-d`) first when trying complex regexes!
🐞 Found an issue? [Report it here](https://github.com/Intro-iu/exrn/issues)
⭐ Love exrn? Give us a star on [GitHub](https://github.com/Intro-iu/exrn)!
