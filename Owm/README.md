# OWM (Oxide Window Manager)

OWM (Oxide Window Manager) is a lightweight, dynamic window manager for the X11 display server, written entirely in Rust. Designed as both a learning project and a highly efficient tool, OWM balances simplicity, speed, and aesthetic appeal. Inspired by minimalistic window managers like dwm, OWM is built with a focus on being approachable and customizable.

## Features

- **Dynamic Tiling**: Automatically organizes windows in a space-efficient layout.
- **Lightweight**: Minimal dependencies for speed and low resource usage.
- **Customizable**: Simple configuration for keybindings and layouts.
- **Educational**: Aimed at providing insight into how window managers work.
- **Rust-Powered**: Leverages Rust's safety and performance.

## Why OWM?

OWM is for those who:
- Want to explore how a window manager is built.
- Appreciate minimalism and efficiency.
- Seek a foundation to customize and extend.

## Roadmap

### **v0.1: Initial Prototype**
- [ ] Establish connection to the X11 server.
- [ ] Capture and handle basic events (e.g., window creation, keypress).
- [ ] Implement basic window management (e.g., tiling, focus switching).
- [ ] Add a configuration file for keybindings.

### **v0.2: Feature Expansion**
- [ ] Support for floating and fullscreen windows.
- [ ] Multi-monitor support.
- [ ] Basic compatibility with ICCCM and EWMH standards.
- [ ] Improved error handling and logging.

### **v0.3: Usability Improvements**
- [ ] Polished layouts and smooth animations.
- [ ] Comprehensive documentation.
- [ ] Modular codebase for easier contributions.
- [ ] Theming support for aesthetic customization.

### **v1.0: Stable Release**
- [ ] Complete ICCCM and EWMH compatibility.
- [ ] Extensive user testing and bug fixes.
- [ ] Highly optimized performance.
- [ ] Community-driven enhancements.

## Installation

> OWM is currently in development and not yet ready for production use.

To try out the latest version:

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/owm.git
   cd owm
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```
3. Launch OWM (ensure no other window manager is running):
   ```sh
   ./target/release/owm
   ```

## Contributing

Contributions are welcome! Whether it's a bug report, feature suggestion, or pull request, your input helps make OWM better. Please follow the contribution guidelines outlined in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

OWM is licensed under the MIT License. See [LICENSE](LICENSE) for more information.

