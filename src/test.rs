use crate::{RLimit, RLIM_INFINITY};
use std::io::ErrorKind;

const SOFT: u64 = 1024 * 1024 * 1024;
const HARD: u64 = 2048 * 1024 * 1024;

#[test]
fn test_eq() {
    assert_eq!(RLimit::AS.to_c_uint(), libc::RLIMIT_AS);
}

#[test]
fn test_set_get() {
    assert!(RLimit::AS.set(SOFT, HARD).is_ok());

    assert_eq!(RLimit::AS.get().unwrap(), (SOFT, HARD));

    assert_eq!(
        RLimit::AS.set(HARD, SOFT).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(
        RLimit::AS.set(HARD, HARD + 1).unwrap_err().kind(),
        ErrorKind::PermissionDenied
    );
}

#[test]
fn test_get() {
    assert_eq!(RLimit::CPU.get().unwrap(), (RLIM_INFINITY, RLIM_INFINITY));
}
