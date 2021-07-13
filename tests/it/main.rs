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
