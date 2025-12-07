use std::io;
use std::os::raw::c_int;

extern "C" {
    fn _setmaxstdio(new_max: c_int) -> c_int;
    fn _getmaxstdio() -> c_int;
}

/// Sets a maximum for the number of simultaneously open files at the stream I/O level.
///
/// Windows-specific function that controls the maximum number of files that can be
/// opened simultaneously using the C runtime stream I/O functions (like `fopen`).
///
/// # Parameters
///
/// - `new_max`: The new maximum number of open files (typically 512-8192)
///
/// # Returns
///
/// Returns the new limit value if successful.
///
/// # Notes
///
/// - The default maximum is typically 512
/// - The maximum value allowed is typically 8192, but may vary by system
/// - This only affects C runtime stream I/O, not Windows API file handles
/// - Each open file consumes system resources
///
/// # Examples
///
/// ```no_run
/// # #[cfg(windows)]
/// # {
/// // Increase the limit to 2048
/// match rlimit::setmaxstdio(2048) {
///     Ok(new_max) => println!("New limit: {}", new_max),
///     Err(e) => eprintln!("Failed to set limit: {}", e),
/// }
/// # }
/// ```
///
/// See <https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/setmaxstdio?view=msvc-170>
///
/// # Errors
///
/// Returns an error if:
/// - The value is out of valid range
/// - System resources are insufficient
#[cfg_attr(docsrs, doc(cfg(windows)))]
pub fn setmaxstdio(new_max: u32) -> io::Result<u32> {
    // A negative `new_max` will cause EINVAL.
    // A negative `ret` should never appear.
    // It is safe even if the return value is wrong.
    #[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
    unsafe {
        let ret = _setmaxstdio(new_max as c_int);
        if ret < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(ret as u32)
    }
}

/// Returns the number of simultaneously open files permitted at the stream I/O level.
///
/// Windows-specific function that queries the current maximum number of files that can be
/// opened simultaneously using the C runtime stream I/O functions.
///
/// # Returns
///
/// The current maximum number of open files (typically 512 by default).
///
/// # Examples
///
/// ```no_run
/// # #[cfg(windows)]
/// # {
/// let current = rlimit::getmaxstdio();
/// println!("Current max stdio: {}", current);
///
/// // Check if we need to increase the limit
/// if current < 1024 {
///     rlimit::setmaxstdio(1024).unwrap();
/// }
/// # }
/// ```
///
/// See <https://docs.microsoft.com/en-us/cpp/c-runtime-library/reference/getmaxstdio?view=msvc-170>
#[cfg_attr(docsrs, doc(cfg(windows)))]
#[must_use]
pub fn getmaxstdio() -> u32 {
    // A negative `ret` should never appear.
    // It is safe even if the return value is wrong.
    #[allow(clippy::cast_sign_loss)]
    unsafe {
        let ret = _getmaxstdio();
        debug_assert!(ret >= 0);
        ret as u32
    }
}
