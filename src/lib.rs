//! rlimit - A simple wrapper for `getrlimit` and `setrlimit`.
//! # Example
//! ```no_run
//! # use rlimit::rlim;
//! const SOFT: rlim = 4 * 1024 * 1024;
//! const HARD: rlim = 8 * 1024 * 1024;
//!```
//!
//! ## Set resource limit
//! ```no_run
//! # use rlimit::{rlim, Resource};
//! # const SOFT: rlim = 4 * 1024 * 1024;
//! # const HARD: rlim = 8 * 1024 * 1024;
//! assert!(Resource::FSIZE.set(SOFT, HARD).is_ok());
//! ```
//! or
//! ```no_run
//! # use rlimit::{rlim, Resource, setrlimit};
//! # const SOFT: rlim = 4 * 1024 * 1024;
//! # const HARD: rlim = 8 * 1024 * 1024;
//! assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());
//!```
//!
//! ## Get resource limit
//! ```no_run
//! # use rlimit::{rlim, Resource, RLIM_INFINITY, getrlimit};
//! # const SOFT: rlim = 4 * 1024 * 1024;
//! # const HARD: rlim = 8 * 1024 * 1024;
//! assert_eq!(getrlimit(Resource::CPU).unwrap(), (RLIM_INFINITY, RLIM_INFINITY));
//! ```

#![deny(
    missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::as_conversions,
    clippy::implicit_return,
    clippy::inline_always,
    clippy::must_use_candidate
)]
#![allow(clippy::missing_errors_doc)]

#[macro_use]
extern crate cfg_if;


use std::io;
use std::mem;
use std::ptr;

pub mod errors;


cfg_if! {
    if #[cfg(all(target_os = "linux", target_env = "gnu"))]{
        use libc::__rlimit_resource_t as __resource_t;
    }else{
        use libc::c_int as __resource_t;
    }
}

use crate::errors::RlimitsError;

use libc::rlim_t as __rlim_t;
use libc::rlimit as __rlimit;

use libc::getrlimit as __getrlimit;
use libc::setrlimit as __setrlimit;

#[cfg(target_os = "linux")]
use libc::prlimit as __prlimit;

/// Unsigned integer type used for limit values.
#[allow(non_camel_case_types)]
pub type rlim = __rlim_t;

/// A value of rlim indicating no limit.
pub const RLIM_INFINITY: rlim = libc::RLIM_INFINITY;

/// Integer type used for resource values.
pub type RawResource = __resource_t;

/// Enum type used for resource values.
#[allow(clippy::cast_possible_wrap)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resource {
    /// The maximum size (in bytes)
    /// of the process's virtual memory (address space).
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    AS = libc::RLIMIT_AS as _,

    /// The maximum size (in bytes)
    /// of a core file that the process may dump.
    CORE = libc::RLIMIT_CORE as _,

    /// A limit (in seconds)
    /// on the amount of CPU time that the process can consume.
    CPU = libc::RLIMIT_CPU as _,

    /// The maximum size (in bytes)
    /// of the process's data segment
    /// (initialized data, uninitialized data, and heap).
    DATA = libc::RLIMIT_DATA as _,

    /// The maximum size (in bytes)
    /// of files that the process may create.
    FSIZE = libc::RLIMIT_FSIZE as _,

    /// A limit on the combined number
    /// of `flock(2)` locks and `fcntl(2)` leases
    /// that this process may establish.
    #[cfg(target_os = "linux")]
    LOCKS = libc::RLIMIT_LOCKS as _,

    /// The maximum number (in bytes)
    /// of memory that may be locked into RAM.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    MEMLOCK = libc::RLIMIT_MEMLOCK as _,

    /// A limit on the number
    /// of bytes that can be allocated for POSIX message queues
    /// for the real user ID of the calling process.
    #[cfg(target_os = "linux")]
    MSGQUEUE = libc::RLIMIT_MSGQUEUE as _,

    /// This specifies a ceiling
    /// to which the process's nice value can be raised
    /// using `setpriority(2)` or `nice(2)`.
    #[cfg(target_os = "linux")]
    NICE = libc::RLIMIT_NICE as _,

    /// This specifies a value
    /// one greater than the maximum file descriptor number
    /// that can be opened by this process.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    NOFILE = libc::RLIMIT_NOFILE as _,

    /// A limit on the number of extant process (or, more precisely on Linux, threads)
    /// for the real user ID of the calling process.
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    NPROC = libc::RLIMIT_NPROC as _,

    /// A limit (in bytes)
    /// on the process's resident set
    /// (the number of virtual pages resident in RAM).
    #[cfg(target_os = "linux")]
    RSS = libc::RLIMIT_RSS as _,

    /// This specifies a ceiling on the real-time priority
    /// that may be set for this process
    /// using `sched_setscheduler(2)` and `sched_setparam(2)`.
    #[cfg(target_os = "linux")]
    RTPRIO = libc::RLIMIT_RTPRIO as _,

    /// A limit (in microseconds) on the amount of CPU time
    /// that a process scheduled under a real-time scheduling policy
    /// may consume without making a blocking system call.
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    RTTIME = libc::RLIMIT_RTTIME as _,

    /// A limit on the number
    /// of signals that may be queued
    /// for the real user ID of the calling process.
    #[cfg(target_os = "linux")]
    SIGPENDING = libc::RLIMIT_SIGPENDING as _,

    /// The maximum size (in bytes)
    /// of the process stack.
    STACK = libc::RLIMIT_STACK as _,
}

