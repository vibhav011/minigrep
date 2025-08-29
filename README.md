# minigrep

A simple command-line tool written in Rust for searching for lines containing a query string in a file, inspired by the classic `grep` utility.

Note: This project is a learning exercise inspired by "The Rust Programming Language" book.

## Features

- Search for a string in a file
- Case-sensitive and case-insensitive search modes
- Simple and fast

## Usage

```bash
cargo run -- <query> <filename>
```

Example:

```bash
cargo run -- to poem.txt
```

To run a case-insensitive search, set the `IGNORE_CASE` environment variable:

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

## Building

```bash
cargo build --release
```

## Running Tests

```bash
cargo test
```