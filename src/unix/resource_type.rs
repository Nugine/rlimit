#![deny(unsafe_code)]

use super::rlim_type::Rlim;

use std::error::Error;
use std::fmt;
use std::io;
use std::str::FromStr;

#[cfg(all(target_os = "linux", target_env = "gnu"))]
use libc::__rlimit_resource_t as resource_t;

#[cfg(not(all(target_os = "linux", target_env = "gnu")))]
use libc::c_int as resource_t;

/// Integer type used for resource values.
///
/// The actual type of [`RawResource`][RawResource] can be different on different platforms.
///
/// [RawResource]: type.RawResource.html
pub type RawResource = resource_t;

#[allow(clippy::doc_markdown)]
/// A kind of resource.
///
/// **Be careful**: The documentation of [`Resource`][Resource] constants are based on a few systems.
/// It may be inconsistent with other platforms.
///
/// # References
/// Linux: <https://man7.org/linux/man-pages/man2/getrlimit.2.html>
///
/// FreeBSD: <https://www.freebsd.org/cgi/man.cgi?query=getrlimit>
///
/// NetBSD: <https://man.netbsd.org/getrlimit.2>
///
/// [Resource]: struct.Resource.html
///
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resource {
    tag: u16,
    value: u16,
}

impl fmt::Debug for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = Self::VALUE_TABLE.iter().position(|v| v == self).unwrap();
        write!(f, "Resource::{}", Self::IDENT_TABLE[idx])
    }
}

impl FromStr for Resource {
    type Err = ParseResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = Self::NAME_TABLE.iter().position(|&name| s == name);
        match pos {
            Some(idx) => Ok(Self::VALUE_TABLE[idx]),
            None => Err(ParseResourceError { _priv: () }),
        }
    }
}

/// An error returned when parsing a `Resource` using [`from_str`] fails
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResourceError {
    /// private place holder
    _priv: (),
}

impl fmt::Display for ParseResourceError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse Resource")
    }
}

impl Error for ParseResourceError {}

macro_rules! declare_resource {
    {$($(#[$attr:meta])* $id:ident = $tag:expr => $c_enum:ident,)+} => {
        impl Resource{
            $(
                $(#[$attr])*
                pub const $id: Self = Self{ tag: $tag, value: libc::$c_enum as u16 };
            )+
        }

        #[allow(unused_doc_comments)]
        impl Resource{
            const NAME_TABLE: &'static [&'static str] = &[
                $(
                    $(#[$attr])*
                    {
                        stringify!($c_enum)
                    },
                )+
            ];

            const VALUE_TABLE: &'static [Self] = &[
                $(
                    $(#[$attr])*
                    {
                        Self::$id
                    },
                )+
            ];

            const IDENT_TABLE: &'static [&'static str] = &[
                $(
                    $(#[$attr])*
                    {
                        stringify!($id)
                    },
                )+
            ];
        }

        #[cfg(test)]
        mod tests{
            use super::*;

            #[allow(unused_comparisons)]
            #[allow(unused_doc_comments)]
            #[test]
            fn name_value(){
                $(
                    $(#[$attr])*
                    {
                        assert_eq!(Resource::$id.as_name(), stringify!($c_enum));
                        assert_eq!(Resource::from_str(stringify!($c_enum)).unwrap(), Resource::$id);
                        assert!((0..=128).contains(&libc::$c_enum));
                    }
                )+
            }

            #[allow(unused_doc_comments)]
            #[test]
            fn unique_tag(){
                use std::collections::HashSet;

                let tags = [
                    $(
                        $(#[$attr])*
                        { $tag },
                    )+
                ];

                let s: HashSet<u16> = tags.iter().copied().collect();
                assert_eq!(s.len(), Resource::NAME_TABLE.len());
            }

            #[allow(unused_doc_comments)]
            #[test]
            fn raw_eq(){
                $(
                    $(#[$attr])*
                    {
                        assert_eq!(Resource::$id.as_raw(), libc::$c_enum);
                    }
                )+
            }

            #[allow(unused_doc_comments)]
            #[test]
            fn from_str(){
                $(
                    $(#[$attr])*
                    {
                        assert_eq!(Resource::from_str(stringify!($c_enum)), Ok(Resource::$id));
                    }
                )+

                assert!(Resource::from_str("asdqwe").is_err());
            }

            #[test]
            fn available(){
                assert_eq!(
                    Resource::available_names().len(),
                    Resource::available_resources().len()
                );
            }
        }
    };
}

impl Resource {
    /// Set resource limits.
    /// # Errors
    /// See [`setrlimit`](fn.setrlimit.html)
    #[inline]
    pub fn set(self, soft: Rlim, hard: Rlim) -> io::Result<()> {
        super::setrlimit(self, soft, hard)
    }

    /// Get resource limits.
    /// # Errors
    /// See [`getrlimit`](fn.getrlimit.html)
    #[inline]
    pub fn get(self) -> io::Result<(Rlim, Rlim)> {
        super::getrlimit(self)
    }

    /// Returns the name of the resource.
    ///
    /// # Example
    /// ```
    /// # use rlimit::Resource;
    /// assert_eq!(Resource::NOFILE.as_name(), "RLIMIT_NOFILE");
    /// ```
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // this method should never panic
    pub fn as_name(self) -> &'static str {
        let idx = Self::VALUE_TABLE.iter().position(|&v| v == self).unwrap();
        Self::NAME_TABLE[idx]
    }

    /// Returns available resource names.
    #[must_use]
    pub const fn available_names() -> &'static [&'static str] {
        Self::NAME_TABLE
    }

    /// Returns available resources.
    #[must_use]
    pub const fn available_resources() -> &'static [Self] {
        Self::VALUE_TABLE
    }

    /// Returns the raw resource type.
    ///
    /// **Be careful**: The actual type of [`RawResource`][RawResource] can be different on different platforms.
    ///
    /// [RawResource]: type.RawResource.html
    #[inline]
    #[must_use]
    pub const fn as_raw(self) -> RawResource {
        self.value as _
    }
}

include!("__resources.rs");
