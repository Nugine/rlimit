#![allow(
    clippy::assertions_on_constants,
    clippy::absurd_extreme_comparisons,
    clippy::cast_possible_truncation,
    unused_comparisons
)]

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_AS: u8 = libc::RLIMIT_AS as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_AS && libc::RLIMIT_AS < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_AS: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_CORE: u8 = libc::RLIMIT_CORE as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_CORE && libc::RLIMIT_CORE < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_CORE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_CPU: u8 = libc::RLIMIT_CPU as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_CPU && libc::RLIMIT_CPU < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_CPU: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_DATA: u8 = libc::RLIMIT_DATA as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_DATA && libc::RLIMIT_DATA < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_DATA: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_FSIZE: u8 = libc::RLIMIT_FSIZE as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_FSIZE && libc::RLIMIT_FSIZE < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_FSIZE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "freebsd",))]
pub const RLIMIT_KQUEUES: u8 = libc::RLIMIT_KQUEUES as u8;

#[cfg(any(target_os = "freebsd",))]
const _: () = assert!(0 <= libc::RLIMIT_KQUEUES && libc::RLIMIT_KQUEUES < 128);

#[cfg(not(any(target_os = "freebsd",)))]
pub const RLIMIT_KQUEUES: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_LOCKS: u8 = libc::RLIMIT_LOCKS as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_LOCKS && libc::RLIMIT_LOCKS < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_LOCKS: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_MEMLOCK: u8 = libc::RLIMIT_MEMLOCK as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_MEMLOCK && libc::RLIMIT_MEMLOCK < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_MEMLOCK: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_MSGQUEUE: u8 = libc::RLIMIT_MSGQUEUE as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_MSGQUEUE && libc::RLIMIT_MSGQUEUE < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_MSGQUEUE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_NICE: u8 = libc::RLIMIT_NICE as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_NICE && libc::RLIMIT_NICE < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_NICE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_NOFILE: u8 = libc::RLIMIT_NOFILE as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_NOFILE && libc::RLIMIT_NOFILE < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_NOFILE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "haiku",))]
pub const RLIMIT_NOVMON: u8 = libc::RLIMIT_NOVMON as u8;

#[cfg(any(target_os = "haiku",))]
const _: () = assert!(0 <= libc::RLIMIT_NOVMON && libc::RLIMIT_NOVMON < 128);

#[cfg(not(any(target_os = "haiku",)))]
pub const RLIMIT_NOVMON: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_NPROC: u8 = libc::RLIMIT_NPROC as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_NPROC && libc::RLIMIT_NPROC < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_NPROC: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "freebsd",))]
pub const RLIMIT_NPTS: u8 = libc::RLIMIT_NPTS as u8;

#[cfg(any(target_os = "freebsd",))]
const _: () = assert!(0 <= libc::RLIMIT_NPTS && libc::RLIMIT_NPTS < 128);

#[cfg(not(any(target_os = "freebsd",)))]
pub const RLIMIT_NPTS: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "netbsd",))]
pub const RLIMIT_NTHR: u8 = libc::RLIMIT_NTHR as u8;

#[cfg(any(target_os = "netbsd",))]
const _: () = assert!(0 <= libc::RLIMIT_NTHR && libc::RLIMIT_NTHR < 128);

#[cfg(not(any(target_os = "netbsd",)))]
pub const RLIMIT_NTHR: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "dragonfly",))]
pub const RLIMIT_POSIXLOCKS: u8 = libc::RLIMIT_POSIXLOCKS as u8;

#[cfg(any(target_os = "dragonfly",))]
const _: () = assert!(0 <= libc::RLIMIT_POSIXLOCKS && libc::RLIMIT_POSIXLOCKS < 128);

#[cfg(not(any(target_os = "dragonfly",)))]
pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_RSS: u8 = libc::RLIMIT_RSS as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_RSS && libc::RLIMIT_RSS < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_RSS: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_RTPRIO: u8 = libc::RLIMIT_RTPRIO as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_RTPRIO && libc::RLIMIT_RTPRIO < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_RTPRIO: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_RTTIME: u8 = libc::RLIMIT_RTTIME as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_RTTIME && libc::RLIMIT_RTTIME < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_RTTIME: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
))]
pub const RLIMIT_SBSIZE: u8 = libc::RLIMIT_SBSIZE as u8;

#[cfg(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
))]
const _: () = assert!(0 <= libc::RLIMIT_SBSIZE && libc::RLIMIT_SBSIZE < 128);

