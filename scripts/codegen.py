from pathlib import Path

target_list = [
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "aarch64-fuchsia",
    "aarch64-linux-android",
    "aarch64-pc-windows-msvc",
    "aarch64-unknown-linux-gnu",
    "aarch64-unknown-linux-musl",
    "aarch64-unknown-none",
    "aarch64-unknown-none-softfloat",
    "arm-linux-androideabi",
    "arm-unknown-linux-gnueabi",
    "arm-unknown-linux-gnueabihf",
    "arm-unknown-linux-musleabi",
    "arm-unknown-linux-musleabihf",
    "armebv7r-none-eabi",
    "armebv7r-none-eabihf",
    "armv5te-unknown-linux-gnueabi",
    "armv5te-unknown-linux-musleabi",
    "armv7-linux-androideabi",
    "armv7-unknown-linux-gnueabi",
    "armv7-unknown-linux-gnueabihf",
    "armv7-unknown-linux-musleabi",
    "armv7-unknown-linux-musleabihf",
    "armv7a-none-eabi",
    "armv7r-none-eabi",
    "armv7r-none-eabihf",
    "asmjs-unknown-emscripten",
    "i586-pc-windows-msvc",
    "i586-unknown-linux-gnu",
    "i586-unknown-linux-musl",
    "i686-linux-android",
    "i686-pc-windows-gnu",
    "i686-pc-windows-msvc",
    "i686-unknown-freebsd",
    "i686-unknown-linux-gnu",
    "i686-unknown-linux-musl",
    "mips-unknown-linux-gnu",
    "mips-unknown-linux-musl",
    "mips64-unknown-linux-gnuabi64",
    "mips64-unknown-linux-muslabi64",
    "mips64el-unknown-linux-gnuabi64",
    "mips64el-unknown-linux-muslabi64",
    "mipsel-unknown-linux-gnu",
    "mipsel-unknown-linux-musl",
    "nvptx64-nvidia-cuda",
    "powerpc-unknown-linux-gnu",
    "powerpc64-unknown-linux-gnu",
    "powerpc64le-unknown-linux-gnu",
    "riscv32i-unknown-none-elf",
    "riscv32imac-unknown-none-elf",
    "riscv32imc-unknown-none-elf",
    "riscv64gc-unknown-linux-gnu",
    "riscv64gc-unknown-none-elf",
    "riscv64imac-unknown-none-elf",
    "s390x-unknown-linux-gnu",
    "sparc64-unknown-linux-gnu",
    "sparcv9-sun-solaris",
    "thumbv6m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
    "thumbv7m-none-eabi",
    "thumbv7neon-linux-androideabi",
    "thumbv7neon-unknown-linux-gnueabihf",
    "thumbv8m.base-none-eabi",
    "thumbv8m.main-none-eabi",
    "thumbv8m.main-none-eabihf",
    "wasm32-unknown-emscripten",
    "wasm32-unknown-unknown",
    "wasm32-wasi",
    "x86_64-apple-darwin",
    "x86_64-apple-ios",
    "x86_64-fortanix-unknown-sgx",
    "x86_64-fuchsia",
    "x86_64-linux-android",
    "x86_64-pc-solaris",
    "x86_64-pc-windows-gnu",
    "x86_64-pc-windows-msvc",
    "x86_64-sun-solaris",
    "x86_64-unknown-freebsd",
    "x86_64-unknown-illumos",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-gnux32",
    "x86_64-unknown-linux-musl",
    "x86_64-unknown-netbsd",
    "x86_64-unknown-redox",
]

arch_map = {
    "aarch64": "aarch64",
    "arm": "arm",
    "armebv7r": "arm",
    "armv5te": "arm",
    "armv7": "arm",
    "armv7a": "arm",
    "armv7r": "arm",
    "asmjs": "asmjs",
    "i586": "x86",
    "i686": "x86",
    "mips": "mips",
    "mips64": "mips64",
    "mips64el": "mips64",
    "mipsel": "mips",
    "nvptx64": "nvptx64",
    "powerpc": "powerpc",
    "powerpc64": "powerpc64",
    "powerpc64le": "powerpc64",
    "riscv32i": "riscv32",
    "riscv32imac": "riscv32",
    "riscv32imc": "riscv32",
    "riscv64gc": "riscv64",
    "riscv64imac": "riscv64",
    "s390x": "s390x",
    "sparc64": "sparc64",
    "sparcv9": "sparc",
    "thumbv6m": "thumbv6",
    "thumbv7em": "thumbv7",
    "thumbv7m": "thumbv7",
    "thumbv7neon": "thumbv7",
    "wasm32": "wasm32",
    "x86_64": "x86_64",
}

env_map = {
    "gnueabi": "gnu",
    "gnueabihf": "gnu",
    "musleabi": "musl",
    "musleabihf": "musl",
    "gnuabi64": "gnu",
    "muslabi64": "musl",
}

if __name__ == "__main__":
    print("#![allow(non_camel_case_types)]\n")

    content_map = dict()

    for target in target_list:
        t = list(target.split("-"))
        t_arch = t[0]
        t_os = t[2] if len(t) > 2 else t[1]
        t_env = t[3] if len(t) > 3 else None

        outrs = Path(target) / "out.rs"
        if not outrs.exists():
            continue
        content = open(outrs, "r", encoding="utf8").read()

        t_arch = arch_map[t_arch]

        if t_os == "darwin":
            t_os = "macos"
        elif t_os == "androideabi":
            t_os = "android"

        if t_env is not None and t_env in env_map:
            t_env = env_map[t_env]

        if t_env is not None:
            cfg = f'#[cfg(all(target_arch="{t_arch}", target_os="{t_os}", target_env="{t_env}"))]'
        else:
            cfg = f'#[cfg(all(target_arch="{t_arch}", target_os="{t_os}"))]'

        if target == "x86_64-unknown-linux-gnu":
            cfg = '#[cfg(any(all(doc, windows), all(target_arch = "x86_64", target_os = "linux", target_env = "gnu")))]'

        if (cfg in content_map) and content_map[cfg][1] == content:
            a = target.ljust(40, " ")
            b = content_map[cfg][0].ljust(40, " ")
            print(f"// {a} ~ {b}")
            print()
            continue
        content_map[cfg] = (target, content)

        mod_name = target.replace("-", "_")
        print(cfg)
        print(f"pub mod {mod_name} {{")
        print(content)
        print("}")
        print(cfg)
        print(f"pub use self::{mod_name}::*;")
        print()
