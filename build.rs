fn main() {
    let has_prlimit64 = cfg!(any(
        all(target_os = "android", not(target_env = "newlib")),
        all(
            any(target_env = "gnu", target_env = "musl", target_env = "ohos"),
            any(target_os = "l4re", target_os = "linux")
        )
    ));
    if has_prlimit64 {
        println!("cargo:rustc-cfg=rlimit__has_prlimit64");
    }

    let get_kern_max_files_per_proc = cfg!(all(
        any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos"
        ),
        not(target_env = "newlib")
    ));
    if get_kern_max_files_per_proc {
        println!("cargo:rustc-cfg=rlimit__get_kern_max_files_per_proc");
    }
}