impl Resource {
    /// Returns the raw resource type
    #[inline(always)]
    pub const fn as_raw_resource(self) -> RawResource {
        self as _
    }

    /// Set resource limits.
    #[inline(always)]
    pub fn set(self, soft: rlim, hard: rlim) -> io::Result<()> {
        setrlimit(self, soft, hard)
    }

    /// Get resource limits.
    #[inline(always)]
    pub fn get(self) -> std::io::Result<(rlim, rlim)> {
        getrlimit(self)
    }
}

impl std::str::FromStr for Resource {
    type Err = RlimitsError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "RLIMIT_AS" => Ok(Resource::AS),
            "RLIMIT_CORE" => Ok(Resource::CORE),
            "RLIMIT_CPU" => Ok(Resource::CPU),
            "RLIMIT_DATA" => Ok(Resource::DATA),
            "RLIMIT_FSIZE" => Ok(Resource::FSIZE),
            "RLIMIT_LOCKS" => Ok(Resource::LOCKS),
            "RLIMIT_MEMLOCK" => Ok(Resource::MEMLOCK),
            "RLIMIT_MSGQUEUE" => Ok(Resource::MSGQUEUE),
            "RLIMIT_NICE" => Ok(Resource::NICE),
            "RLIMIT_NOFILE" => Ok(Resource::NOFILE),
            "RLIMIT_NPROC" => Ok(Resource::NPROC),
            "RLIMIT_RSS" => Ok(Resource::RSS),
            "RLIMIT_RTPRIO" => Ok(Resource::RTPRIO),
            "RLIMIT_RTTIME" => Ok(Resource::RTTIME),
            "RLIMIT_SIGPENDING" => Ok(Resource::SIGPENDING),
            "RLIMIT_STACK" => Ok(Resource::STACK),
            _ => Err(format!("invalid rlimit: {}", s).into()),
        }
    }
}

/// Set resource limits.
#[inline]
pub fn setrlimit(resource: Resource, soft: rlim, hard: rlim) -> io::Result<()> {
    let raw_resource = resource.as_raw_resource();
    let limit = __rlimit {
        rlim_cur: soft,
        rlim_max: hard,
    };

    let ret = unsafe { __setrlimit(raw_resource as _, &limit) };

    if ret == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Get resource limits.
#[inline]
pub fn getrlimit(resource: Resource) -> io::Result<(rlim, rlim)> {
    let mut limit = std::mem::MaybeUninit::<__rlimit>::uninit();

    let ret = unsafe { __getrlimit(resource.as_raw_resource() as _, limit.as_mut_ptr()) };

    if ret == 0 {
        let limit = unsafe { limit.assume_init() };
        Ok((limit.rlim_cur, limit.rlim_max))
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// [Linux] Set and get the resource limits of an arbitrary process.
///
///
#[allow(clippy::similar_names, clippy::needless_pass_by_value)]
#[inline]
#[cfg(target_os = "linux")]
pub fn prlimit(
    pid: libc::pid_t,
    resource: Resource,
    new_limit: Option<(rlim, rlim)>,
    old_limit: Option<&mut (rlim, rlim)>,
) -> io::Result<()> {
    let new_rlimit: Option<__rlimit> = new_limit.map(|(soft, hard)| __rlimit {
        rlim_cur: soft,
        rlim_max: hard,
    });
    let new_rlimit_ptr: *const __rlimit = new_rlimit.as_ref().map_or(ptr::null(), |r| r);

    let mut old_rlimit: mem::MaybeUninit<__rlimit> = mem::MaybeUninit::uninit();
    let old_rlimit_ptr: *mut __rlimit = if old_limit.is_some() {
        old_rlimit.as_mut_ptr()
    } else {
        ptr::null_mut()
    };

    let ret = unsafe {
        __prlimit(
            pid,
            resource.as_raw_resource() as _,
            new_rlimit_ptr,
            old_rlimit_ptr,
        )
    };

    if ret == 0 {
        if let Some((soft, hard)) = old_limit {
            let old_rlimit: __rlimit = unsafe { old_rlimit.assume_init() };
            *soft = old_rlimit.rlim_cur;
            *hard = old_rlimit.rlim_max;
        }

        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}
