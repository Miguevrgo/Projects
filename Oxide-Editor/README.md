# Oxide Editor

Oxide Editor is a lightweight and efficient text editor, inspired by the simplicity and functionality of modern editors like LazyVim. It is written in [Rust](https://www.rust-lang.org/), ensuring fast performance and robust safety. Oxide Editor uses a **GapBuffer** data structure, optimized for editing operations such as inserting and deleting text with minimal performance impact.

## Features

- **Multi-Line Support**: Full support for editing text with multiple lines.
- **Stylized Interface**: Bottom status bar inspired by LazyVim, displaying useful information such as the open file, editing mode, and more.
- **File Management**: Efficiently open, close, and save files.
- **High Performance**: Built with Rust and using GapBuffer for fast and efficient text editing.
- **User-Friendly**: Simple and clear controls, perfect for both beginners and advanced users.

## Installation

Oxide Editor requires Rust and Cargo for compilation and installation. You can install it with the following commands:

```bash
# Clone the repository
git clone https://github.com/yourusername/OxideEditor.git
cd OxideEditor

# Build the project
cargo build --release

# Run the binary
./target/release/oxide_editor
```

## Usage
Once you've succesfully installed Oxide Editor you can use it to open and edit text files:
```
oxide_editor file.txt 
```
Or create a new untitled file
```
oxide_editor
```
```
```
```
```
