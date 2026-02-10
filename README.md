# Wimc (Where Is My Command)

## Overview

Wimc was originally created to locate forgotten debug prints scattered throughout a codebase, but it has evolved into a versatile search tool. It scans directories efficiently and helps you find any command, keyword, or pattern across one or multiple projects. It’s useful for code cleanup, auditing, or understanding large codebases.

## Installation
```bash
# Build the optimized release version
cargo build --release

# The executable will be located at:
target/release/wimc # or wimc.exe on Windows

# Linux:
# move the executable to the system’s binary directory with:
sudo cp target/release/wimc /usr/bin

# Windows:
# Move the wimc.exe file to any folder included in your system’s PATH
# (e.g., C:\Windows\System32 or any folder of your choice).
```

## Usage
```bash
# In the root of your project, run:
wimc <path> <command> -<arguments>

# Example:
wimc . println! -deo

# Extra arguments.
-d — scan all directories
-e — scan all files
-o — scan ocult files

# Generated Files
# After execution, Wimc creates two text files in the directory where the command was run:

wimc_results.txt — lists all files that matched the search
wimc_errors.txt — logs files that could not be read or analyzed
```

## Results

### Wimc_results.txt

This file contains a summary of the results in the first line, followed by the paths and the corresponding lines for the requested command.

```
Total_files: 5 | Total_lines: 16

Path: ./README.pt_br.md
Lines: [48, 51, 54]

Path: ./src/args.rs
Lines: [1, 12, 19]

Path: ./src/commands.rs
Lines: [3, 11, 30]

Path: ./src/entity.rs
Lines: [1, 4, 10, 27]

Path: ./src/file.rs
Lines: [4, 12, 63]
```

### Wimc_errors.txt

This file contains a summary of the errors in the first line, followed by the file paths and the error type encountered while trying to read the file.

```
Total_files: 139

Path: ./.git/index
Error: stream did not contain valid UTF-8

Path: ./.git/objects/09/ff8c2a84cf0ad520dcb4322b537e562a2a25b2
Error: stream did not contain valid UTF-8

Path: ./.git/objects/0b/2666c1a7673817b2c6d1d4e6e1f8efc38b1e4a
Error: stream did not contain valid UTF-8

Path: ./.git/objects/0c/c4b75e4e985d5bedf7afc0303eddeb347371ca
Error: stream did not contain valid UTF-8
```

## Tips

Without using the -d argument, directories such as /target are ignored. However, if wimc is executed inside that directory, it will work normally.

## Customization
To add new directories or extensions to the ignore lists, open ignore.rs and include your entries in the ignored_files and ignored_extensions lists.

```rust
// Example:
pub fn ignored_files() -> Vec<&'static str> {
    vec![
        "/.git",
        "/target", // Rust

        // Add new paths here
    ]
}
```