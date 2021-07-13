import os
import json
import re
from pprint import pprint
from typing import Dict, List

from . import libc_source

docs = {}

docs[
    "INFINITY"
] = """
/// A value indicating no limit.
"""

docs[
    "SAVED_CUR"
] = """
/// A value indicating an unrepresentable saved soft limit.
"""

docs[
    "SAVED_MAX"
] = """
/// A value indicating an unrepresentable saved hard limit.
"""

if __name__ == "__main__":
    print(f"// generated from rust-lang/libc {libc_source.COMMIT_HASH}")

    rlim64_t_idents = libc_source.search_ident("type rlim64_t", ".+(rlim64_t).+")
    selectors = libc_source.calc_selectors(rlim64_t_idents)

    cfg = "".join(f"\n    {v}," for v in selectors["rlim64_t"].values())

    print(f"#[cfg(any({cfg}\n))]")
    print("group! {")
    print(f"    type c_rlimit = libc::rlimit64;")
    print(f"    use libc::setrlimit64 as c_setrlimit;")
    print(f"    use libc::getrlimit64 as c_getrlimit;")
    print(f"    const RLIM_INFINITY: u64 = u64::MAX;")
    print(f"    const RLIM_SAVED_CUR: u64 = u64::MAX;")
    print(f"    const RLIM_SAVED_MAX: u64 = u64::MAX;")
    print("}")
    print()

    print(f"#[cfg(not(any({cfg}\n)))]")
    print("group! {")
    print(f"    type c_rlimit = libc::rlimit;")
    print(f"    use libc::setrlimit as c_setrlimit;")
    print(f"    use libc::getrlimit as c_getrlimit;")

    rlims = libc_source.search_ident("RLIM_.+?:", ".+RLIM_(.+?):")
    del rlims["NLIMITS"]
    selectors = libc_source.calc_selectors(rlims)

    for rlim in sorted(selectors.keys()):
        cfg = "".join(f"\n        {v}," for v in selectors[rlim].values())
        print(f"    #[cfg(any({cfg}\n    ))]")
        print(f"    const RLIM_{rlim}: u64 = libc::RLIM_{rlim} as u64;")
    print("}")
    print()

    for rlim in sorted(selectors.keys()):
        cfg = "".join(f"\n    {v}," for v in selectors[rlim].values())
        print(docs[rlim], end="")
        print(f"#[cfg(any({cfg}\n))]")
        print(f"pub const {rlim}: u64 = RLIM_{rlim};")
        print()
