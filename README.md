# Learning Rust 🦀

> ⚠️ **Educational purposes only.**
> This repository is a personal learning space. The code here is written while learning Rust and is **not intended for production use**. It may contain mistakes, unidiomatic patterns or incomplete exercises.

## About

This repository contains my exercises and experiments while working through
[*The Rust Programming Language*](https://doc.rust-lang.org/book/) (a.k.a. "the Rust Book").

I'm a student at 42 and I'm learning Rust on my own, alongside the common core curriculum.
The goal is simply to understand the language step by step: ownership, borrowing, lifetimes,
traits, error handling, and everything in between.

## Structure

Each folder is a standalone Cargo project, usually matching a chapter or a concept from the book.

```
.
├── README.md
├── .gitignore
└── <project_name>/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Getting started

### Prerequisites

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Run an exercise

```bash
git clone <this-repo-url>
cd <repo>/<project_name>
cargo run
```

### Useful commands

| Command             | Description                                  |
|---------------------|----------------------------------------------|
| `cargo build`       | Compile the project (debug mode)             |
| `cargo run`         | Compile and run                              |
| `cargo check`       | Check that the code compiles, without building a binary |
| `cargo clippy`      | Lint the code                                |
| `cargo fmt`         | Format the code                              |
| `cargo clean`       | Remove the `target/` directory               |

## Progress

- [ ] Getting Started
- [ ] Programming a Guessing Game
- [ ] Common Programming Concepts
- [ ] Understanding Ownership
- [ ] Using Structs
- [ ] Enums and Pattern Matching
- [ ] Packages, Crates and Modules
- [ ] Common Collections
- [ ] Error Handling
- [ ] Generic Types, Traits and Lifetimes
- [ ] Writing Automated Tests
- [ ] An I/O Project: Building a Command Line Program
- [ ] Functional Language Features: Iterators and Closures
- [ ] More About Cargo and Crates.io
- [ ] Smart Pointers
- [ ] Fearless Concurrency
- [ ] Async and Await
- [ ] Object-Oriented Programming Features
- [ ] Patterns and Matching
- [ ] Advanced Features
- [ ] Final Project: Building a Multithreaded Web Server

## Resources

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Rust Standard Library docs](https://doc.rust-lang.org/std/)

## License

This project is for personal educational use. Feel free to browse it, but keep in mind it's a learning playground, not a reference.
