#[cfg(not(feature = "linux_raw"))]
use crate::bindings as C;
use crate::resource::Resource;

use std::io;
#[cfg(not(feature = "linux_raw"))]
use std::mem;

/// A value indicating no limit.
#[cfg(not(feature = "linux_raw"))]
#[allow(clippy::unnecessary_cast)]
pub const INFINITY: u64 = C::RLIM_INFINITY as u64;

/// A value indicating no limit.
#[cfg(feature = "linux_raw")]
pub const INFINITY: u64 = u64::MAX;

#[cfg(feature = "linux_raw")]
fn to_rustix_resource(resource: Resource) -> io::Result<rustix::process::Resource> {
    use rustix::process::Resource as R;
    match resource {
        Resource::AS => Ok(R::As),
        Resource::CORE => Ok(R::Core),
        Resource::CPU => Ok(R::Cpu),
        Resource::DATA => Ok(R::Data),
        Resource::FSIZE => Ok(R::Fsize),
        Resource::LOCKS => Ok(R::Locks),
        Resource::MEMLOCK => Ok(R::Memlock),
        Resource::MSGQUEUE => Ok(R::Msgqueue),
        Resource::NICE => Ok(R::Nice),
        Resource::NOFILE => Ok(R::Nofile),
        Resource::NPROC => Ok(R::Nproc),
        Resource::RSS => Ok(R::Rss),
        Resource::RTPRIO => Ok(R::Rtprio),
        Resource::RTTIME => Ok(R::Rttime),
        Resource::SIGPENDING => Ok(R::Sigpending),
        Resource::STACK => Ok(R::Stack),
        // These resources are not available in rustix's linux_raw backend
        _ => Err(io::Error::new(io::ErrorKind::Other, "unsupported resource")),
    }
}

#[cfg(not(feature = "linux_raw"))]
fn check_supported(resource: Resource) -> io::Result<()> {
    let raw_resource = resource.as_raw();
    if raw_resource == u8::MAX {
        return Err(io::Error::new(io::ErrorKind::Other, "unsupported resource"));
    }
    Ok(())
}

/// Set resource limits.
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
#[cfg(feature = "linux_raw")]
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn setrlimit(resource: Resource, soft: u64, hard: u64) -> io::Result<()> {
    let rustix_resource = to_rustix_resource(resource)?;
    let rlimit = rustix::process::Rlimit {
        current: if soft == INFINITY { None } else { Some(soft) },
        maximum: if hard == INFINITY { None } else { Some(hard) },
    };
    rustix::process::setrlimit(rustix_resource, rlimit).map_err(|e| e.into())
}

/// Set resource limits.
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
#[cfg(not(feature = "linux_raw"))]
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
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
#[cfg(feature = "linux_raw")]
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn getrlimit(resource: Resource) -> io::Result<(u64, u64)> {
    let rustix_resource = to_rustix_resource(resource)?;
    let rlimit = rustix::process::getrlimit(rustix_resource);
    let soft = rlimit.current.unwrap_or(INFINITY);
    let hard = rlimit.maximum.unwrap_or(INFINITY);
    Ok((soft, hard))
}

/// Get resource limits.
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
#[cfg(not(feature = "linux_raw"))]
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
/// # Errors
/// See <https://man7.org/linux/man-pages/man2/prlimit.2.html>
#[cfg(all(feature = "linux_raw", any(target_os = "linux", target_os = "android")))]
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
pub fn prlimit(
    pid: pid_t,
    resource: Resource,
    new_limit: Option<(u64, u64)>,
    old_limit: Option<(&mut u64, &mut u64)>,
) -> io::Result<()> {
    let rustix_resource = to_rustix_resource(resource)?;
    
    // Handle the case where we just want to get limits (no new limit to set)
    if new_limit.is_none() && pid == 0 {
        // For current process, use getrlimit
        let rlimit = rustix::process::getrlimit(rustix_resource);
        if let Some((soft, hard)) = old_limit {
            *soft = rlimit.current.unwrap_or(INFINITY);
            *hard = rlimit.maximum.unwrap_or(INFINITY);
        }
        return Ok(());
    }

    let rustix_pid = if pid == 0 {
        None
    } else {
        Some(rustix::process::Pid::from_raw(pid).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "invalid pid")
        })?)
    };

    // When new_limit is None but pid != 0, we need to use prlimit with the current values
    // to get the old values without changing them
    let new_rlimit = if let Some((soft, hard)) = new_limit {
        rustix::process::Rlimit {
            current: if soft == INFINITY { None } else { Some(soft) },
            maximum: if hard == INFINITY { None } else { Some(hard) },
        }
    } else {
        // Get current values first to use as "new" values (no change)
        let current = rustix::process::getrlimit(rustix_resource);
        current
    };

    let old_rlimit = rustix::process::prlimit(rustix_pid, rustix_resource, new_rlimit)
        .map_err(|e| io::Error::from(e))?;

    if let Some((soft, hard)) = old_limit {
        *soft = old_rlimit.current.unwrap_or(INFINITY);
        *hard = old_rlimit.maximum.unwrap_or(INFINITY);
    }

    Ok(())
}

/// Set and get the resource limits of an arbitrary process.
/// # Errors
/// See <https://man7.org/linux/man-pages/man2/prlimit.2.html>
#[cfg(all(not(feature = "linux_raw"), any(doc, rlimit__has_prlimit64)))]
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
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
