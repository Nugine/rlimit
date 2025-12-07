//! rlimit - Resource limits.
//!
//! A cross-platform library for getting and setting resource limits.
//!
//! ## What are resource limits?
//!
//! Resource limits (rlimits) are restrictions placed by the operating system on the resources
//! that a process can consume. Each limit has two values:
//!
//! - **Soft limit**: The current limit enforced by the kernel. A process can lower or raise this
//!   up to the hard limit.
//! - **Hard limit**: The maximum value that the soft limit can be set to. Only privileged processes
//!   can raise the hard limit.
//!
//! Common use cases include:
//! - Preventing processes from consuming too much memory
//! - Limiting the number of open file descriptors
//! - Restricting CPU time usage
//! - Controlling the size of files that can be created
//!
//! ## Basic Usage
//!
//! ### Get resource limit
//! ```no_run
//! # #[cfg(unix)]
//! # {
//! use rlimit::{getrlimit, Resource};
//!
//! // Get NOFILE (max open files) limits
//! let (soft, hard) = Resource::NOFILE.get().unwrap();
//! println!("NOFILE limits: soft={}, hard={}", soft, hard);
//!
//! // Alternative function-style syntax
//! assert!(getrlimit(Resource::CPU).is_ok());
//! # }
//! ```
//!
//! ### Set resource limit
//! ```no_run
//! # #[cfg(unix)]
//! # {
//! use rlimit::{setrlimit, Resource};
//!
//! // Set FSIZE (max file size) limits using method syntax
//! const DEFAULT_SOFT_LIMIT: u64 = 4 * 1024 * 1024;  // 4 MB
//! const DEFAULT_HARD_LIMIT: u64 = 8 * 1024 * 1024;  // 8 MB
//! Resource::FSIZE.set(DEFAULT_SOFT_LIMIT, DEFAULT_HARD_LIMIT).unwrap();
//!
//! // Set NOFILE limits using function syntax
//! let soft = 1024;
//! let hard = 2048;
//! setrlimit(Resource::NOFILE, soft, hard).unwrap();
//! # }
//! ```
//!
//! ### Error handling
//! ```no_run
//! # #[cfg(unix)]
//! # {
//! use rlimit::Resource;
//!
//! match Resource::NOFILE.set(10000, 20000) {
//!     Ok(_) => println!("Successfully set NOFILE limits"),
//!     Err(e) => eprintln!("Failed to set NOFILE limits: {}", e),
//! }
//! # }
//! ```
//!
//! ## Windows Support
//!
//! Windows does not have Unix-like resource limits.
//! It only supports changing the number of simultaneously open files currently permitted at the stdio level.
//!
//! See the official documentation of
//! [`_getmaxstdio`](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/getmaxstdio?view=msvc-170)
//! and
//! [`_setmaxstdio`](https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/setmaxstdio?view=msvc-170).
//!
//! ```no_run
//! # #[cfg(windows)]
//! # {
//! // Query the current limit
//! let current = rlimit::getmaxstdio();
//! println!("Current max stdio: {}", current); // Default is usually 512
//!
//! // Set a new limit
//! rlimit::setmaxstdio(2048).unwrap();
//! println!("New max stdio: {}", rlimit::getmaxstdio()); // 2048
//! # }
//! ```
//!
//! ## Increase NOFILE limit
//!
//! A common task is to increase the maximum number of open files (`NOFILE`). This library
//! provides a convenient helper function [`increase_nofile_limit`] that handles platform-specific
//! quirks automatically.
//!
//! ```no_run
//! use rlimit::increase_nofile_limit;
//!
//! // Try to set NOFILE to 10240 (or the maximum possible if lower)
//! match increase_nofile_limit(10240) {
//!     Ok(limit) => println!("NOFILE limit set to {}", limit),
//!     Err(e) => eprintln!("Failed to increase NOFILE: {}", e),
//! }
//!
//! // Try to set to the maximum possible value
//! increase_nofile_limit(u64::MAX).unwrap();
//! ```
//!
//! For a more detailed demonstration, see the `nofile.rs` example in the repository.
//!
//! ## Linux-specific: prlimit
//!
//! On Linux, you can use [`prlimit`] to get and set resource limits for any process,
//! not just the current one.
//!
//! ```no_run
//! # #[cfg(target_os = "linux")]
//! # {
//! use rlimit::{prlimit, Resource};
//!
//! let pid = 0; // 0 means current process
//! let mut old_soft = 0;
//! let mut old_hard = 0;
//!
//! // Set new limits and retrieve old limits in one call
//! prlimit(
//!     pid,
//!     Resource::NOFILE,
//!     Some((2048, 4096)),                    // new limits
//!     Some((&mut old_soft, &mut old_hard)),  // old limits
//! ).unwrap();
//!
//! println!("Previous NOFILE: soft={}, hard={}", old_soft, old_hard);
//! # }
//! ```
//!
//! # Troubleshooting
//!
//! ## Failed to increase NOFILE to hard limit on macOS
//!
//! On macOS, `getrlimit` by default reports that the hard limit is unlimited, but there is
//! usually a stricter hard limit discoverable via `sysctl` (`kern.maxfilesperproc`).
//!
//! If you try to set the limit higher than this hidden maximum, `setrlimit` will fail with
//! an error. To avoid this issue, use [`increase_nofile_limit`] which automatically respects
//! `kern.maxfilesperproc` on macOS.
//!
//! ## Permission denied errors
//!
//! Setting resource limits may fail with permission denied errors if:
//! - You're trying to raise the hard limit without sufficient privileges
//! - You're trying to set limits for another process without appropriate permissions
//!
//! Generally, only privileged processes (root) can raise hard limits. Unprivileged processes
//! can only lower limits or raise the soft limit up to the hard limit.
//!

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::cargo
)]
#![allow(
    clippy::option_if_let_else,  // I don't like it. The match expression is more readable.
)]

#[allow(unused_macros)]
macro_rules! group {
    ($($item:item)*) => {
        $($item)*
    }
}

#[cfg(any(doc, windows))]
group! {
    mod windows;

    #[doc(inline)]
    pub use self::windows::*;
}

#[cfg(any(doc, unix))]
group! {
    mod bindings;

    mod unix;
    mod resource;

    #[doc(inline)]
    pub use self::unix::*;

    #[doc(inline)]
    pub use self::resource::Resource;
}

#[cfg(any(doc, target_os = "linux", target_os = "android"))]
group! {
    mod proc_limits;

    #[doc(inline)]
    pub use self::proc_limits::*;
}

mod tools;
#[doc(inline)]
pub use self::tools::*;

#[cfg(test)]
mod tests {
    #[allow(clippy::assertions_on_constants)]
    #[test]
    fn build_cfg() {
        if cfg!(target_os = "linux") || cfg!(target_os = "android") {
            assert!(cfg!(rlimit__has_prlimit64));
            assert!(cfg!(not(rlimit__get_kern_max_files_per_proc)));
        }

        if cfg!(target_os = "macos") {
            assert!(cfg!(not(rlimit__has_prlimit64)));
            assert!(cfg!(rlimit__get_kern_max_files_per_proc));
        }
    }
}
