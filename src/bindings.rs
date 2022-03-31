#![allow(non_camel_case_types)]

#[cfg(all(target_arch = "aarch64", target_os = "android"))]
pub mod aarch64_linux_android {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "aarch64", target_os = "android"))]
pub use self::aarch64_linux_android::*;

#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"))]
pub mod aarch64_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"))]
pub use self::aarch64_unknown_linux_gnu::*;

#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "musl"))]
pub mod aarch64_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "musl"))]
pub use self::aarch64_unknown_linux_musl::*;

#[cfg(all(target_arch = "arm", target_os = "androideabi"))]
pub mod arm_linux_androideabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "arm", target_os = "androideabi"))]
pub use self::arm_linux_androideabi::*;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnueabi"))]
pub mod arm_unknown_linux_gnueabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnueabi"))]
pub use self::arm_unknown_linux_gnueabi::*;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnueabihf"))]
pub mod arm_unknown_linux_gnueabihf {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnueabihf"))]
pub use self::arm_unknown_linux_gnueabihf::*;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musleabi"))]
pub mod arm_unknown_linux_musleabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musleabi"))]
pub use self::arm_unknown_linux_musleabi::*;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musleabihf"))]
pub mod arm_unknown_linux_musleabihf {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musleabihf"))]
pub use self::arm_unknown_linux_musleabihf::*;

#[cfg(all(target_arch = "armv5te", target_os = "linux", target_env = "gnueabi"))]
pub mod armv5te_unknown_linux_gnueabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "armv5te", target_os = "linux", target_env = "gnueabi"))]
pub use self::armv5te_unknown_linux_gnueabi::*;

#[cfg(all(target_arch = "armv5te", target_os = "linux", target_env = "musleabi"))]
pub mod armv5te_unknown_linux_musleabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "armv5te", target_os = "linux", target_env = "musleabi"))]
pub use self::armv5te_unknown_linux_musleabi::*;

#[cfg(all(target_arch = "armv7", target_os = "androideabi"))]
pub mod armv7_linux_androideabi {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "armv7", target_os = "androideabi"))]
pub use self::armv7_linux_androideabi::*;

#[cfg(all(target_arch = "armv7", target_os = "linux", target_env = "gnueabihf"))]
pub mod armv7_unknown_linux_gnueabihf {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "armv7", target_os = "linux", target_env = "gnueabihf"))]
pub use self::armv7_unknown_linux_gnueabihf::*;

#[cfg(all(target_arch = "armv7", target_os = "linux", target_env = "musleabihf"))]
pub mod armv7_unknown_linux_musleabihf {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "armv7", target_os = "linux", target_env = "musleabihf"))]
pub use self::armv7_unknown_linux_musleabihf::*;

#[cfg(all(target_arch = "i586", target_os = "linux", target_env = "gnu"))]
pub mod i586_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "i586", target_os = "linux", target_env = "gnu"))]
pub use self::i586_unknown_linux_gnu::*;

#[cfg(all(target_arch = "i586", target_os = "linux", target_env = "musl"))]
pub mod i586_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "i586", target_os = "linux", target_env = "musl"))]
pub use self::i586_unknown_linux_musl::*;

#[cfg(all(target_arch = "i686", target_os = "android"))]
pub mod i686_linux_android {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "i686", target_os = "android"))]
pub use self::i686_linux_android::*;

#[cfg(all(target_arch = "i686", target_os = "linux", target_env = "gnu"))]
pub mod i686_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "i686", target_os = "linux", target_env = "gnu"))]
pub use self::i686_unknown_linux_gnu::*;

#[cfg(all(target_arch = "i686", target_os = "linux", target_env = "musl"))]
pub mod i686_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "i686", target_os = "linux", target_env = "musl"))]
pub use self::i686_unknown_linux_musl::*;

#[cfg(all(target_arch = "mips", target_os = "linux", target_env = "gnu"))]
pub mod mips_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mips", target_os = "linux", target_env = "gnu"))]
pub use self::mips_unknown_linux_gnu::*;

