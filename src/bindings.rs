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

#[cfg(all(target_arch = "arm", target_os = "android"))]
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
#[cfg(all(target_arch = "arm", target_os = "android"))]
pub use self::arm_linux_androideabi::*;

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
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
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
pub use self::arm_unknown_linux_gnueabi::*;

// arm-unknown-linux-gnueabihf              ~ arm-unknown-linux-gnueabi

#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musl"))]
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
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "musl"))]
pub use self::arm_unknown_linux_musleabi::*;

// arm-unknown-linux-musleabihf             ~ arm-unknown-linux-musleabi

// armv5te-unknown-linux-gnueabi            ~ arm-unknown-linux-gnueabi

// armv5te-unknown-linux-musleabi           ~ arm-unknown-linux-musleabi

// armv7-linux-androideabi                  ~ arm-linux-androideabi

// armv7-unknown-linux-gnueabihf            ~ arm-unknown-linux-gnueabi

// armv7-unknown-linux-musleabihf           ~ arm-unknown-linux-musleabi

#[cfg(all(target_arch = "x86", target_os = "linux", target_env = "gnu"))]
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
#[cfg(all(target_arch = "x86", target_os = "linux", target_env = "gnu"))]
pub use self::i586_unknown_linux_gnu::*;

#[cfg(all(target_arch = "x86", target_os = "linux", target_env = "musl"))]
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
#[cfg(all(target_arch = "x86", target_os = "linux", target_env = "musl"))]
pub use self::i586_unknown_linux_musl::*;

#[cfg(all(target_arch = "x86", target_os = "android"))]
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
#[cfg(all(target_arch = "x86", target_os = "android"))]
pub use self::i686_linux_android::*;

// i686-unknown-linux-gnu                   ~ i586-unknown-linux-gnu

// i686-unknown-linux-musl                  ~ i586-unknown-linux-musl

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

#[cfg(all(target_arch = "mips64", target_os = "linux", target_env = "gnu"))]
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
#[cfg(all(target_arch = "mips64", target_os = "linux", target_env = "gnu"))]
pub use self::mips64_unknown_linux_gnuabi64::*;

// mips64el-unknown-linux-gnuabi64          ~ mips64-unknown-linux-gnuabi64

// mipsel-unknown-linux-gnu                 ~ mips-unknown-linux-gnu

// mipsel-unknown-linux-musl                ~ mips-unknown-linux-musl

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

#[cfg(all(target_arch = "powerpc64", target_os = "linux", target_env = "gnu"))]
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
#[cfg(all(target_arch = "powerpc64", target_os = "linux", target_env = "gnu"))]
pub use self::powerpc64le_unknown_linux_gnu::*;

#[cfg(all(target_arch = "riscv64", target_os = "linux", target_env = "gnu"))]
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
#[cfg(all(target_arch = "riscv64", target_os = "linux", target_env = "gnu"))]
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

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
pub mod x86_64_apple_darwin {
    pub const RLIM_INFINITY: u64 = 9223372036854775807;
    pub const RLIMIT_AS: u8 = 5;
    pub const RLIMIT_CORE: u8 = 4;
    pub const RLIMIT_CPU: u8 = 0;
    pub const RLIMIT_DATA: u8 = 2;
    pub const RLIMIT_FSIZE: u8 = 1;
    pub const RLIMIT_KQUEUES: u8 = u8::MAX;
    pub const RLIMIT_LOCKS: u8 = u8::MAX;
    pub const RLIMIT_MEMLOCK: u8 = 6;
    pub const RLIMIT_MSGQUEUE: u8 = u8::MAX;
    pub const RLIMIT_NICE: u8 = u8::MAX;
    pub const RLIMIT_NOFILE: u8 = 8;
    pub const RLIMIT_NOVMON: u8 = u8::MAX;
    pub const RLIMIT_NPROC: u8 = 7;
    pub const RLIMIT_NPTS: u8 = u8::MAX;
    pub const RLIMIT_NTHR: u8 = u8::MAX;
    pub const RLIMIT_POSIXLOCKS: u8 = u8::MAX;
    pub const RLIMIT_RSS: u8 = 5;
    pub const RLIMIT_RTPRIO: u8 = u8::MAX;
    pub const RLIMIT_RTTIME: u8 = u8::MAX;
    pub const RLIMIT_SBSIZE: u8 = u8::MAX;
    pub const RLIMIT_SIGPENDING: u8 = u8::MAX;
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
        pub fn getrlimit(resource: u32, rlimit: *mut rlimit) -> i32;
        pub fn setrlimit(resource: u32, rlimit: *const rlimit) -> i32;
    }
}
#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
pub use self::x86_64_apple_darwin::*;

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

#[cfg(any(
    all(doc, windows),
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu")
))]
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
#[cfg(any(
    all(doc, windows),
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu")
))]
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
