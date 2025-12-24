//! Demonstrates running with the `asm` feature to avoid linking libc on
//! x86_64 Linux/Android.

#[cfg(all(feature = "asm", rlimit__asm_syscall))]
fn main() {
    use rlimit::{getrlimit, setrlimit, Resource};

    let res = Resource::NOFILE;
    let (soft, hard) = getrlimit(res).expect("getrlimit");
    println!("NOFILE: soft={soft}, hard={hard}");

    // A no-op update that exercises the asm syscall path.
    setrlimit(res, soft.min(hard), hard).expect("setrlimit");
    println!("setrlimit via asm syscalls succeeded");
}

#[cfg(not(all(feature = "asm", rlimit__asm_syscall)))]
fn main() {
    println!("Enable the `asm` feature on x86_64 Linux/Android to run this example.");
}
