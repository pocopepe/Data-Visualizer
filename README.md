File Tool
A command-line utility written in Rust for file content analysis and secure data overwriting. This tool is designed for users who want to securely erase files or get a quick visual overview of a file's binary data.

Features
Visualize: Analyzes a file's binary content and visualizes it as a 200x200 grid. Each cell in the grid represents a chunk of the file, with the cell's value corresponding to the most frequent byte in that chunk. This can be useful for quickly identifying patterns or randomness in a file's data.

Overwrite: Securely and permanently overwrites a file or all files within a directory with zeros. This helps ensure that the original data cannot be recovered.

Installation
Ensure you have the Rust programming language and Cargo (Rust's package manager) installed.

Add the clap crate as a dependency in your Cargo.toml file to handle command-line arguments.

[dependencies]
clap = { version = "4.4.1", features = ["derive"] }

Build the project using Cargo:

cargo build --release

Usage
Run the tool from your terminal.

Visualize a file
To get a visual representation of a file's binary content, use the visualize command:

cargo run -- visualize --file <path_to_file>

Replace <path_to_file> with the path to the file you want to analyze.

Securely overwrite a file
To overwrite a single file, use the overwrite command. You will be prompted for confirmation unless you use the -y flag.

# Basic overwrite with confirmation
cargo run -- overwrite --file <path_to_file>

# Overwrite without a confirmation prompt
cargo run -- overwrite --file <path_to_file> -y

Securely overwrite a directory
To recursively overwrite all files within a directory and its subdirectories, use the --recursive flag. This action is irreversible.

# Recursive overwrite with confirmation
cargo run -- overwrite --file <path_to_directory> --recursive

# Recursive overwrite without a confirmation prompt
cargo run -- overwrite --file <path_to_directory> -y --recursive

Contributing
Feel free to submit issues or pull requests to improve the tool.

License
This project is licensed under the MIT License.