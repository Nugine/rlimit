use rlimit::{getrlimit, rlim, Resource, RLIM_INFINITY};
use std::io::ErrorKind;

const SOFT: rlim = 4 * 1024 * 1024;
const HARD: rlim = 8 * 1024 * 1024;

#[test]
fn test_eq() {
    assert_eq!(Resource::FSIZE.as_raw_resource(), libc::RLIMIT_FSIZE);
}

#[test]
fn test_set_get() {
    assert!(Resource::FSIZE.set(SOFT, HARD).is_ok());

    // assert!(setrlimit(Resource::FSIZE, SOFT, HARD).is_ok());

    assert_eq!(Resource::FSIZE.get().unwrap(), (SOFT, HARD));

    assert_eq!(
        Resource::FSIZE.set(HARD, SOFT).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        Resource::FSIZE.set(HARD, HARD + 1).unwrap_err().kind(),
        ErrorKind::PermissionDenied
    );
}

#[test]
fn test_get() {
    assert_eq!(
        getrlimit(Resource::CPU).unwrap(),
        (RLIM_INFINITY, RLIM_INFINITY)
    );
}

#[cfg(target_os = "linux")]
#[test]
fn test_prlimit() {
    use rlimit::{prlimit, prlimit_with_old};
    let res = Resource::CORE;

    assert!(prlimit(0, res, Some((SOFT, HARD))).is_ok());

    assert_eq!(prlimit_with_old(0, res, None).unwrap(), (SOFT, HARD));

    assert_eq!(
        prlimit(0, res, Some((HARD, SOFT))).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        prlimit(0, res, Some((HARD, HARD + 1))).unwrap_err().kind(),
        ErrorKind::PermissionDenied
    );
}
