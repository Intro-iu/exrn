# exrn

`exrn` is a simple and flexible command-line tool that renames files based on regular expression patterns. It allows users to specify patterns to match files and transform them into new names using customizable replacement rules.

## Features
- Rename files or directories using regular expressions.
- Supports wildcards to match multiple files.
- Optionally confirm renaming operations.
- Skip confirmation prompts using a `--yes` flag for automated renaming.

## Installation
To use `exrn`, ensure you have a Rust environment set up. If not, follow these steps to install Rust:

1. Install Rust by running the following command:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone or download the source code from this repository.

3. Build the project:
   ```bash
   cargo build --release
   ```

4. You can find the compiled executable in the `target/release/` directory.

## Usage
Run `exrn` from the command line to rename files based on regex patterns. Below is the basic command structure:

```bash
exrn --src <SOURCE_PATTERN> --rule <SOURCE_REGEX> <TARGET_REGEX> [--yes]
```

### Arguments:
- `--src`, `-s`: Specify the source file(s) to rename. Supports wildcards (e.g., `*.txt`).
- `--rule`, `-r`: Provide two regular expressions, one to match source file names and another to define the target name pattern.
- `--yes`, `-y` (optional): Skip the confirmation prompt before renaming the files.

### Example:
Suppose you have multiple `.txt` files that follow a naming pattern like `file_123.txt`, and you want to rename them to `document_123.txt`. You can achieve this with:

```bash
exrn -s *.txt -r 'file_(\d+)' 'document_$1'
```

This will match files like `file_123.txt` and rename them to `document_123.txt` after confirmation below:

```txt
Matching files...
        /path/to/file_123.txt -> /path/to/document_123.txt
Do you want to proceed with the renaming of all matched files? [y/N]: 
```

### Skipping Confirmation:
If you are certain of the changes and want to skip the confirmation prompt, use the `--yes` flag:

```bash
exrn -s *.txt -r 'file_(\d+)' 'document_$1' --yes
```

## Confirmation Prompt
By default, `exrn` will ask for user confirmation before renaming multiple files. It will display a prompt like:

```
Do you want to proceed with the renaming of all matched files? [y/N]:
```

Type `y` or `yes` to proceed, or `n` to cancel the operation.

## Error Handling
- If a file cannot be accessed or renamed, an error message will be printed without stopping the renaming process for other files.
- The tool ensures that only files matching the source regex pattern will be renamed.

## License
This project is licensed under the GPLv3 License. See the `LICENSE` file for more details.

## Contributing
Feel free to open issues and submit pull requests to improve this tool.

---

Enjoy renaming your files with ease using `exrn`!
