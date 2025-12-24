fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    println!("cargo:rustc-check-cfg=cfg(target_os, values(\"switch\"))");
    let has_prlimit64 = (target_os == "android" && target_env != "newlib")
        || (target_os == "linux"
            && (target_env == "gnu" || target_env == "musl" || target_env == "ohos"));
    println!("cargo:rustc-check-cfg=cfg(rlimit__has_prlimit64)");
    if has_prlimit64 {
        println!("cargo:rustc-cfg=rlimit__has_prlimit64");
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

    let asm_feature = std::env::var("CARGO_FEATURE_ASM").is_ok();
    let asm_syscall = asm_feature
        && target_arch == "x86_64"
        && (target_os == "linux" || target_os == "android");
    println!("cargo:rustc-check-cfg=cfg(rlimit__asm_syscall)");
    if asm_syscall {
        println!("cargo:rustc-cfg=rlimit__asm_syscall");
    }
}
