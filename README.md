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

## Examples 🧪

### 1. Change File Extensions (With Confirmation)
```bash
exrn -s "*.txt" -r '^(.*)\.txt$' '$1.md'
```

**Interactive Output**:
```text
Matching files...
[1] notes.txt => notes.md
[2] draft.txt => draft.md

Planned changes (2 files):
Do you want to proceed with the renaming? [y/N]: y

Renaming completed! 2/2 files successfully renamed
```

### 2. Auto-Confirm with -y Flag
```bash
exrn -s "temp_*" -r 'temp_' 'final_' -y
```

**Output**:
```text
Auto-confirm enabled
[1] temp_file1 => final_file1
[2] temp_file2 => final_file2
Renamed 2 files successfully
```

### 3. Date Format Conversion
```bash
exrn -s "*.log" -r '(\d{4})-(\d{2})-(\d{2})' '$2$3$1'
```
📆 *Converts `2023-08-15.log` → `08152023.log`*

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

💡 **Pro Tip**: Always check the preview before confirming!  
🐞 Found an issue? [Report it here](https://github.com/Intro-iu/exrn/issues)  
⭐ Love exrn? Give us a star on [GitHub](https://github.com/Intro-iu/exrn)!