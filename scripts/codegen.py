from typing import Optional, Dict, List
import json
import os
import re
from pprint import pprint

LIBC_REPO_PATH = os.getenv("LIBC_REPO_PATH", "libc")

PREDICATES = {
    "fuchsia/mod.rs": {"os": ["fuchsia"]},
    "unix/bsd/apple/mod.rs": {"os": ["macos", "ios"]},
    "unix/bsd/freebsdlike/dragonfly/mod.rs": {"os": ["dragonfly"]},
    "unix/bsd/freebsdlike/freebsd/mod.rs": {"os": ["freebsd"]},
    "unix/bsd/freebsdlike/mod.rs": {"os": ["freebsd", "dragonfly"]},
    "unix/bsd/netbsdlike/mod.rs": {"os": ["openbsd", "netbsd"]},
    "unix/bsd/netbsdlike/netbsd/mod.rs": {"os": ["netbsd"]},
    "unix/haiku/mod.rs": {"os": ["haiku"]},
    "unix/linux_like/android/mod.rs": {"os": ["android"]},
    "unix/linux_like/emscripten/mod.rs": {"os": ["emscripten"]},
    "unix/linux_like/linux/arch/generic/mod.rs": {"os": ["linux"]},
    "unix/linux_like/linux/arch/mips/mod.rs": {"os": ["linux"], "arch": ["mips", "mips64"]},
    "unix/linux_like/linux/arch/powerpc/mod.rs": {"os": ["linux"], "arch": ["powerpc", "powerpc64"]},
    "unix/linux_like/linux/arch/sparc/mod.rs": {"os": ["linux"], "arch": ["sparc", "sparc64"]},
    "unix/solarish/mod.rs": {"os": ["solaris", "illumos"]},
    "unix/linux_like/linux/gnu/mod.rs": {"os": ["linux"], "env": ["gnu"]},
    "unix/linux_like/linux/musl/mod.rs": {"os": ["linux"], "env": ["musl"]},
    "unix/linux_like/linux/musl/b32/riscv32/mod.rs": {"os": ["linux"], "env": ["musl"], "arch": ["riscv32"]},
    "unix/linux_like/linux/uclibc/mod.rs": {"os": ["linux"], "env": ["uclibc"]},
    "unix/linux_like/android/b32/mod.rs": {"os": ["android"], "pointer_width": ["32"]},
    "unix/linux_like/android/b64/mod.rs": {"os": ["android"], "pointer_width": ["64"]},
    "unix/linux_like/linux/mod.rs": {"os": ["linux"]},
    "unix/mod.rs": {"family": ["unix"]},
    "vxworks/mod.rs": {"os": ["vxworks"]},
    "unix/bsd/mod.rs": {"os": ["macos", "ios", "watchos", "freebsd", "dragonfly", "openbsd", "netbsd"]},
    "unix/hermit/mod.rs": {"os": ["hermit"]},
    "unix/newlib/mod.rs": {"env": ["newlib"]},
    "unix/redox/mod.rs": {"os": ["redox"]},
    "unix/nto/mod.rs": {"os": ["nto"]},
}


def extract_paths(rg_lines: List[str]) -> List[str]:
    paths = set()
    for line in rg_lines:
        item = json.loads(line)
        if item["type"] == "match":
            file_path = item["data"]["path"]["text"]
            rel_file_path = re.match(".+src/(.+)", file_path).group(1)  # type: ignore
            paths.add(rel_file_path)
    return sorted(paths)


def search(prefix: str, ident: str) -> List[Dict[str, List[str]]]:
    pipe = os.popen(f"rg --json 'pub {prefix} {ident}' {LIBC_REPO_PATH}")
    lines = [l for l in pipe.read().split("\n") if l != ""]
    cfgs = [PREDICATES[path] for path in extract_paths(lines)]
    return cfgs


def emit_predicate(kind: str, cond: List[str]) -> str:
    if len(cond) == 1:
        return f'{kind} = "{cond[0]}"'
    else:
        return "any(" + ", ".join(f'{kind} = "{c}"' for c in cond) + ")"


def emit_cfg(cfgs: List[Dict[str, List[str]]], indent: int) -> str:
    predicates = []
    for cfg in cfgs:
        ps = []
        for kind in ["os", "arch", "env", "pointer_width", "family"]:
            if kind in cfg:
                ps.append(emit_predicate(f"target_{kind}", cfg[kind]))
        if len(ps) == 1:
            predicates.append(ps[0])
        else:
            predicates.append("all(" + ", ".join(ps) + ")")
    ans = "any(\n"
    for p in predicates:
        ans += "    " * (indent + 1) + p + ",\n"
    ans += "    " * indent + ")"
    return ans


if __name__ == "__main__":
    resources = [
        "RLIMIT_AS",
        "RLIMIT_CORE",
        "RLIMIT_CPU",
        "RLIMIT_DATA",
        "RLIMIT_FSIZE",
        "RLIMIT_KQUEUES",
        "RLIMIT_LOCKS",
        "RLIMIT_MEMLOCK",
        "RLIMIT_MSGQUEUE",
        "RLIMIT_NICE",
        "RLIMIT_NOFILE",
        "RLIMIT_NOVMON",
        "RLIMIT_NPROC",
        "RLIMIT_NPTS",
        "RLIMIT_NTHR",
        "RLIMIT_POSIXLOCKS",
        "RLIMIT_RSS",
        "RLIMIT_RTPRIO",
        "RLIMIT_RTTIME",
        "RLIMIT_SBSIZE",
        "RLIMIT_SIGPENDING",
        "RLIMIT_STACK",
        "RLIMIT_SWAP",
        "RLIMIT_UMTXP",
        "RLIMIT_VMEM",
    ]

    print("#![allow(clippy::cast_possible_truncation)]\n")
    print("#![allow(clippy::unnecessary_cast)]\n")

    resource_cfgs = []
    for resource in resources:
        cfg = emit_cfg(search("const", resource), indent=0)
        resource_cfgs.append((resource, cfg))

        print(f"#[cfg({cfg})]")
        print(f"pub const {resource}: u8 = libc::{resource} as u8;")
        print()

        print(f"#[cfg(not({cfg}))]")
        print(f"pub const {resource}: u8 = u8::MAX;")
        print()

        print("// " + "-" * 77)
        print()

    print("#[allow(clippy::too_many_lines)]")
    print("#[test]")
    print("fn resource_bound() {")
    for resource, cfg in resource_cfgs:
        print(f"    #[cfg({cfg})]")
        print(f"    assert!((0..128).contains(&libc::{resource}));")
        print()
    print("}")
    print()

    for ident in ["rlimit", "getrlimit", "setrlimit"]:
        if ident == "rlimit":
            cfg64 = emit_cfg(search("struct", ident + "64"), indent=0)
            cfg = emit_cfg(search("struct", ident), indent=0)
        else:
            cfg64 = emit_cfg(search("fn", ident + "64"), indent=0)
            cfg = emit_cfg(search("fn", ident), indent=0)

        print(f"#[cfg({cfg64})]")
        print(f"pub use libc::{ident}64 as {ident};")
        print()
        print(f"#[cfg(all(not({cfg64}), {cfg}))]")
        print(f"pub use libc::{ident};")
        print()

    ident = "RLIM_INFINITY"
    cfg = emit_cfg(search("const", ident), indent=0)
    print(f"#[cfg({cfg})]")
    print(f"pub const {ident}: u64 = libc::{ident} as u64;")
    print()
    print(f"#[cfg(not({cfg}))]")
    print(f"pub const {ident}: u64 = u64::MAX;")
    print()
