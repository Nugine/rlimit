mod linux;
mod rlim;
mod unix;

use std::io;

#[track_caller]
fn expect_ok(result: io::Result<()>) {
    assert!(result.is_ok());
}

#[track_caller]
fn expect_err(result: io::Result<()>, kind: io::ErrorKind) {
    assert_eq!(result.unwrap_err().kind(), kind);
}

#[test]
fn utils_nofile() {
    use rlimit::utils::increase_nofile_limit;
    let lim = increase_nofile_limit(u64::MAX).unwrap();
    dbg!(lim);
}
