use super::atomically;

#[test]
fn maxstdio() {
    atomically(|| {
        assert_eq!(rlimit::getmaxstdio(), 512);
        assert_eq!(rlimit::setmaxstdio(2048).unwrap(), 2048);
        assert_eq!(rlimit::getmaxstdio(), 2048);
    });
}

#[test]
fn maxstdio_overflow() {
    // Test that u32::MAX is rejected (would overflow to negative when cast to c_int)
    let result = rlimit::setmaxstdio(u32::MAX);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn maxstdio_large_value() {
    // Test that values larger than c_int::MAX are rejected
    let c_int_max = std::os::raw::c_int::MAX as u32;
    // Only test if we can represent a value larger than c_int::MAX in u32
    if let Some(test_value) = c_int_max.checked_add(1) {
        let result = rlimit::setmaxstdio(test_value);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidInput);
    }
}
