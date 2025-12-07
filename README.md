# rlimit

[![Latest Version]][crates.io]
[![Documentation]][docs.rs] 
[![License]](LICENSE)
[![Downloads]][downloads]

Resource limits.

[crates.io]: https://crates.io/crates/rlimit
[Latest Version]: https://img.shields.io/crates/v/rlimit.svg
[Documentation]: https://docs.rs/rlimit/badge.svg
[docs.rs]: https://docs.rs/rlimit
[License]: https://img.shields.io/crates/l/rlimit.svg
[downloads]: https://img.shields.io/crates/d/rlimit

Documentation: <https://docs.rs/rlimit>

## Features

### Linux raw syscalls

On Linux, you can enable the `linux_raw` feature to use direct syscalls via [rustix](https://crates.io/crates/rustix) instead of going through libc. This enables **pure Rust** binaries without libc dependency.

Add this to your `Cargo.toml`:
```toml
[dependencies]
rlimit = { version = "0.10", features = ["linux_raw"] }
```

## Contributing

+ [Development Guide](./CONTRIBUTING.md)

## Sponsor

If my open-source work has been helpful to you, please [sponsor me](https://github.com/Nugine#sponsor).

Every little bit helps. Thank you!
