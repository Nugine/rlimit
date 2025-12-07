# rlimit

[![Latest Version]][crates.io]
[![Documentation]][docs.rs] 
[![License]](LICENSE)
[![Downloads]][downloads]

A Rust library for getting and setting resource limits on Unix and Windows systems.

[crates.io]: https://crates.io/crates/rlimit
[Latest Version]: https://img.shields.io/crates/v/rlimit.svg
[Documentation]: https://docs.rs/rlimit/badge.svg
[docs.rs]: https://docs.rs/rlimit
[License]: https://img.shields.io/crates/l/rlimit.svg
[downloads]: https://img.shields.io/crates/d/rlimit

## Overview

This crate provides a safe interface to system resource limits. Resource limits control the amount of system resources (such as CPU time, memory, open files, etc.) that a process can consume.

On Unix-like systems, this library wraps the `getrlimit`, `setrlimit`, and `prlimit` system calls. On Windows, it provides limited support for controlling the number of open file handles via `_getmaxstdio` and `_setmaxstdio`.

## Platform Support

- **Unix/Linux**: Full support for all resource limit types
- **macOS**: Full support with special handling for `NOFILE` limits
- **Windows**: Limited support (file handle limits only)

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
rlimit = "0.10"
```

## Usage Examples

### Get resource limits

```rust
use rlimit::{getrlimit, Resource};

fn main() {
    // Get the current NOFILE (maximum number of open files) limits
    match Resource::NOFILE.get() {
        Ok((soft, hard)) => {
            println!("NOFILE limits: soft={}, hard={}", soft, hard);
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Alternative syntax
    let (soft, hard) = getrlimit(Resource::CPU).unwrap();
    println!("CPU limits: soft={}, hard={}", soft, hard);
}
```

### Set resource limits

```rust
use rlimit::{setrlimit, Resource};

fn main() {
    // Set NOFILE limits (soft=1024, hard=2048)
    if let Err(e) = Resource::NOFILE.set(1024, 2048) {
        eprintln!("Failed to set NOFILE limits: {}", e);
    }
    
    // Alternative syntax
    let soft = 4 * 1024 * 1024; // 4 MB
    let hard = 8 * 1024 * 1024; // 8 MB
    setrlimit(Resource::FSIZE, soft, hard).unwrap();
}
```

### Increase NOFILE limit to maximum

```rust
use rlimit::increase_nofile_limit;

fn main() {
    // Try to increase NOFILE to the specified limit (or the maximum possible)
    match increase_nofile_limit(10240) {
        Ok(limit) => println!("NOFILE limit increased to {}", limit),
        Err(e) => eprintln!("Failed to increase NOFILE limit: {}", e),
    }
    
    // Try to set to the highest possible value
    increase_nofile_limit(u64::MAX).unwrap();
}
```

### Linux-specific: prlimit

```rust
#[cfg(target_os = "linux")]
use rlimit::{prlimit, Resource};

#[cfg(target_os = "linux")]
fn main() {
    let pid = 0; // 0 means current process
    
    // Set new limit and get old limit
    let mut old_soft = 0;
    let mut old_hard = 0;
    
    prlimit(
        pid,
        Resource::NOFILE,
        Some((2048, 4096)),
        Some((&mut old_soft, &mut old_hard)),
    ).unwrap();
    
    println!("Previous limits: soft={}, hard={}", old_soft, old_hard);
}
```

### Windows

```rust
#[cfg(windows)]
fn main() {
    println!("Current max stdio: {}", rlimit::getmaxstdio());
    rlimit::setmaxstdio(2048).unwrap();
    println!("New max stdio: {}", rlimit::getmaxstdio());
}
```

## Available Resources

The library supports various resource types through the `Resource` enum:

- `Resource::AS` - Maximum size of virtual memory
- `Resource::CORE` - Maximum core file size
- `Resource::CPU` - CPU time limit in seconds
- `Resource::DATA` - Maximum data segment size
- `Resource::FSIZE` - Maximum file size
- `Resource::NOFILE` - Maximum number of open file descriptors
- `Resource::STACK` - Maximum stack size
- `Resource::NPROC` - Maximum number of processes/threads
- And many more (platform-dependent)

See the [documentation](https://docs.rs/rlimit) for a complete list.

## Documentation

Full API documentation: <https://docs.rs/rlimit>

## Contributing

+ [Development Guide](./CONTRIBUTING.md)

## Sponsor

If my open-source work has been helpful to you, please [sponsor me](https://github.com/Nugine#sponsor).

Every little bit helps. Thank you!
