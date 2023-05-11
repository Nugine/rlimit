#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::unnecessary_cast)]

#[cfg(any(target_os = "aix", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", all(target_os = "android", target_pointer_width = "32"), all(target_os = "android", target_pointer_width = "64")))]
pub use libc::rlimit64 as rlimit;

#[cfg(all(not(any(target_os = "aix", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", all(target_os = "android", target_pointer_width = "32"), all(target_os = "android", target_pointer_width = "64"))), any(unix, target_os = "fuchsia", target_os = "vxworks")))]
pub use libc::rlimit;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc")))]
pub use libc::getrlimit64 as getrlimit;

#[cfg(all(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc"))), any(target_os = "aix", target_os = "android", target_os = "dragonfly", target_os = "emscripten", target_os = "freebsd", target_os = "haiku", target_os = "hermit", target_os = "illumos", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "nto", target_os = "openbsd", target_os = "redox", target_os = "solaris", target_os = "tvos", target_os = "watchos", target_env = "newlib", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc"))))]
pub use libc::getrlimit;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc")))]
pub use libc::setrlimit64 as setrlimit;

#[cfg(all(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc"))), any(target_os = "aix", target_os = "android", target_os = "dragonfly", target_os = "emscripten", target_os = "freebsd", target_os = "haiku", target_os = "hermit", target_os = "illumos", target_os = "ios", target_os = "macos", target_os = "netbsd", target_os = "nto", target_os = "openbsd", target_os = "redox", target_os = "solaris", target_os = "tvos", target_os = "watchos", target_env = "newlib", all(target_os = "linux", target_env = "gnu"), all(target_os = "linux", target_env = "musl"), all(target_os = "linux", target_env = "uclibc"))))]
pub use libc::setrlimit;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", all(target_arch = "powerpc64", target_os = "aix"), target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIM_INFINITY: u64 = libc::RLIM_INFINITY as u64;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", all(target_arch = "powerpc64", target_os = "aix"), target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIM_INFINITY: u64 = u64::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "netbsd", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_AS: u8 = libc::RLIMIT_AS as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "netbsd", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_AS: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_CORE: u8 = libc::RLIMIT_CORE as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_CORE: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_CPU: u8 = libc::RLIMIT_CPU as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_CPU: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_DATA: u8 = libc::RLIMIT_DATA as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_DATA: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_FSIZE: u8 = libc::RLIMIT_FSIZE as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_FSIZE: u8 = u8::MAX;

#[cfg(target_os = "freebsd")]
pub const RLIMIT_KQUEUES: u8 = libc::RLIMIT_KQUEUES as u8;

#[cfg(not(target_os = "freebsd"))]
pub const RLIMIT_KQUEUES: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_LOCKS: u8 = libc::RLIMIT_LOCKS as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_LOCKS: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_MEMLOCK: u8 = libc::RLIMIT_MEMLOCK as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_MEMLOCK: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_MSGQUEUE: u8 = libc::RLIMIT_MSGQUEUE as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_MSGQUEUE: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_NICE: u8 = libc::RLIMIT_NICE as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_NICE: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_NOFILE: u8 = libc::RLIMIT_NOFILE as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_NOFILE: u8 = u8::MAX;

#[cfg(target_os = "haiku")]
pub const RLIMIT_NOVMON: u8 = libc::RLIMIT_NOVMON as u8;

#[cfg(not(target_os = "haiku"))]
pub const RLIMIT_NOVMON: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_NPROC: u8 = libc::RLIMIT_NPROC as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_NPROC: u8 = u8::MAX;

#[cfg(target_os = "freebsd")]
pub const RLIMIT_NPTS: u8 = libc::RLIMIT_NPTS as u8;

#[cfg(not(target_os = "freebsd"))]
pub const RLIMIT_NPTS: u8 = u8::MAX;

#[cfg(target_os = "netbsd")]
pub const RLIMIT_NTHR: u8 = libc::RLIMIT_NTHR as u8;

#[cfg(not(target_os = "netbsd"))]
pub const RLIMIT_NTHR: u8 = u8::MAX;

#[cfg(target_os = "dragonfly")]
pub const RLIMIT_POSIXLOCKS: u8 = libc::RLIMIT_POSIXLOCKS as u8;

#[cfg(not(target_os = "dragonfly"))]
pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_RSS: u8 = libc::RLIMIT_RSS as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_RSS: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_RTPRIO: u8 = libc::RLIMIT_RTPRIO as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_RTPRIO: u8 = u8::MAX;

