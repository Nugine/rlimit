// generated from rust-lang/libc 13c8ceb1ed9077295edf68747bb282a6bee5f31c
impl Rlim {
    /// A value of Rlim indicating no limit.
    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "macos", target_os = "ios"),
        any(target_os = "freebsd", target_os = "dragonfly"),
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "android",
        target_os = "emscripten",
        all(target_os = "linux", target_env = "gnu"),
        all(target_os = "linux", target_env = "musl"),
        all(target_os = "linux", target_env = "uclibc", target_arch = "arm"),
        all(target_os = "linux", target_env = "uclibc", target_arch = "mips"),
        all(target_os = "linux", target_env = "uclibc", target_arch = "mips64"),
        all(target_os = "linux", target_env = "uclibc", target_arch = "x86_64"),
        target_os = "solarish",
    ))]
    pub const INFINITY: Self = Self(libc::RLIM_INFINITY);

    /// A value of type Rlim indicating an unrepresentable saved hard limit.
    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "emscripten",
        target_os = "linux",
    ))]
    pub const SAVED_MAX: Self = Self(libc::RLIM_SAVED_MAX);

    /// A value of type Rlim indicating an unrepresentable saved soft limit.
    #[cfg(any(
        target_os = "fuchsia",
        any(target_os = "openbsd", target_os = "netbsd"),
        target_os = "emscripten",
        target_os = "linux",
    ))]
    pub const SAVED_CUR: Self = Self(libc::RLIM_SAVED_CUR);
}
