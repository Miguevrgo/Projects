# ONE (Oxide Network Engine)

**ONE (Oxide Network Engine)** is a minimalistic and efficient HTTP server written in **Rust**, designed to 
explore the transport and application layers of the TCP/IP protocol. Inspired by educational projects like 
Chapter 20 of the Rust book, ONE aims to be simple, fast, and highly comprehensible, allowing developers to learn and extend it easily.

## Features

- **Basic HTTP Server**: Responds to HTTP requests following the fundamental principles of the protocol.
- **Clean and Modular Code**: Designed to be easy to read and understand.
- **Educational Focus**: Ideal for learning about networks and the HTTP protocol.
- **Safe and Fast**: Rust guarantees memory safety and optimal performance.
- **Extensible**: A solid foundation to add features like dynamic routes, concurrency handling, and HTTPS.

## Why ONE?

ONE is perfect for:
- Developers who want to understand the fundamentals of HTTP servers.
- Students interested in learning about TCP/IP protocols in practice.
- Rust enthusiasts looking for a compact and educational project.

## Roadmap

### **v0.1: Initial Prototype**
- [ ] Implement basic TCP connection.
- [ ] Handle simple HTTP requests and responses.
- [ ] Initial documentation of the workflow.

### **v0.2: Feature Expansion**
- [ ] Handle multiple connections (tokio).
- [ ] Support basic HTTP headers.
- [ ] Simple logging for debugging.

### **v0.3: Usability Improvements**
- [ ] Support for dynamic content and custom routes.
- [ ] Improved documentation and examples.
- [ ] Basic configuration through files.

### **v1.0: Stable Release**
- [ ] Full support for HTTP 1.1.
- [ ] Efficient resource usage with concurrency.
- [ ] Experimental support for HTTPS (with TLS).

## Installation

> ONE is under active development and not recommended for production use.

To try the latest version:

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/one.git
   cd one
   ```
2. Build the project:
   ```sh
   cargo build --release
   ```
3. Start the server:
   ```sh
   ./target/release/one
   ```
4. Access it from your browser at [http://localhost:7878](http://localhost:7878).

## Contributions

Contributions are welcome! Whether it's a bug report, feature suggestion, or pull request, your input improves ONE.

## License

ONE is licensed under the MIT License.

