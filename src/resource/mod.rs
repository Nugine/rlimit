mod generated;

use crate::{getrlimit, setrlimit};

use std::fmt;
use std::io;

/// A kind of resource.
///
/// Represents a system resource type that can have limits set on it. Each resource
/// controls a different aspect of process behavior (memory usage, open files, CPU time, etc.).
///
/// # Platform Support
///
/// All resource constants are available on all Unix platforms for convenience, but not all
/// resources are supported on every platform. Passing an unsupported resource to
/// [`setrlimit`], [`getrlimit`], or `prlimit` will result in an error.
///
/// Use [`Resource::is_supported`] to check if a resource is available on the current platform.
///
/// # Common Resources
///
/// - [`Resource::NOFILE`] - Maximum number of open file descriptors
/// - [`Resource::NPROC`] - Maximum number of processes/threads
/// - [`Resource::FSIZE`] - Maximum file size
/// - [`Resource::DATA`] - Maximum data segment size
/// - [`Resource::STACK`] - Maximum stack size
/// - [`Resource::AS`] - Maximum address space (virtual memory)
/// - [`Resource::CORE`] - Maximum core file size
/// - [`Resource::CPU`] - Maximum CPU time in seconds
///
/// See the individual constant documentation for detailed descriptions.
///
/// # Examples
///
/// ```
/// # #[cfg(unix)]
/// # {
/// use rlimit::Resource;
///
/// // Check if a resource is supported
/// if Resource::NOFILE.is_supported() {
///     match Resource::NOFILE.get() {
///         Ok((soft, hard)) => println!("NOFILE: soft={}, hard={}", soft, hard),
///         Err(e) => eprintln!("Failed to get NOFILE limits: {}", e),
///     }
/// }
///
/// // Get the resource name
/// assert_eq!(Resource::CPU.as_name(), "RLIMIT_CPU");
/// # }
/// ```
///
/// **Note**: The documentation for resource constants is based on common systems.
/// Some details may vary across different platforms.
///
/// ## References
///
/// Linux: <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
///
/// FreeBSD: <https://www.freebsd.org/cgi/man.cgi?query=getrlimit>
///
/// NetBSD: <https://man.netbsd.org/getrlimit.2>
///
/// AIX: <https://www.ibm.com/docs/en/aix/7.3?topic=g-getrlimit-getrlimit64-setrlimit-setrlimit64-vlimit-subroutine>
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resource {
    tag: u8,
    value: u8,
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match Self::find_ident_by_tag(self.tag) {
            Some(ident) => write!(f, "Resource::{ident}"),
            None => unreachable!(),
        }
    }
}

/// An error returned when `Resource::from_str` fails
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResourceError(());

impl fmt::Display for ParseResourceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse Resource")
    }
}

impl std::error::Error for ParseResourceError {}

impl Resource {
    /// Set resource limits.
    ///
    /// Convenience method equivalent to [`setrlimit(self, soft, hard)`][setrlimit].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(unix)]
    /// # {
    /// use rlimit::Resource;
    ///
    /// Resource::NOFILE.set(1024, 2048).unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    /// See [`setrlimit`]
    #[inline]
    pub fn set(self, soft: u64, hard: u64) -> io::Result<()> {
        setrlimit(self, soft, hard)
    }

    /// Get resource limits.
    ///
    /// Convenience method equivalent to [`getrlimit(self)`][getrlimit].
    ///
    /// # Returns
    ///
    /// Returns a tuple `(soft, hard)` containing the current soft and hard limits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(unix)]
    /// # {
    /// use rlimit::Resource;
    ///
    /// let (soft, hard) = Resource::NOFILE.get().unwrap();
    /// println!("NOFILE: soft={}, hard={}", soft, hard);
    /// # }
    /// ```
    ///
    /// # Errors
    /// See [`getrlimit`]
    #[inline]
    pub fn get(self) -> io::Result<(u64, u64)> {
        getrlimit(self)
    }

    /// Get soft resource limit (`rlim_cur`)
    ///
    /// Returns only the soft limit, which is the currently enforced limit.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(unix)]
    /// # {
    /// use rlimit::Resource;
    ///
    /// let soft = Resource::NOFILE.get_soft().unwrap();
    /// println!("Current NOFILE soft limit: {}", soft);
    /// # }
    /// ```
    ///
    /// # Errors
    /// See [`getrlimit`]
    pub fn get_soft(self) -> io::Result<u64> {
        self.get().map(|(soft, _)| soft)
    }

    /// Get hard resource limit (`rlim_max`)
    ///
    /// Returns only the hard limit, which is the maximum value that the soft limit
    /// can be raised to by an unprivileged process.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[cfg(unix)]
    /// # {
    /// use rlimit::Resource;
    ///
    /// let hard = Resource::NOFILE.get_hard().unwrap();
    /// println!("Maximum NOFILE limit: {}", hard);
    /// # }
    /// ```
    ///
    /// # Errors
    /// See [`getrlimit`]
    pub fn get_hard(self) -> io::Result<u64> {
        self.get().map(|(_, hard)| hard)
    }

    /// Returns the name of the resource.
    ///
    /// # Example
    /// ```
    /// # #[cfg(unix)]
    /// # {
    /// # use rlimit::Resource;
    /// assert_eq!(Resource::NOFILE.as_name(), "RLIMIT_NOFILE");
    /// # }
    /// ```
    #[must_use]
    pub fn as_name(self) -> &'static str {
        match Self::find_name_by_tag(self.tag) {
            Some(name) => name,
            None => unreachable!(),
        }
    }

    /// Returns true if the current platform supports this resource.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        self.value != u8::MAX
    }

    /// `u8::MAX` indicates unsupported resource.
    #[inline]
    #[must_use]
    pub(crate) const fn as_raw(self) -> u8 {
        self.value
    }
}
