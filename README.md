# Directory Explorer

Directory Explorer is a simple command-line tool written in Rust for listing and exploring files in a directory. It allows you to list all files or filter them by type, and optionally sort them by size.

## Features

- List all files in a directory
- Filter files by type (extension)
- Sort files by size (top or bottom)
- Display file sizes in megabytes (MB)

## Requirements

- Rust compiler (https://www.rust-lang.org/)
- Cargo package manager (included with Rust)

## Usage

To use the Directory Explorer, follow these steps:

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/directory-explorer.git
    ```

2. Navigate to the project directory:

    ```bash
    cd directory-explorer
    ```

3. Run the program using Cargo:

    ```bash
    cargo run <operation> [-d <directory>] [<file type>] [-b or -t to list top or bottom size]
    ```

   Replace `<operation>` with either `ls` (to list files) or `lsz` (to list files with sizes).
   
   Optional arguments:
   - `-d <directory>`: Specify the directory to explore (default is current directory).
   - `<file type>`: Filter files by type (e.g., txt, jpg, png).
   - `-b` or `-t`: Sort files by size (bottom or top).

## Examples

List all files in the current directory:
```bash
cargo run ls
```


List all .txt files in the current directory:
```bash
cargo run lsz -t -b
```