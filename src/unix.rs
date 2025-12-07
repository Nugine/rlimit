use crate::bindings as C;
use crate::resource::Resource;

use std::{io, mem};

/// A value indicating no limit.
#[allow(clippy::unnecessary_cast)]
pub const INFINITY: u64 = C::RLIM_INFINITY as u64;

fn check_supported(resource: Resource) -> io::Result<()> {
    let raw_resource = resource.as_raw();
    if raw_resource == u8::MAX {
        return Err(io::Error::new(io::ErrorKind::Other, "unsupported resource"));
    }
    Ok(())
}

/// Set resource limits.
///
/// Sets both the soft and hard limits for the specified resource.
///
/// # Parameters
///
/// - `resource`: The resource type to set limits for (e.g., [`Resource::NOFILE`])
/// - `soft`: The soft limit value (current enforced limit)
/// - `hard`: The hard limit value (maximum the soft limit can be raised to)
///
/// # Notes
///
/// - The soft limit must be less than or equal to the hard limit
/// - Only privileged processes can raise the hard limit
/// - Unprivileged processes can only lower limits or raise soft limit up to hard limit
/// - Use [`INFINITY`] to specify no limit
///
/// # Examples
///
/// ```no_run
/// # #[cfg(unix)]
/// # {
/// use rlimit::{setrlimit, Resource};
///
/// // Set NOFILE limits: soft=1024, hard=2048
/// setrlimit(Resource::NOFILE, 1024, 2048).unwrap();
///
/// // Set unlimited CPU time
/// setrlimit(Resource::CPU, rlimit::INFINITY, rlimit::INFINITY).unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The resource is not supported on this platform
/// - Permission denied (trying to raise hard limit without privileges)
/// - Invalid argument (soft > hard, or values out of range)
///
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn setrlimit(resource: Resource, soft: u64, hard: u64) -> io::Result<()> {
    check_supported(resource)?;
    let rlim = C::rlimit {
        rlim_cur: soft.min(INFINITY) as _,
        rlim_max: hard.min(INFINITY) as _,
    };
    #[allow(clippy::cast_lossless)]
    let ret = unsafe { C::setrlimit(resource.as_raw() as _, &rlim) };
    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// Get resource limits.
///
/// Retrieves both the soft and hard limits for the specified resource.
///
/// # Parameters
///
/// - `resource`: The resource type to query (e.g., [`Resource::NOFILE`])
///
/// # Returns
///
/// Returns a tuple `(soft, hard)` where:
/// - `soft`: The current soft limit (enforced limit)
/// - `hard`: The hard limit (maximum the soft limit can be raised to)
///
/// # Examples
///
/// ```no_run
/// # #[cfg(unix)]
/// # {
/// use rlimit::{getrlimit, Resource};
///
/// let (soft, hard) = getrlimit(Resource::NOFILE).unwrap();
/// println!("NOFILE: soft={}, hard={}", soft, hard);
///
/// // Check if a resource has unlimited limits
/// let (cpu_soft, cpu_hard) = getrlimit(Resource::CPU).unwrap();
/// if cpu_soft == rlimit::INFINITY {
///     println!("CPU time is unlimited");
/// }
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The resource is not supported on this platform
/// - The system call fails
///
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn getrlimit(resource: Resource) -> io::Result<(u64, u64)> {
    check_supported(resource)?;
    let mut rlim = unsafe { mem::zeroed() };
    #[allow(clippy::cast_lossless)]
    let ret = unsafe { C::getrlimit(resource.as_raw() as _, &mut rlim) };

    #[allow(clippy::unnecessary_cast)]
    if ret == 0 {
        let soft = (rlim.rlim_cur as u64).min(INFINITY);
        let hard = (rlim.rlim_max as u64).min(INFINITY);
        Ok((soft, hard))
    } else {
        Err(io::Error::last_os_error())
    }
}

/// The type of a process ID
#[allow(non_camel_case_types)]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
#[cfg(any(doc, target_os = "linux", target_os = "android"))]
pub type pid_t = i32;

/// Set and get the resource limits of an arbitrary process.
///
/// Linux-specific function that can get and set resource limits for any process,
/// not just the current one. This is more flexible than [`getrlimit`] and [`setrlimit`].
///
/// # Parameters
///
/// - `pid`: Process ID (use `0` for the current process)
/// - `resource`: The resource type to query/modify
/// - `new_limit`: Optional new limits as `(soft, hard)`. Pass `None` to only query.
/// - `old_limit`: Optional mutable references to receive the old limits. Pass `None` if not needed.
///
/// # Examples
///
/// ```no_run
/// # #[cfg(target_os = "linux")]
/// # {
/// use rlimit::{prlimit, Resource};
///
/// // Get limits for the current process
/// let mut soft = 0;
/// let mut hard = 0;
/// prlimit(0, Resource::NOFILE, None, Some((&mut soft, &mut hard))).unwrap();
/// println!("Current NOFILE: soft={}, hard={}", soft, hard);
///
/// // Set new limits and get old limits in one call
/// let mut old_soft = 0;
/// let mut old_hard = 0;
/// prlimit(
///     0,
///     Resource::NOFILE,
///     Some((2048, 4096)),
///     Some((&mut old_soft, &mut old_hard)),
/// ).unwrap();
/// println!("Changed from soft={}, hard={}", old_soft, old_hard);
///
/// // Set limits for another process (requires permissions)
/// let other_pid = 1234;
/// prlimit(other_pid, Resource::CPU, Some((60, 120)), None).unwrap();
/// # }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The resource is not supported
/// - Permission denied (accessing other processes requires privileges)
/// - Invalid PID or resource values
///
/// See <https://man7.org/linux/man-pages/man2/prlimit.2.html>
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
#[cfg(any(doc, rlimit__has_prlimit64))]
pub fn prlimit(
    pid: pid_t,
    resource: Resource,
    new_limit: Option<(u64, u64)>,
    old_limit: Option<(&mut u64, &mut u64)>,
) -> io::Result<()> {
    check_supported(resource)?;

    let new_rlim: Option<C::rlimit> = new_limit.map(|(soft, hard)| C::rlimit {
        rlim_cur: soft.min(INFINITY) as _,
        rlim_max: hard.min(INFINITY) as _,
    });

    let new_rlimit_ptr: *const C::rlimit = match new_rlim {
        Some(ref rlim) => rlim,
        None => std::ptr::null(),
    };

    let mut old_rlim: C::rlimit = unsafe { mem::zeroed() };

    let old_rlimit_ptr: *mut C::rlimit = if old_limit.is_some() {
        &mut old_rlim
    } else {
        std::ptr::null_mut()
    };

    #[allow(clippy::cast_lossless)]
    let ret = unsafe { C::prlimit(pid, resource.as_raw() as _, new_rlimit_ptr, old_rlimit_ptr) };

    if ret == 0 {
        #[allow(clippy::unnecessary_cast)]
        if let Some((soft, hard)) = old_limit {
            *soft = (old_rlim.rlim_cur as u64).min(INFINITY);
            *hard = (old_rlim.rlim_max as u64).min(INFINITY);
        }

        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
