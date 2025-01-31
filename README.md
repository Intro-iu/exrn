# exrn 🔄

**A Safe & Powerful Batch File Renamer with Regex Magic** ✨

![Rust](https://img.shields.io/badge/rust-%23e57373.svg?style=flat&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-green)
 
*Transform your files like a wizard! 🧙*

## Features 🌟

- 🎯 **Precision Matching** with regex patterns
- 🔄 **Smart Replacement** using capture groups
- 👮 **Safety First** - Interactive confirmation & dry-run preview
- 🚀 **Blazing Fast** - Built with Rust performance
- 📦 **Cross-Platform** - Works on Windows/macOS/Linux
- 📝 **Visual Feedback** - Colorized diff previews

## Installation ⚡

### Via Cargo (Recommended)
```bash
cargo install exrn --git https://github.com/Intro-iu/exrn.git
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
exrn -s "GLOB_PATTERN" -r "SOURCE_REGEX" "TARGET_PATTERN" [OPTIONS]
```

### Key Options
| Option          | Description                          |
|-----------------|--------------------------------------|
| `-s, --sources` | File glob patterns (e.g., `*.txt`)   |
| `-r, --rule`    | Regex match and replacement pattern  |
| `-y, --yes`     | Auto-confirm all actions             |
| `--dry-run`     | Preview changes without executing    |

## Examples 🧪

### 1. Change File Extensions
```bash
exrn -s "*.txt" -r '^(.*)\.txt$' '$1.md'
```
🔍 *Matches: `notes.txt` → `notes.md`*

### 2. Date Format Conversion
```bash
exrn -s "*.log" -r '(\d{4})-(\d{2})-(\d{2})' '$2$3$1'
```
📆 *Converts `2023-08-15.log` → `08152023.log*

### 3. Batch Numbering
```bash
exrn -s "photo_*.jpg" -r 'photo_(\d+)' 'vacation_$1'
```
📸 *Renames `photo_001.jpg` → `vacation_001.jpg`*

## Advanced Tips 🚀

### Capture Group Magic
```bash
# Swap name components
exrn -s "*.csv" -r '^(experiment)_(\d{4})_(\d+)' '$2_$3_$1'
```
🔀 *Transforms `experiment_2023_45.csv` → `2023_45_experiment.csv*

### Dry Run Mode
```bash
exrn -s "*.tmp" -r '(.+)\.tmp$' '$1.bak' --dry-run
```
👁️ *Preview changes before executing*

## Safety Measures 🔒

exrn includes multiple protection layers:
```rust
if source == target { /* Skip identical paths */ }
if target.exists() { /* Prevent overwrites */ }
if new_name.is_empty() { /* Block empty names */ }
```
## License 📄

This project is licensed under the MIT License - see the [LICENSE](https://github.com/Intro-iu/exrn/blob/main/LICENSE) file for details.

---

💡 **Pro Tip**: Always test your patterns with `--dry-run` first!  
🐞 Found an issue? [Report it here](https://github.com/Intro-iu/exrn/issues)  
⭐ Love exrn? Give us a star on [GitHub](https://github.com/Intro-iu/exrn)!
