# rlimit

[![Latest Version]][crates.io]
[![Documentation]][docs.rs] 
[![License]](LICENSE)
[![Downloads]][downloads]

Resource limits.

## Features

### `asm` - Direct syscalls without libc (Linux only)

The `asm` feature enables direct syscalls using inline assembly on Linux, bypassing libc entirely. This allows for creating pure Rust binaries without any libc dependency for rlimit operations.

**Supported architectures:**
- x86_64
- aarch64 (ARM64)  
- x86 (32-bit)
- arm (32-bit)
- riscv64

**Usage:**
```toml
[dependencies]
rlimit = { version = "0.10", features = ["asm"] }
```

[crates.io]: https://crates.io/crates/rlimit
[Latest Version]: https://img.shields.io/crates/v/rlimit.svg
[Documentation]: https://docs.rs/rlimit/badge.svg
[docs.rs]: https://docs.rs/rlimit
[License]: https://img.shields.io/crates/l/rlimit.svg
[downloads]: https://img.shields.io/crates/d/rlimit

Documentation: <https://docs.rs/rlimit>

## Contributing

+ [Development Guide](./CONTRIBUTING.md)

## Sponsor

If my open-source work has been helpful to you, please [sponsor me](https://github.com/Nugine#sponsor).

Every little bit helps. Thank you!
