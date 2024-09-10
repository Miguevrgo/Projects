
# Algori - Algorithm Visualizer in Rust with GTK4

## Description

Algori is an application written in Rust that uses GTK4 to provide a graphical user interface (GUI) for visualizing various algorithms. 
This application is designed to help students and professionals understand and analyze the behavior of different algorithms in an interactive and visual manner.

## Features

- Interactive visualization of algorithms
- Intuitive graphical user interface (GUI) using GTK4
- Support for multiple algorithms from different categories (sorting, searching, etc.)

## Tasks
- [] Make help documentation for each view
- [] Provide an interactive experience for each view
- [] Error handling

## Requirements

To run Algori, you need to have the following components installed:

- [Rust](https://www.rust-lang.org/)
- [GTK4](https://gtk-rs.org/)

## Installation

### Installing Rust

If you don't have Rust installed, you can install it using the following command:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installing Rust, ensure that your PATH environment variable is set up correctly by adding the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):
```sh
source $HOME/.cargo/env
```
### Installing GTK4
To install GTK4, follow the instructions for your specific operating system. You can find detailed installation guides on the [GTK-rs website.](https://gtk-rs.org/gtk4-rs/stable/latest/book/):

### Debian/Ubuntu
For Debian-based systems (like Ubuntu), you can install all the dependencies with the following command:
```sh
sudo apt install libgtk-4-dev build-essential
```

### Fedora and derivatives
```sh
sudo dnf install gtk4-devel gcc
```
### Arch and derivatives
```sh
sudo pacman -S gtk4 base-devel
```
## Building and running
Building and Running the Application

Once you have Rust and GTK4 installed, you can build and run the application using Cargo:
```sh
cargo run
```
This command will compile the application and start it, allowing you to interact with the algorithm visualizer.
## Contributing
Contributions are welcome! If you have any ideas for new features, find bugs, or want to improve the documentation, feel free to open an issue or submit a pull request.

## Inspiration

This project was inspired by [Visualgo](https://visualgo.net/), an educational tool for visualizing data structures and algorithms.
However, my goal was to create a desktop application that is faster and provides a more pleasant experience for users.
While the idea originated from Visualgo, I did not copy anything directly. Many aspects will be different as I have implemented what I considered best at each moment.
My intention was neither to make it the same nor completely different, but rather to create the most effective solution possible, even if that sometimes aligns with Visualgo's approach.






