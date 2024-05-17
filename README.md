# lsr

lsr is a simple command-line tool written in Rust for listing and exploring files in a directory. It allows you to list all files or filter them by type, and optionally sort them by size.

## Features

- List all files in a directory
- Filter files by type (extension)
- Sort files by size (top or bottom)
- Display file sizes in megabytes (MB)

## Requirements

- Rust compiler (https://www.rust-lang.org/)
- Cargo package manager (included with Rust)

## Usage

To use the lsr, follow these steps:

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
    cargo run <operation> <options>
    ```
    
     ```bash
Options:
  -d, --directory <DIRECTORY>          [default: .]
  -f, --file-type <FILE_TYPE>          
  -o, --order-by-size <ORDER_BY_SIZE>  [default: ]
  -p, --purge                          
  -r, --recursive                      
  -h, --help                           Print help
 ```
 
    ```

   Replace `<operation>` with either `ls` (to list files) or `lsz` (to list files with sizes).
   
   Optional arguments:
   - `-d <directory>`: Specify the directory to explore (default is current directory).
   - `<file type>`: Filter files by type (e.g., txt, jpg, png).
   - `-b` or `-t`: Sort files by size (bottom or top).

## Examples


 ### Example
    ```bash
    cargo run lsr -d /home/username -f txt -o -t
    ```
    This command lists all text files in the `/home/username` directory, sorted by size in descending order.

    ```bash
    cargo run lsz -d /home/username -f jpg -o -b
    ```
    This command lists all JPEG files in the `/home/username` directory, sorted by size in ascending order.
    
    ```bash


## Examples **Adding Binary to PATH in Linux:**

To make lsr easily accessible from anywhere in your terminal, you can add the binary to your PATH environment variable. Follow these steps:

1. Navigate to the directory containing the `lsr` binary.

2. Copy the binary to a directory that is already in your PATH, such as `/usr/local/bin`. You may need superuser privileges to copy files to this directory. Use the following command:

   ```bash
   sudo cp lsr /usr/local/bin

3. Once the binary is copied, you can run lsr from any directory in your terminal by simply typing lsr  followed by the desired arguments.