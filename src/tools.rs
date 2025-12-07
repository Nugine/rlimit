use std::io;

/// Returns the value of `kern.maxfilesperproc` by sysctl.
/// # Errors
/// Returns an error if any syscall failed.
#[cfg(rlimit__get_kern_max_files_per_proc)]
fn get_kern_max_files_per_proc() -> io::Result<u64> {
    use std::mem;
    use std::ptr;

    let mut mib = [libc::CTL_KERN, libc::KERN_MAXFILESPERPROC];
    let mut max_files_per_proc: libc::c_int = 0;
    let mut oldlen = mem::size_of::<libc::c_int>();
    let ret = unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            2,
            &mut max_files_per_proc as *mut libc::c_int as *mut libc::c_void,
            &mut oldlen,
            ptr::null_mut(),
            0,
        )
    };

    if ret < 0 {
        return Err(io::Error::last_os_error());
    }

    debug_assert!(max_files_per_proc >= 0);
    Ok(max_files_per_proc as u64)
}

/// Try to increase NOFILE limit and return the current soft limit.
///
/// This is a convenience function that safely increases the `NOFILE` (maximum number of open
/// file descriptors) limit to the requested value, or as high as possible if the requested
/// value exceeds system limits.
///
/// # Parameters
///
/// - `lim`: The desired limit value (can be [`u64::MAX`] to request the maximum possible)
///
/// # Returns
///
/// Returns the actual limit that was set, which may be lower than requested if:
/// - The requested limit exceeds the hard limit
/// - The requested limit exceeds system maximums (e.g., `kern.maxfilesperproc` on macOS)
/// - The current soft limit is already at or above the requested limit
///
/// # Platform-specific behavior
///
/// - **Unix/Linux**: Increases `RLIMIT_NOFILE` up to the hard limit
/// - **macOS**: Respects `kern.maxfilesperproc` sysctl value to avoid errors
/// - **Windows**: Does nothing and returns the requested limit
///
/// # Examples
///
/// ```no_run
/// use rlimit::increase_nofile_limit;
///
/// // Try to set NOFILE to 10240
/// match increase_nofile_limit(10240) {
///     Ok(lim) => println!("NOFILE limit is now {}", lim),
///     Err(e) => eprintln!("Failed: {}", e),
/// }
///
/// // Request the maximum possible limit
/// let max_limit = increase_nofile_limit(u64::MAX).unwrap();
/// println!("Maximum NOFILE limit: {}", max_limit);
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - Getting the current limits fails
/// - Setting the new limit fails (e.g., permission denied)
/// - On macOS, if querying `kern.maxfilesperproc` fails
pub fn increase_nofile_limit(lim: u64) -> io::Result<u64> {
    #[cfg(unix)]
    {
        use crate::Resource;

        if !Resource::NOFILE.is_supported() {
            return Ok(lim);
        }

        let (soft, hard) = Resource::NOFILE.get()?;

        if soft >= hard {
            return Ok(hard);
        }

        if soft >= lim {
            return Ok(soft);
        }

        let mut lim = lim;

        lim = lim.min(hard);

        #[cfg(rlimit__get_kern_max_files_per_proc)]
        {
            lim = lim.min(get_kern_max_files_per_proc()?)
        }

        Resource::NOFILE.set(lim, hard)?;

        Ok(lim)
    }
    #[cfg(windows)]
    {
        Ok(lim)
    }
}