#[cfg(all(target_arch = "mips", target_os = "linux", target_env = "musl"))]
pub mod mips_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mips", target_os = "linux", target_env = "musl"))]
pub use self::mips_unknown_linux_musl::*;

#[cfg(all(target_arch = "mips64", target_os = "linux", target_env = "gnuabi64"))]
pub mod mips64_unknown_linux_gnuabi64 {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mips64", target_os = "linux", target_env = "gnuabi64"))]
pub use self::mips64_unknown_linux_gnuabi64::*;

#[cfg(all(target_arch = "mips64el", target_os = "linux", target_env = "gnuabi64"))]
pub mod mips64el_unknown_linux_gnuabi64 {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mips64el", target_os = "linux", target_env = "gnuabi64"))]
pub use self::mips64el_unknown_linux_gnuabi64::*;

#[cfg(all(target_arch = "mipsel", target_os = "linux", target_env = "gnu"))]
pub mod mipsel_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mipsel", target_os = "linux", target_env = "gnu"))]
pub use self::mipsel_unknown_linux_gnu::*;

#[cfg(all(target_arch = "mipsel", target_os = "linux", target_env = "musl"))]
pub mod mipsel_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "mipsel", target_os = "linux", target_env = "musl"))]
pub use self::mipsel_unknown_linux_musl::*;

#[cfg(all(target_arch = "powerpc", target_os = "linux", target_env = "gnu"))]
pub mod powerpc_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "powerpc", target_os = "linux", target_env = "gnu"))]
pub use self::powerpc_unknown_linux_gnu::*;

#[cfg(all(target_arch = "powerpc64le", target_os = "linux", target_env = "gnu"))]
pub mod powerpc64le_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "powerpc64le", target_os = "linux", target_env = "gnu"))]
pub use self::powerpc64le_unknown_linux_gnu::*;

#[cfg(all(target_arch = "riscv64gc", target_os = "linux", target_env = "gnu"))]
pub mod riscv64gc_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "riscv64gc", target_os = "linux", target_env = "gnu"))]
pub use self::riscv64gc_unknown_linux_gnu::*;

#[cfg(all(target_arch = "s390x", target_os = "linux", target_env = "gnu"))]
pub mod s390x_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "s390x", target_os = "linux", target_env = "gnu"))]
pub use self::s390x_unknown_linux_gnu::*;

#[cfg(all(target_arch = "x86_64", target_os = "android"))]
pub mod x86_64_linux_android {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "x86_64", target_os = "android"))]
pub use self::x86_64_linux_android::*;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub mod x86_64_unknown_linux_gnu {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
pub use self::x86_64_unknown_linux_gnu::*;

#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "musl"))]
pub mod x86_64_unknown_linux_musl {
    pub const RLIM_INFINITY: u64 = u64::MAX;
    pub const RLIMIT_AS: u8 = 9;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = 10;
    pub const RLIMIT_MEMLOCK: u8 = 8;
    pub const RLIMIT_MSGQUEUE: u8 = 12;
    pub const RLIMIT_NICE: u8 = 13;
    pub const RLIMIT_NOFILE: u8 = 7;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 6;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = 14;
    pub const RLIMIT_RTTIME: u8 = 15;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = 11;
    pub const RLIMIT_STACK: u8 = 3;
    pub const RLIMIT_SWAP: u8 = u8::MAX;
    pub const RLIMIT_UMTXP: u8 = u8::MAX;
    pub const RLIMIT_VMEM: u8 = u8::MAX;
    pub type rlim_t = u64;
    #[repr(C)]
    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }
    extern "C" {
        pub fn getrlimit64(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit64(resource: u32, rlimit: *const rlimit) -> i32;
    }
    pub use self::getrlimit64 as getrlimit;
    pub use self::setrlimit64 as setrlimit;
}
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "musl"))]
pub use self::x86_64_unknown_linux_musl::*;
