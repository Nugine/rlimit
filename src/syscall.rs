//! Direct syscalls using inline assembly for Linux.
//!
//! This module provides raw syscall interfaces without going through libc.

#![cfg(all(feature = "asm", target_os = "linux"))]

use std::arch::asm;

/// `RLIM_INFINITY` value for Linux
///
/// For most modern Linux systems (`x86_64`, `aarch64`, `riscv64`), this is `u64::MAX`.
///
/// **Note**: Some architectures (MIPS, SPARC, `PowerPC` with certain configurations)
/// may use different values for `RLIM_INFINITY`. This implementation uses `u64::MAX`
/// which is correct for the commonly used architectures. If you're targeting an
/// architecture with a different `RLIM_INFINITY`, you should use the default libc-based
/// implementation instead of the `asm` feature.
pub const RLIM_INFINITY: u64 = u64::MAX;

/// Syscall numbers for `x86_64` Linux
/// Note: On `x86_64`, getrlimit/setrlimit use the 64-bit versions automatically
#[cfg(target_arch = "x86_64")]
mod syscall_nr {
    pub const SYS_GETRLIMIT: usize = 97;  // getrlimit (64-bit on x86_64)
    pub const SYS_SETRLIMIT: usize = 160; // setrlimit (64-bit on x86_64)
    pub const SYS_PRLIMIT64: usize = 302;
}

/// Syscall numbers for aarch64 Linux
#[cfg(target_arch = "aarch64")]
mod syscall_nr {
    pub const SYS_SETRLIMIT: usize = 164;
    pub const SYS_GETRLIMIT: usize = 163;
    pub const SYS_PRLIMIT64: usize = 261;
}

/// Syscall numbers for x86 Linux (32-bit)
#[cfg(target_arch = "x86")]
mod syscall_nr {
    pub const SYS_GETRLIMIT: usize = 76;
    pub const SYS_SETRLIMIT: usize = 75;
    pub const SYS_PRLIMIT64: usize = 340;
}

/// Syscall numbers for arm Linux (32-bit)
#[cfg(target_arch = "arm")]
mod syscall_nr {
    pub const SYS_GETRLIMIT: usize = 76;
    pub const SYS_SETRLIMIT: usize = 75;
    pub const SYS_PRLIMIT64: usize = 369;
}

/// Syscall numbers for riscv64 Linux
#[cfg(target_arch = "riscv64")]
mod syscall_nr {
    pub const SYS_SETRLIMIT: usize = 164;
    pub const SYS_GETRLIMIT: usize = 163;
    pub const SYS_PRLIMIT64: usize = 261;
}

use syscall_nr::{SYS_GETRLIMIT, SYS_PRLIMIT64, SYS_SETRLIMIT};

/// Perform a syscall with 2 arguments
#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn syscall2(nr: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "syscall",
        inlateout("rax") nr => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 4 arguments
#[cfg(target_arch = "x86_64")]
#[inline]
unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
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
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 2 arguments
#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn syscall2(nr: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "svc 0",
        inlateout("x8") nr => _,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 4 arguments
#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    asm!(
        "svc 0",
        inlateout("x8") nr => _,
        inlateout("x0") arg1 => ret,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 2 arguments
#[cfg(target_arch = "x86")]
#[inline]
unsafe fn syscall2(nr: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "int 0x80",
        inlateout("eax") nr => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 4 arguments
#[cfg(target_arch = "x86")]
#[inline]
unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    asm!(
        "int 0x80",
        inlateout("eax") nr => ret,
        in("ebx") arg1,
        in("ecx") arg2,
        in("edx") arg3,
        in("esi") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 2 arguments
#[cfg(target_arch = "arm")]
#[inline]
unsafe fn syscall2(nr: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "svc 0",
        inlateout("r7") nr => _,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 4 arguments
#[cfg(target_arch = "arm")]
#[inline]
unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    asm!(
        "svc 0",
        inlateout("r7") nr => _,
        inlateout("r0") arg1 => ret,
        in("r1") arg2,
        in("r2") arg3,
        in("r3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 2 arguments
#[cfg(target_arch = "riscv64")]
#[inline]
unsafe fn syscall2(nr: usize, arg1: usize, arg2: usize) -> isize {
    let ret: isize;
    asm!(
        "ecall",
        inlateout("a7") nr => _,
        inlateout("a0") arg1 => ret,
        in("a1") arg2,
        options(nostack, preserves_flags)
    );
    ret
}

/// Perform a syscall with 4 arguments
#[cfg(target_arch = "riscv64")]
#[inline]
unsafe fn syscall4(nr: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> isize {
    let ret: isize;
    asm!(
        "ecall",
        inlateout("a7") nr => _,
        inlateout("a0") arg1 => ret,
        in("a1") arg2,
        in("a2") arg3,
        in("a3") arg4,
        options(nostack, preserves_flags)
    );
    ret
}

/// The rlimit structure for syscalls
#[repr(C)]
#[derive(Clone, Copy)]
pub struct rlimit {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

/// Get resource limit
///
/// # Safety
/// The caller must ensure `rlim` points to valid memory for a `rlimit` struct.
///
/// # Returns
/// Returns 0 on success, or a negative error code on failure (to match libc behavior).
/// The error code is truncated to i32 which is safe because Linux error codes fit in this range.
#[inline]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub unsafe fn getrlimit(resource: i32, rlim: *mut rlimit) -> i32 {
    let ret = syscall2(SYS_GETRLIMIT, resource as usize, rlim as usize);
    if ret < 0 {
        ret as i32
    } else {
        0
    }
}

/// Set resource limit
///
/// # Safety
/// The caller must ensure `rlim` points to valid memory for a `rlimit` struct.
///
/// # Returns
/// Returns 0 on success, or a negative error code on failure (to match libc behavior).
#[inline]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub unsafe fn setrlimit(resource: i32, rlim: *const rlimit) -> i32 {
    let ret = syscall2(SYS_SETRLIMIT, resource as usize, rlim as usize);
    if ret < 0 {
        ret as i32
    } else {
        0
    }
}

/// Get and set resource limits of an arbitrary process
///
/// # Safety
/// The caller must ensure that any non-null pointers point to valid memory for `rlimit` structs.
///
/// # Returns
/// Returns 0 on success, or a negative error code on failure (to match libc behavior).
#[inline]
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub unsafe fn prlimit(
    pid: i32,
    resource: i32,
    new_limit: *const rlimit,
    old_limit: *mut rlimit,
) -> i32 {
    let ret = syscall4(
        SYS_PRLIMIT64,
        pid as usize,
        resource as usize,
        new_limit as usize,
        old_limit as usize,
    );
    if ret < 0 {
        ret as i32
    } else {
        0
    }
}
