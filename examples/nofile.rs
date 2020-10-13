use rlimit::{Resource, Rlim};
use std::cmp;
use std::io;

const DEFAULT_NOFILE_LIMIT: Rlim = Rlim::from_raw(16384); // or another number

/// Try to increase NOFILE limit and return the current soft limit.
fn increase_nofile_limit() -> io::Result<Rlim> {
    let (soft, hard) = Resource::NOFILE.get()?;
    println!("Before increasing: soft   = {}, hard = {}", soft, hard);

    let target = cmp::min(DEFAULT_NOFILE_LIMIT, hard);
    println!("Try to increase:   target = {}", target);
    Resource::NOFILE.set(target, target)?;

    let (soft, hard) = Resource::NOFILE.get()?;
    println!("After increasing:  soft   = {}, hard = {}", soft, hard);
    Ok(soft)
}

fn main() {
    match increase_nofile_limit() {
        Ok(soft) => println!("NOFILE limit:      soft   = {}", soft),
        Err(err) => println!("Failed to increase NOFILE limit: {}", err),
    }
}
