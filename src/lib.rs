//! rlimit - A simple wrapper for `getrlimit` and `setrlimit`.
//!
//! # Examples
//!
//! ## Set resource limit
//! ```no_run
//! # #[cfg(unix)]
//! # {
//! use rlimit::{setrlimit, Resource};
//!
//! const DEFAULT_SOFT_LIMIT: u64 = 4 * 1024 * 1024;
//! const DEFAULT_HARD_LIMIT: u64 = 8 * 1024 * 1024;
//! assert!(Resource::FSIZE.set(DEFAULT_SOFT_LIMIT, DEFAULT_HARD_LIMIT).is_ok());
//!
//! let soft = 16384;
//! let hard = soft * 2;
//! assert!(setrlimit(Resource::NOFILE, soft, hard).is_ok());
//! # }
//! ```
//!
//! ## Get resource limit
//! ```no_run
//! # #[cfg(unix)]
//! # {
//! use rlimit::{getrlimit, Resource};
//!
//! assert!(Resource::NOFILE.get().is_ok());
//! assert_eq!(getrlimit(Resource::CPU).unwrap(), (rlimit::INFINITY, rlimit::INFINITY));
//! # }
//! ```
//!
//! ## Increase NOFILE limit
//! See the example [nofile](https://github.com/Nugine/rlimit/tree/v0.6.1-dev/examples/nofile.rs).
//!
//! # Troubleshoot
//!
//! ## Failed to increase NOFILE to hard limit on macOS
//! On macOS, getrlimit by default reports that the hard limit is
//! unlimited, but there is usually a stricter hard limit discoverable
//! via sysctl (`kern.maxfilesperproc`). Failing to discover this secret stricter hard limit will
//! cause the call to setrlimit to fail.
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

#[allow(unused_macros)]
macro_rules! group {
    ($($tt:tt)*) => {
        $($tt)*
    }
}

#[cfg(unix)]
group! {
    mod unix;

    #[doc(inline)]
    pub use self::unix::*;
}

pub mod utils;
