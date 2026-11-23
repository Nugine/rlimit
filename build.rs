fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();

    println!("cargo:rustc-check-cfg=cfg(target_os, values(\"switch\",\"qurt\"))");

    println!("cargo:rustc-check-cfg=cfg(gnu_file_offset_bits64)");

    let has_prlimit64 = (target_os == "android" && target_env != "newlib")
        || (target_os == "linux"
            && (target_env == "gnu" || target_env == "musl" || target_env == "ohos"));
    println!("cargo:rustc-check-cfg=cfg(rlimit__has_prlimit64)");
    if has_prlimit64 {
        println!("cargo:rustc-cfg=rlimit__has_prlimit64");
    }

    let has_getrlimit = ((target_os == "aix"
        || target_os == "android"
        || target_os == "cygwin"
        || target_os == "dragonfly"
        || target_os == "emscripten"
        || target_os == "freebsd"
        || target_os == "haiku"
        || target_os == "hurd"
        || target_os == "illumos"
        || target_os == "ios"
        || target_os == "macos"
        || target_os == "netbsd"
        || target_os == "nto"
        || target_os == "openbsd"
        || target_os == "qnx"
        || target_os == "redox"
        || target_os == "solaris"
        || target_os == "tvos"
        || target_os == "visionos"
        || target_os == "watchos")
        && target_env != "newlib")
        || (target_env == "uclibc" && target_os == "l4re")
        || (target_env == "gnu" && target_os == "linux")
        || (target_os == "linux" && (target_env == "musl" || target_env == "ohos"))
        || (target_env == "uclibc" && target_os == "linux")
        || (std::env::var("CARGO_CFG_UNIX").is_ok()
            && target_env == "newlib"
            && !(target_os == "fuchsia"
                || target_os == "qurt"
                || target_os == "solid_asp3"
                || target_os == "switch"
                || target_os == "vxworks"));
    println!("cargo:rustc-check-cfg=cfg(rlimit__has_getrlimit)");
    if has_getrlimit {
        println!("cargo:rustc-cfg=rlimit__has_getrlimit");
    }

    let has_setrlimit = ((target_os == "aix"
        || target_os == "android"
        || target_os == "cygwin"
        || target_os == "dragonfly"
        || target_os == "emscripten"
        || target_os == "freebsd"
        || target_os == "haiku"
        || target_os == "hurd"
        || target_os == "illumos"
        || target_os == "ios"
        || target_os == "macos"
        || target_os == "netbsd"
        || target_os == "nto"
        || target_os == "openbsd"
        || target_os == "qnx"
        || target_os == "redox"
        || target_os == "solaris"
        || target_os == "tvos"
        || target_os == "visionos"
        || target_os == "watchos")
        && target_env != "newlib")
        || (target_env == "uclibc" && target_os == "l4re")
        || (target_env == "gnu" && target_os == "linux")
        || (target_os == "linux" && (target_env == "musl" || target_env == "ohos"))
        || (target_env == "uclibc" && target_os == "linux")
        || (std::env::var("CARGO_CFG_UNIX").is_ok()
            && target_env == "newlib"
            && !(target_os == "fuchsia"
                || target_os == "qurt"
                || target_os == "solid_asp3"
                || target_os == "switch"
                || target_os == "vxworks"));
    println!("cargo:rustc-check-cfg=cfg(rlimit__has_setrlimit)");
    if has_setrlimit {
        println!("cargo:rustc-cfg=rlimit__has_setrlimit");
    }

    let get_kern_max_files_per_proc = (target_os == "dragonfly"
        || target_os == "freebsd"
        || target_os == "ios"
        || target_os == "macos"
        || target_os == "tvos"
        || target_os == "visionos"
        || target_os == "watchos")
        && target_env != "newlib";
    println!("cargo:rustc-check-cfg=cfg(rlimit__get_kern_max_files_per_proc)");
    if get_kern_max_files_per_proc {
        println!("cargo:rustc-cfg=rlimit__get_kern_max_files_per_proc");
    }
}
