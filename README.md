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
const SOFT: rlim = 4 * 1024 * 1024;
const HARD: rlim = 8 * 1024 * 1024;
```

### Set resource limit
```rust
assert!(Resource::FSIZE.set(SOFT, HARD).is_ok());
```
or
```rust
assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());
```

### Get resource limit
```rust
assert_eq!(getrlimit(Resource::CPU).unwrap(), (RLIM_INFINITY, RLIM_INFINITY));
```

## Todo

Support more targets.

| Current targets           |
| ------------------------- |
| i686-unknown-linux-gnu    |
| i686-unknown-linux-musl   |
| x86_64-unknown-linux-gnu  |
| x86_64-unknown-linux-musl |
| i686-apple-darwin         |
| x86_64-apple-darwin       |


