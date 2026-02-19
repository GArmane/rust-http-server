# Rust HTTP Server

This is a basic HTTP server built in Rust, following the ["The Rust Programming Language"](https://doc.rust-lang.org/book/) book. This project is for educational purposes only.

## Features

- **Basic HTTP server** with single-threaded and multi-threaded modes
- **Handles GET requests** for static files
- **Customizable response** with HTML content
- **Graceful shutdown** support

## Table of Contents

- [Getting Started](#getting-started)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)

# Getting Started

These instructions will help you set up and run the Rust HTTP Server on your local machine.
Prerequisites

* Rust (version 1.82.0 or higher)

To check your Rust installation, run:

```bash
$ rustc --version
```


# Installation

1. Clone this repository:

```bash
$ git clone https://github.com/GArmane/rust-http-server.git
$ cd rust-http-server
```

2. Run tests to ensure the server works as expected:

```bash
$ cargo test
```

3. Build project with cargo:

```bash
$ cargo build --release
```

# Usage

To start the server, run:

```bash
$ cargo run
```

The server will start on 127.0.0.1:7878 by default. You can access it by opening a web browser and navigating to http://127.0.0.1:7878.

## Example

Once the server is running, it can serve static HTML files placed in a predefined directory (e.g., ./public). For example, placing an index.html file in the public directory will make it accessible at http://127.0.0.1:7878/index.html.

# Configuration

To change the port or host address, update the code in main.rs:
```rust
let addr = "127.0.0.1:7878";
```
You can also extend the server’s functionality by adding more route handlers or response types.

# Next tasks

These are the next tasks, suggested by the Rust Book.

1. Add more documentation to ThreadPool and its public methods.
2. Add tests of the library’s functionality.
3. Change calls to unwrap to more robust error handling.
4. Use ThreadPool to perform some task other than serving web requests.
5. Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.

# License

This project is licensed under The Unlicense - see the LICENSE file for details.
