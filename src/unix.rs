use crate::bindings as C;
use crate::resource::Resource;

use std::{io, mem};

#[cfg(rlimit__asm_syscall)]
use core::arch::asm;

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

#[cfg(rlimit__asm_syscall)]
// x86_64 Linux syscall numbers (matching libc::SYS_* definitions).
const SYS_GETRLIMIT: isize = 97;

#[cfg(rlimit__asm_syscall)]
const SYS_SETRLIMIT: isize = 160;

#[cfg(all(rlimit__asm_syscall, rlimit__has_prlimit64))]
const SYS_PRLIMIT64: isize = 302;

#[cfg(rlimit__asm_syscall)]
#[inline]
/// Convert a syscall return value into `io::Result`.
///
/// # Safety
/// The caller must ensure `ret` originates from a Linux syscall that returns
/// negative errno values on failure.
unsafe fn syscall_result(ret: isize) -> io::Result<()> {
    if ret < 0 {
        Err(io::Error::from_raw_os_error(-ret as i32))
    } else {
        Ok(())
    }
}

#[cfg(rlimit__asm_syscall)]
#[inline]
/// Invoke a two-argument syscall.
///
/// # Safety
/// The caller must ensure the arguments follow the syscall's contract.
unsafe fn syscall2(nr: isize, arg1: usize, arg2: usize) -> io::Result<()> {
    let ret: isize;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    syscall_result(ret)
}

#[cfg(rlimit__asm_syscall)]
#[inline]
/// Invoke a four-argument syscall.
///
/// # Safety
/// The caller must ensure the arguments follow the syscall's contract.
unsafe fn syscall4(
    nr: isize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
) -> io::Result<()> {
    let ret: isize;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack),
    );
    syscall_result(ret)
}

#[cfg(rlimit__asm_syscall)]
#[inline]
/// Perform `setrlimit` via inline assembly.
///
/// # Safety
/// The caller must ensure `resource` is supported and `rlim` points to a valid
/// `rlimit` structure.
unsafe fn asm_setrlimit(resource: Resource, rlim: &C::rlimit) -> io::Result<()> {
    syscall2(
        SYS_SETRLIMIT,
        usize::from(resource.as_raw()),
        rlim as *const C::rlimit as usize,
    )
}

#[cfg(rlimit__asm_syscall)]
#[inline]
/// Perform `getrlimit` via inline assembly.
///
/// # Safety
/// The caller must ensure `resource` is supported and `rlim` is valid for
/// writes.
unsafe fn asm_getrlimit(resource: Resource, rlim: &mut C::rlimit) -> io::Result<()> {
    syscall2(
        SYS_GETRLIMIT,
        usize::from(resource.as_raw()),
        rlim as *mut C::rlimit as usize,
    )
}

#[cfg(all(rlimit__asm_syscall, rlimit__has_prlimit64))]
#[inline]
/// Perform `prlimit64` via inline assembly.
///
/// # Safety
/// The caller must ensure pointer arguments reference valid `rlimit` storage or
/// are null when unused.
unsafe fn asm_prlimit(
    pid: pid_t,
    resource: Resource,
    new_rlimit_ptr: *const C::rlimit,
    old_rlimit_ptr: *mut C::rlimit,
) -> io::Result<()> {
    syscall4(
        SYS_PRLIMIT64,
        pid as u32 as usize,
        usize::from(resource.as_raw()),
        new_rlimit_ptr as usize,
        old_rlimit_ptr as usize,
    )
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
        asm_setrlimit(resource, &rlim)?;
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
        asm_getrlimit(resource, &mut rlim)?;
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
        asm_prlimit(pid, resource, new_rlimit_ptr, old_rlimit_ptr)?;
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
