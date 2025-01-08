# Rust Cargo Commands

This README provides a quick reference for commonly used Cargo commands when working with Rust projects. Cargo is the Rust package manager and build system that simplifies managing Rust projects.

---

## Table of Contents
1. [Setting Up a New Project](#setting-up-a-new-project)
2. [Building the Project](#building-the-project)
3. [Running the Project](#running-the-project)
4. [Testing the Project](#testing-the-project)
5. [Adding Dependencies](#adding-dependencies)
6. [Formatting Code](#formatting-code)
7. [Checking Code for Errors](#checking-code-for-errors)

---

## Setting Up a New Project

### Create a New Project
To create a new Rust project:
```bash
cargo new <project_name>
```
Example:
```bash
cargo new my_project
```
This creates the following structure:
```
my_project/
├── Cargo.toml
└── src/
    └── main.rs
```

### Initialize an Existing Directory
To turn an existing directory into a Rust project:
```bash
cargo init
```

---

## Building the Project

### Build the Project
To compile your project:
```bash
cargo build
```
This creates a debug build in the `target/debug` directory.

### Build for Release
For optimized builds:
```bash
cargo build --release
```
The release build is located in the `target/release` directory.

---

## Running the Project

To build and run your project in one step:
```bash
cargo run
```
This compiles the project (if necessary) and executes the binary.

---

## Testing the Project

Run all tests in your project:
```bash
cargo test
```
This automatically discovers and runs tests defined in your project.

---

## Adding Dependencies

To add a new dependency to your project:
```bash
cargo add <dependency_name>
```
Example:
```bash
cargo add serde
```
This updates the `Cargo.toml` file with the new dependency.

---

## Formatting Code

Ensure consistent formatting with Rust's style guidelines:
```bash
cargo fmt
```

### Install the Formatter
If `cargo fmt` is not available, install it via Rust's toolchain:
```bash
rustup component add rustfmt
```

---

## Checking Code for Errors

Check your code for warnings and errors without building:
```bash
cargo check
```
This is faster than `cargo build` as it skips the final compilation step.

---

## Additional Commands

### Clean the Build Directory
To remove the `target` directory:
```bash
cargo clean
```

### Generate Documentation
To generate and open documentation for your project and dependencies:
```bash
cargo doc --open
```

---

## Conclusion
These Cargo commands cover the essentials for managing Rust projects. For more information, refer to the [Cargo documentation](https://doc.rust-lang.org/cargo/).