#[cfg(any(target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_RTTIME: u8 = libc::RLIMIT_RTTIME as u8;

#[cfg(not(any(target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_RTTIME: u8 = u8::MAX;

#[cfg(any(target_os = "netbsd", target_os = "dragonfly", target_os = "freebsd"))]
pub const RLIMIT_SBSIZE: u8 = libc::RLIMIT_SBSIZE as u8;

#[cfg(not(any(target_os = "netbsd", target_os = "dragonfly", target_os = "freebsd")))]
pub const RLIMIT_SBSIZE: u8 = u8::MAX;

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_SIGPENDING: u8 = libc::RLIMIT_SIGPENDING as u8;

#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_SIGPENDING: u8 = u8::MAX;

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
pub const RLIMIT_STACK: u8 = libc::RLIMIT_STACK as u8;

#[cfg(not(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux"))))]
pub const RLIMIT_STACK: u8 = u8::MAX;

#[cfg(target_os = "freebsd")]
pub const RLIMIT_SWAP: u8 = libc::RLIMIT_SWAP as u8;

#[cfg(not(target_os = "freebsd"))]
pub const RLIMIT_SWAP: u8 = u8::MAX;

#[cfg(target_os = "freebsd")]
pub const RLIMIT_UMTXP: u8 = libc::RLIMIT_UMTXP as u8;

#[cfg(not(target_os = "freebsd"))]
pub const RLIMIT_UMTXP: u8 = u8::MAX;

#[cfg(any(target_os = "illumos", target_os = "nto", target_os = "solaris", target_os = "dragonfly", target_os = "freebsd"))]
pub const RLIMIT_VMEM: u8 = libc::RLIMIT_VMEM as u8;

#[cfg(not(any(target_os = "illumos", target_os = "nto", target_os = "solaris", target_os = "dragonfly", target_os = "freebsd")))]
pub const RLIMIT_VMEM: u8 = u8::MAX;

#[cfg(test)]
mod tests {
#[allow(clippy::too_many_lines)]
#[test]
fn resource_range() {
#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "netbsd", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_AS));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_CORE));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_CPU));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_DATA));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_FSIZE));

#[cfg(target_os = "freebsd")]
assert!((0..128).contains(&libc::RLIMIT_KQUEUES));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_LOCKS));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_MEMLOCK));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_MSGQUEUE));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_NICE));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_NOFILE));

#[cfg(target_os = "haiku")]
assert!((0..128).contains(&libc::RLIMIT_NOVMON));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_NPROC));

#[cfg(target_os = "freebsd")]
assert!((0..128).contains(&libc::RLIMIT_NPTS));

#[cfg(target_os = "netbsd")]
assert!((0..128).contains(&libc::RLIMIT_NTHR));

#[cfg(target_os = "dragonfly")]
assert!((0..128).contains(&libc::RLIMIT_POSIXLOCKS));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", target_os = "nto", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_RSS));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_RTPRIO));

#[cfg(any(target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_RTTIME));

#[cfg(any(target_os = "netbsd", target_os = "dragonfly", target_os = "freebsd"))]
assert!((0..128).contains(&libc::RLIMIT_SBSIZE));

#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "linux", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_SIGPENDING));

#[cfg(any(target_os = "aix", target_os = "android", target_os = "emscripten", target_os = "fuchsia", target_os = "haiku", target_os = "illumos", target_os = "linux", target_os = "nto", target_os = "solaris", target_os = "ios", target_os = "macos", target_os = "tvos", target_os = "watchos", target_os = "dragonfly", target_os = "freebsd", target_os = "netbsd", target_os = "openbsd", all(target_arch = "mips", target_os = "linux"), all(target_arch = "powerpc", target_os = "linux"), all(target_arch = "sparc", target_os = "linux")))]
assert!((0..128).contains(&libc::RLIMIT_STACK));

#[cfg(target_os = "freebsd")]
assert!((0..128).contains(&libc::RLIMIT_SWAP));

#[cfg(target_os = "freebsd")]
assert!((0..128).contains(&libc::RLIMIT_UMTXP));

#[cfg(any(target_os = "illumos", target_os = "nto", target_os = "solaris", target_os = "dragonfly", target_os = "freebsd"))]
assert!((0..128).contains(&libc::RLIMIT_VMEM));

}
}
