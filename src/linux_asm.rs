use super::pid_t;
use crate::bindings as C;
use crate::resource::Resource;

use core::arch::asm;
use std::io;

// x86_64 Linux syscall numbers (matching libc::SYS_* definitions).
const SYS_GETRLIMIT: isize = 97;
const SYS_SETRLIMIT: isize = 160;

#[cfg(rlimit__has_prlimit64)]
const SYS_PRLIMIT64: isize = 302;

/// Convert a syscall return value into `io::Result`.
///
/// # Safety
/// The caller must ensure `ret` originates from a Linux syscall that returns
/// negative errno values on failure.
#[allow(clippy::cast_possible_truncation)]
#[inline]
unsafe fn syscall_result(ret: isize) -> io::Result<()> {
    if ret < 0 {
        Err(io::Error::from_raw_os_error(-ret as i32))
    } else {
        Ok(())
    }
}

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

#[inline]
/// Perform `setrlimit` via inline assembly.
///
/// # Safety
/// The caller must ensure `resource` is supported and `rlim` points to a valid
/// `rlimit` structure.
pub(crate) unsafe fn setrlimit_syscall(resource: Resource, rlim: &C::rlimit) -> io::Result<()> {
    syscall2(
        SYS_SETRLIMIT,
        usize::from(resource.as_raw()),
        rlim as *const C::rlimit as usize,
    )
}

#[inline]
/// Perform `getrlimit` via inline assembly.
///
/// # Safety
/// The caller must ensure `resource` is supported and `rlim` is valid for
/// writes.
pub(crate) unsafe fn getrlimit_syscall(resource: Resource, rlim: &mut C::rlimit) -> io::Result<()> {
    syscall2(
        SYS_GETRLIMIT,
        usize::from(resource.as_raw()),
        rlim as *mut C::rlimit as usize,
    )
}

#[cfg(rlimit__has_prlimit64)]
/// Perform `prlimit64` via inline assembly.
///
/// # Safety
/// The caller must ensure pointer arguments reference valid `rlimit` storage or
/// are null when unused.
#[allow(clippy::cast_sign_loss)]
#[inline]
pub(crate) unsafe fn prlimit_syscall(
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
