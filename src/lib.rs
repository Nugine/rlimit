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

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(all(target_os = "linux", target_env = "gnu"))]{
        use libc::__rlimit_resource_t as __resource_t;
    }else{
        use libc::c_int as __resource_t;
    }
}

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
    #[inline(always)]
    pub fn as_raw_resource(&self) -> RawResource {
        *self as _
    }

    /// Set resource limits.
    #[inline(always)]
    pub fn set(&self, soft: rlim, hard: rlim) -> std::io::Result<()> {
        setrlimit(*self, soft, hard)
    }

    /// Get resource limits.
    #[inline(always)]
    pub fn get(&self) -> std::io::Result<(rlim, rlim)> {
        getrlimit(*self)
    }
}

/// Set resource limits.
pub fn setrlimit(resource: Resource, soft: rlim, hard: rlim) -> std::io::Result<()> {
    let resource = resource.as_raw_resource();
    let limit = __rlimit {
        rlim_cur: soft,
        rlim_max: hard,
    };

    let ret = unsafe { __setrlimit(resource as _, &limit) };

    if ret == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// Get resource limits.
pub fn getrlimit(resource: Resource) -> std::io::Result<(rlim, rlim)> {
    let resource = resource.as_raw_resource();
    let mut limit = std::mem::MaybeUninit::<__rlimit>::uninit();

    let ret = unsafe { __getrlimit(resource as _, limit.as_mut_ptr()) };

    if ret == 0 {
        let limit = unsafe { limit.assume_init() };
        Ok((limit.rlim_cur, limit.rlim_max))
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// [Linux] Set the resource limits of an arbitrary process.
#[cfg(target_os = "linux")]
pub fn prlimit(
    pid: libc::pid_t,
    resource: Resource,
    new_limit: Option<(rlim, rlim)>,
) -> std::io::Result<()> {
    let resource = resource.as_raw_resource();

    let new_rlimit = new_limit.map(|(soft, hard)| __rlimit {
        rlim_cur: soft,
        rlim_max: hard,
    });
    let new_rlimit_ptr = new_rlimit
        .as_ref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null());
    let ret = unsafe { __prlimit(pid, resource, new_rlimit_ptr, std::ptr::null_mut()) };
    if ret == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

/// [Linux] Set and get the resource limits of an arbitrary process.
#[cfg(target_os = "linux")]
pub fn prlimit_with_old(
    pid: libc::pid_t,
    resource: Resource,
    new_limit: Option<(rlim, rlim)>,
) -> std::io::Result<(rlim, rlim)> {
    let resource = resource.as_raw_resource();

    let new_rlimit = new_limit.map(|(soft, hard)| __rlimit {
        rlim_cur: soft,
        rlim_max: hard,
    });
    let new_rlimit_ptr = new_rlimit
        .as_ref()
        .map(|r| r as *const _)
        .unwrap_or(std::ptr::null());

    let mut old_rlimit = std::mem::MaybeUninit::<__rlimit>::uninit();
    let ret = unsafe { __prlimit(pid, resource, new_rlimit_ptr, old_rlimit.as_mut_ptr()) };

    if ret == 0 {
        let old_rlimit = unsafe { old_rlimit.assume_init() };
        Ok((old_rlimit.rlim_cur, old_rlimit.rlim_max))
    } else {
        Err(std::io::Error::last_os_error())
    }
}
