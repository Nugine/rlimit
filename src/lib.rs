#[cfg(test)]
mod test;

use libc::{
    RLIMIT_AS, RLIMIT_CORE, RLIMIT_CPU, RLIMIT_DATA, RLIMIT_FSIZE, RLIMIT_LOCKS, RLIMIT_MEMLOCK,
    RLIMIT_MSGQUEUE, RLIMIT_NICE, RLIMIT_NLIMITS, RLIMIT_NOFILE, RLIMIT_NPROC, RLIMIT_RSS,
    RLIMIT_RTPRIO, RLIMIT_RTTIME, RLIMIT_SIGPENDING, RLIMIT_STACK,
};

pub const RLIM_INFINITY: u64 = libc::RLIM_INFINITY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RLimit {
    AS = RLIMIT_AS as isize,
    CORE = RLIMIT_CORE as isize,
    CPU = RLIMIT_CPU as isize,
    DATA = RLIMIT_DATA as isize,
    FSIZE = RLIMIT_FSIZE as isize,
    LOCKS = RLIMIT_LOCKS as isize,
    MEMLOCK = RLIMIT_MEMLOCK as isize,
    MSGQUEUE = RLIMIT_MSGQUEUE as isize,
    NICE = RLIMIT_NICE as isize,
    NLIMITS = RLIMIT_NLIMITS as isize,
    NOFILE = RLIMIT_NOFILE as isize,
    NPROC = RLIMIT_NPROC as isize,
    RSS = RLIMIT_RSS as isize,
    RTPRIO = RLIMIT_RTPRIO as isize,
    RTTIME = RLIMIT_RTTIME as isize,
    SIGPENDING = RLIMIT_SIGPENDING as isize,
    STACK = RLIMIT_STACK as isize,
}

impl RLimit {
    #[inline(always)]
    pub fn to_c_int(&self) -> libc::c_int {
        *self as isize as i32
    }

    #[inline(always)]
    unsafe fn new_limit(cur: u64, max: u64) -> libc::rlimit {
        let mut limit = std::mem::zeroed::<libc::rlimit>();
        limit.rlim_cur = cur;
        limit.rlim_max = max;
        limit
    }

    pub fn set(&self, soft: u64, hard: u64) -> Result<(), i32> {
        let mut limit = unsafe { RLimit::new_limit(soft, hard) };
        let resource = self.to_c_int();
        let code = unsafe { libc::setrlimit(resource, &mut limit as *mut libc::rlimit) };
        if code == 0 {
            Ok(())
        } else {
            Err(code)
        }
    }

    pub fn get(&self) -> Result<(u64, u64), i32> {
        let mut limit = unsafe { std::mem::zeroed::<libc::rlimit>() };
        let resource = self.to_c_int();
        let code = unsafe { libc::getrlimit(resource, &mut limit as *mut libc::rlimit) };
        if code == 0 {
            Ok((limit.rlim_cur, limit.rlim_max))
        } else {
            Err(code)
        }
    }
}
