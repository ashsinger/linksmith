# Linksmith

[![Rust](https://github.com/ashsinger/linksmith/actions/workflows/rust.yml/badge.svg)](https://github.com/ashsinger/linksmith/actions/workflows/rust.yml)

Linksmith is a command-line tool that converts MediaWiki-style links to Markdown-style links in your collection of Markdown files. It simplifies the process of organizing and maintaining your notes and documentation.

## Features

- Recursively processes all Markdown files in the specified folder
- Replaces MediaWiki-style links `[[link]]` with Markdown-style links `[link name](link)`
- Handles files with the same name in different subfolders
- Provides a progress bar and summary of the changes made

## Installation


To install `linksmith` on your system, follow these steps:

### Prerequisites

Ensure you have Rust installed on your system. If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) and follow the instructions to install it.

### Building from Source

1. Clone the repository:

```bash
git clone https://github.com/ashsinger/linksmith.git
cd linksmith
```


2. Build the project:

```bash
cargo build --release
```

3. The compiled binary will be located at `target/release/linksmith`. You can either move it to a directory in your `PATH` or run it directly from the build directory:

```bash
cp target/release/linksmith /usr/local/bin/
```

Now you can use `linksmith` from the command line by typing `linksmith` followed by the appropriate arguments.

## Usage

```bash
linksmith -f /path/to/your/folder
```

This command will process all Markdown files in the specified folder and its subfolders, replacing MediaWiki-style links with their Markdown equivalents.

## Contributing

Contributions are welcome!

## License

This project is licensed under the MIT License.
