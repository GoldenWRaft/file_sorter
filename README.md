# Rust File Sorter

A simple and efficient command-line utility written in Rust to organize files within a specified directory by sorting them into folders based on their file extensions.

## 📋 Table of Contents

- [Rust File Sorter](#rust-file-sorter)
  - [📋 Table of Contents](#-table-of-contents)
  - [✨ Features](#-features)
  - [🔧 Prerequisites](#-prerequisites)
  - [🚀 Installation](#-installation)
  - [💻 Usage](#-usage)
  - [⚙️ How It Works](#️-how-it-works)

## ✨ Features

*   **Automatic Folder Creation**: Creates dedicated folders for each file type (e.g., `_pdf_folder`, `_jpg_folder`).
*   **Interactive Preview**: Displays a preview of all planned file movements before making any changes.
*   **User Confirmation**: Prompts the user for confirmation (y/n) before proceeding with the file operations, ensuring no accidental changes are made.
*   **Conflict Resolution**: Automatically renames files by appending a counter (e.g., `document (1).pdf`) if a file with the same name already exists in the destination folder.
*   **Error Handling**: Provides clear error messages if the specified path is not a valid directory.

## 🔧 Prerequisites

*   [Rust programming language](https://www.rust-lang.org/tools/install) (and its package manager, Cargo).

## 🚀 Installation

1.  Clone this repository or save the code as `main.rs` in a new project directory.
2.  Navigate to your project directory in the terminal.
3.  Ensure your `Cargo.toml` file matches the following:

    ```toml
    [package]
    name = "file_sorter"
    version = "0.1.0"
    edition = "2024"

    [dependencies]
    clap = { version = "4.4.18", features = ["derive"] }
    ```

4.  Build the project to ensure all dependencies are fetched and the code compiles:

    ```bash
    cargo build --release
    ```
    The executable will be located at `./target/release/file_sorter`.

## 💻 Usage

Run the tool from the command line, providing the path to the directory you want to sort using the `--path` or `-p` argument.

**Syntax (during development):**

```bash
cargo run -- --path <PATH_TO_YOUR_DIRECTORY>
```

**Syntax (using the built executable):**

```bash
./target/release/file_sorter --path <PATH_TO_YOUR_DIRECTORY>
```

**Example:**

To sort files on your Desktop, you might run:

```bash
./target/release/file_sorter -p "/Users/YourUser/Desktop"
```

The application will then show you a preview of the changes and ask for your confirmation before proceeding.

## ⚙️ How It Works

1.  **Parse Arguments**: The program uses the `clap` crate to parse the command-line arguments and retrieve the target directory path.
2.  **Read Directory**: It reads all the entries in the specified directory and identifies the files.
3.  **Plan Moves**: For each file, it determines the file extension and plans a move to a corresponding sub-folder (e.g., a `.txt` file goes into the `_txt_folder`). It also checks for potential file name collisions and plans new names accordingly.
4.  **Preview and Confirm**: The program prints a list of all planned source and destination paths and waits for the user to type `y` or `yes` to proceed.
5.  **Execute Sorting**: If confirmed, it creates the necessary destination folders and then moves each file to its new location. If the user cancels, the operation is safely aborted.