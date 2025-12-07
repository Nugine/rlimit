# Development Guide

This guide will help you get started with contributing to the rlimit project.

## Prerequisites

### Required Tools

+ [Rust 1.65.0 or newer](https://rustup.rs/)
+ [just](https://github.com/casey/just) - A command runner

### Getting the Source Code

```bash
git clone https://github.com/Nugine/rlimit.git
cd rlimit
```

## Development Workflow

### Run Basic Checks and Tests

This will run clippy, tests, and other quality checks:

```bash
just dev
```

### Build Documentation

Build and open the documentation in your browser:

```bash
just doc
```

### Run the Codegen

If you need to regenerate bindings or generated code:

```bash
just codegen
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Tests

```bash
# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test it

# Run doc tests
cargo test --doc
```

### Test on Different Platforms

Resource limits are platform-specific. When possible, test your changes on:
- Linux
- macOS
- FreeBSD (if applicable)

## Code Quality

### Run Clippy

```bash
cargo clippy --all-targets --all-features
```

### Format Code

```bash
cargo fmt
```

### Check Documentation

Build documentation and check for warnings:

```bash
cargo doc --no-deps
```

## Submitting Changes

1. **Create a branch** for your changes
2. **Write tests** for new functionality
3. **Update documentation** as needed
4. **Run all checks** with `just dev`
5. **Submit a pull request**

## Project Structure

- `src/` - Main library source code
  - `lib.rs` - Library entry point and documentation
  - `unix.rs` - Unix-specific implementations
  - `windows.rs` - Windows-specific implementations
  - `resource/` - Resource type definitions
  - `tools.rs` - Helper functions
- `tests/` - Integration tests
- `examples/` - Example programs
- `codegen/` - Code generation utilities

## Documentation Standards

- Add doc comments (`///`) for all public items
- Include examples in documentation where helpful
- Cross-reference related functions with markdown links
- Test code examples with `cargo test --doc`

## Getting Help

If you have questions or need help:
- Open an issue on GitHub
- Check existing issues and pull requests
- Review the [main documentation](https://docs.rs/rlimit)

Thank you for contributing!
