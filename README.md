# rlimit

[![Latest Version]][crates.io]
[![Documentation]][docs.rs] 
![License]

A simple wrapper for `getrlimit` and `setrlimit`.

[crates.io]: https://crates.io/crates/rlimit
[Latest Version]: https://img.shields.io/crates/v/rlimit.svg
[Documentation]: https://docs.rs/rlimit/badge.svg
[docs.rs]: https://docs.rs/rlimit
[License]: https://img.shields.io/crates/l/rlimit.svg

## Example

```rust
use rlimit::{Rlim, Resource, setrlimit, getrlimit};
const SOFT: Rlim = Rlim::from_raw(4 * 1024 * 1024);
const HARD: Rlim = Rlim::from_raw(8 * 1024 * 1024);
```

### Set resource limit

```rust
assert!(Resource::FSIZE.set(SOFT, HARD).is_ok());
assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());
```

### Get resource limit

```rust
assert!(Resource::NOFILE.get().is_ok());
assert_eq!(getrlimit(Resource::CPU).unwrap(), (Rlim::INFINITY, Rlim::INFINITY));
```
