//! rlimit - A simple wrapper for `getrlimit` and `setrlimit`.
//! # Example
//! ```
//! use rlimit::{Rlim, Resource, setrlimit, getrlimit};
//! const SOFT: Rlim = Rlim::from_raw(4 * 1024 * 1024);
//! const HARD: Rlim = Rlim::from_raw(8 * 1024 * 1024);
//! ```
//!
//! ## Set resource limit
//! ```no_run
//! # use rlimit::{Rlim, Resource, setrlimit};
//! # const SOFT: Rlim = Rlim::from_raw(4 * 1024 * 1024);
//! # const HARD: Rlim = Rlim::from_raw(8 * 1024 * 1024);
//! assert!(Resource::FSIZE.set(SOFT, HARD).is_ok());
//! assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());
//! ```
//!
//! ## Get resource limit
//! ```no_run
//! # use rlimit::{Rlim, Resource, getrlimit};
//! assert!(Resource::NOFILE.get().is_ok());
//! assert_eq!(getrlimit(Resource::CPU).unwrap(), (Rlim::INFINITY, Rlim::INFINITY));
//! ```
//!
//! ## Increase NOFILE limit
//! See the example [nofile](https://github.com/Nugine/rlimit/tree/v0.5.0/examples/nofile.rs).
//!

#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

#[cfg(target_os = "windows")]
compile_error!("rlimit does not support windows");

mod resource_type;
mod rlim_type;

pub use crate::resource_type::{RawResource, Resource};
pub use crate::rlim_type::{RawRlim, Rlim};

use std::io;

use libc::c_int;
use libc::rlimit;

#[cfg(target_os = "linux")]
use std::ptr;

#[cfg(target_os = "linux")]
use libc::pid_t;

/// Set resource limits.
/// # Errors
/// [Linux] See <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
#[inline]
pub fn setrlimit(resource: Resource, soft: Rlim, hard: Rlim) -> io::Result<()> {
    let raw_resource = resource.as_raw();

    let rlim: rlimit = rlimit {
        rlim_cur: soft.as_raw(),
        rlim_max: hard.as_raw(),
    };

    let ret: c_int = unsafe { libc::setrlimit(raw_resource, &rlim) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Get resource limits.
/// # Errors
/// [Linux] See <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
#[inline]
pub fn getrlimit(resource: Resource) -> io::Result<(Rlim, Rlim)> {
    let raw_resource = resource.as_raw();

    let mut rlim: rlimit = rlimit {
        rlim_cur: 0, // zero-init
        rlim_max: 0, // zero-init
    };

    let ret: c_int = unsafe { libc::getrlimit(raw_resource, &mut rlim) };

    if ret == 0 {
        let soft = Rlim::from_raw(rlim.rlim_cur);
        let hard = Rlim::from_raw(rlim.rlim_max);
        Ok((soft, hard))
    } else {
        Err(io::Error::last_os_error())
    }
}

/// [Linux] Set and get the resource limits of an arbitrary process.
/// # Errors
/// See <https://man7.org/linux/man-pages/man2/prlimit.2.html>
#[inline]
#[cfg(target_os = "linux")]
pub fn prlimit(
    pid: pid_t,
    resource: Resource,
    new_limit: Option<(Rlim, Rlim)>,
    old_limit: Option<(&mut Rlim, &mut Rlim)>,
) -> io::Result<()> {
    let raw_resource = resource.as_raw();

    let new_rlim: Option<rlimit> = new_limit.map(|(soft, hard)| rlimit {
        rlim_cur: soft.as_raw(),
        rlim_max: hard.as_raw(),
    });

    let new_rlimit_ptr: *const rlimit = match new_rlim {
        Some(ref rlim) => rlim,
        None => ptr::null(),
    };

    let mut old_rlim: rlimit = rlimit {
        rlim_cur: 0, // zero-init
        rlim_max: 0, // zero-init
    };

    let old_rlimit_ptr: *mut rlimit = if old_limit.is_some() {
        &mut old_rlim
    } else {
        ptr::null_mut()
    };

    let ret: c_int = unsafe { libc::prlimit(pid, raw_resource, new_rlimit_ptr, old_rlimit_ptr) };

    if ret == 0 {
        if let Some((soft, hard)) = old_limit {
            *soft = Rlim::from_raw(old_rlim.rlim_cur);
            *hard = Rlim::from_raw(old_rlim.rlim_max);
        }

        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
