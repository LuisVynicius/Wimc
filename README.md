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
wimc . println! -de

# Extra arguments.
-d — scan all directories
-e — scan all files

# Generated Files
# After execution, Wimc creates two text files in the directory where the command was run:

wimc_results.txt — lists all files that matched the search
wimc_errors.txt — logs files that could not be read or analyzed
```

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