#[cfg(not(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    target_os = "netbsd",
)))]
pub const RLIMIT_SBSIZE: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
pub const RLIMIT_SIGPENDING: u8 = libc::RLIMIT_SIGPENDING as u8;

#[cfg(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
))]
const _: () = assert!(0 <= libc::RLIMIT_SIGPENDING && libc::RLIMIT_SIGPENDING < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
)))]
pub const RLIMIT_SIGPENDING: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_STACK: u8 = libc::RLIMIT_STACK as u8;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_STACK && libc::RLIMIT_STACK < 128);

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_STACK: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "freebsd",))]
pub const RLIMIT_SWAP: u8 = libc::RLIMIT_SWAP as u8;

#[cfg(any(target_os = "freebsd",))]
const _: () = assert!(0 <= libc::RLIMIT_SWAP && libc::RLIMIT_SWAP < 128);

#[cfg(not(any(target_os = "freebsd",)))]
pub const RLIMIT_SWAP: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(target_os = "freebsd",))]
pub const RLIMIT_UMTXP: u8 = libc::RLIMIT_UMTXP as u8;

#[cfg(any(target_os = "freebsd",))]
const _: () = assert!(0 <= libc::RLIMIT_UMTXP && libc::RLIMIT_UMTXP < 128);

#[cfg(not(any(target_os = "freebsd",)))]
pub const RLIMIT_UMTXP: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIMIT_VMEM: u8 = libc::RLIMIT_VMEM as u8;

#[cfg(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "solaris", target_os = "illumos"),
))]
const _: () = assert!(0 <= libc::RLIMIT_VMEM && libc::RLIMIT_VMEM < 128);

#[cfg(not(any(
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIMIT_VMEM: u8 = u8::MAX;

// -----------------------------------------------------------------------------

#[cfg(any(
    target_os = "fuchsia",
    all(target_os = "android", target_pointer_width = "32"),
    all(target_os = "android", target_pointer_width = "64"),
    target_os = "emscripten",
    target_os = "linux",
))]
pub use libc::rlimit64 as rlimit;

#[cfg(all(
    not(any(
        target_os = "fuchsia",
        all(target_os = "android", target_pointer_width = "32"),
        all(target_os = "android", target_pointer_width = "64"),
        target_os = "emscripten",
        target_os = "linux",
    )),
    any(
        target_os = "fuchsia",
        all(target_os = "android", target_pointer_width = "32"),
        all(target_os = "android", target_pointer_width = "64"),
        target_os = "emscripten",
        target_os = "linux",
        target_family = "unix",
        target_os = "vxworks",
    )
))]
pub use libc::rlimit;

#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    all(target_os = "linux", target_env = "gnu"),
    all(target_os = "linux", target_env = "musl"),
    all(target_os = "linux", target_env = "uclibc"),
))]
pub use libc::getrlimit64 as getrlimit;

#[cfg(all(
    not(any(
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    )),
    any(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "watchos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        target_os = "haiku",
        target_os = "hermit",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_env = "newlib",
        target_os = "redox",
        any(target_os = "solaris", target_os = "illumos"),
    )
))]
pub use libc::getrlimit;

#[cfg(any(
    target_os = "android",
    target_os = "emscripten",
    all(target_os = "linux", target_env = "gnu"),
    all(target_os = "linux", target_env = "musl"),
    all(target_os = "linux", target_env = "uclibc"),
))]
pub use libc::setrlimit64 as setrlimit;

#[cfg(all(
    not(any(
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
    )),
    any(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "watchos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ),
        target_os = "haiku",
        target_os = "hermit",
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc"),
        target_env = "newlib",
        target_os = "redox",
        any(target_os = "solaris", target_os = "illumos"),
    )
))]
pub use libc::setrlimit;

#[cfg(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
))]
pub const RLIM_INFINITY: u64 = libc::RLIM_INFINITY as u64;

#[cfg(not(any(
    target_os = "fuchsia",
    any(target_os = "macos", target_os = "ios"),
    any(target_os = "freebsd", target_os = "dragonfly"),
    any(target_os = "openbsd", target_os = "netbsd"),
    target_os = "haiku",
    target_os = "android",
    target_os = "emscripten",
    target_os = "linux",
    all(target_os = "linux", any(target_arch = "mips", target_arch = "mips64")),
    all(
        target_os = "linux",
        any(target_arch = "powerpc", target_arch = "powerpc64")
    ),
    all(
        target_os = "linux",
        any(target_arch = "sparc", target_arch = "sparc64")
    ),
    any(target_os = "solaris", target_os = "illumos"),
)))]
pub const RLIM_INFINITY: u64 = u64::MAX;
