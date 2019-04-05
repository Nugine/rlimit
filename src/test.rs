use crate::{RLimit, RLIM_INFINITY};

const SOFT: u64 = 128 * 1024 * 1024;
const HARD: u64 = 256 * 1024 * 1024;

#[test]
fn test_eq() {
    assert_eq!(RLimit::AS.to_c_int(), libc::RLIMIT_AS);
}

#[test]
fn test_set_get() {
    assert_eq!(RLimit::AS.set(SOFT, HARD), Ok(()));
    assert_eq!(RLimit::AS.get(), Ok((SOFT, HARD)));

    assert_eq!(RLimit::AS.set(HARD, SOFT), Err(-1));
    assert_eq!(RLimit::AS.set(HARD, HARD + 1), Err(-1));
}

#[test]
fn test_get() {
    assert_eq!(RLimit::CPU.get(), Ok((RLIM_INFINITY, RLIM_INFINITY)));
}
