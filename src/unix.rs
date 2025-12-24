use crate::bindings as C;
use crate::resource::Resource;

use std::{io, mem};

#[cfg(rlimit__asm_syscall)]
use crate::linux_asm::{getrlimit_syscall, prlimit_syscall, setrlimit_syscall};

/// A value indicating no limit.
///
/// This constant is the minimum of the platform's `RLIM_INFINITY` and `u64::MAX`.
/// On most platforms, `RLIM_INFINITY` is already `u64`, making this cast lossless.
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
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/setrlimit.2.html>
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn setrlimit(resource: Resource, soft: u64, hard: u64) -> io::Result<()> {
    check_supported(resource)?;
    // SAFETY: Values are clamped to INFINITY, which is the maximum value representable
    // by the platform's rlim_t type. On platforms where rlim_t is u64, this is a no-op.
    // On platforms where rlim_t is smaller (e.g., u32), the min() ensures we don't
    // exceed the platform's maximum value before truncating.
    let rlim = C::rlimit {
        rlim_cur: soft.min(INFINITY) as _,
        rlim_max: hard.min(INFINITY) as _,
    };

    #[cfg(rlimit__asm_syscall)]
    unsafe {
        setrlimit_syscall(resource, &rlim)?;
    }

    #[cfg(not(rlimit__asm_syscall))]
    {
        #[allow(clippy::cast_lossless)]
        let ret = unsafe { C::setrlimit(resource.as_raw() as _, &rlim) };
        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

/// Get resource limits.
/// # Errors
/// \[Linux\] See <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
#[allow(clippy::unnecessary_min_or_max)]
#[inline]
pub fn getrlimit(resource: Resource) -> io::Result<(u64, u64)> {
    check_supported(resource)?;
    let mut rlim = unsafe { mem::zeroed() };

    #[cfg(rlimit__asm_syscall)]
    unsafe {
        getrlimit_syscall(resource, &mut rlim)?;
    }

    #[cfg(not(rlimit__asm_syscall))]
    {
        #[allow(clippy::cast_lossless)]
        let ret = unsafe { C::getrlimit(resource.as_raw() as _, &mut rlim) };

        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    #[allow(clippy::unnecessary_cast)]
    // SAFETY: On platforms where rlim_t is u64, this cast is lossless (no-op).
    // On platforms where rlim_t is smaller (e.g., u32), this is a widening cast
    // which is always safe. The min(INFINITY) clamps to our portable maximum.
    let soft = (rlim.rlim_cur as u64).min(INFINITY);
    let hard = (rlim.rlim_max as u64).min(INFINITY);
    Ok((soft, hard))
}

/// The type of a process ID
#[allow(non_camel_case_types)]
#[cfg_attr(docsrs, doc(cfg(any(target_os = "linux", target_os = "android"))))]
#[cfg(any(doc, target_os = "linux", target_os = "android"))]
pub type pid_t = i32;

/// Set and get the resource limits of an arbitrary process.
/// # Errors
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

    // SAFETY: Values are clamped to INFINITY before casting to rlim_t.
    // See setrlimit() for detailed explanation of truncation safety.
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

    #[cfg(all(rlimit__asm_syscall, rlimit__has_prlimit64))]
    unsafe {
        prlimit_syscall(pid, resource, new_rlimit_ptr, old_rlimit_ptr)?;
    }

    #[cfg(not(all(rlimit__asm_syscall, rlimit__has_prlimit64)))]
    {
        #[allow(clippy::cast_lossless)]
        let ret =
            unsafe { C::prlimit(pid, resource.as_raw() as _, new_rlimit_ptr, old_rlimit_ptr) };

        if ret != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    #[allow(clippy::unnecessary_cast)]
    if let Some((soft, hard)) = old_limit {
        // SAFETY: See getrlimit() for detailed explanation of cast safety.
        *soft = (old_rlim.rlim_cur as u64).min(INFINITY);
        *hard = (old_rlim.rlim_max as u64).min(INFINITY);
    }

    Ok(())
}
