import os
import json
import re

from typing import Dict, List

from . import libc_source
from . import epprint

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

    c_rlimit_idents = libc_source.search_ident(r"pub struct rlimit64 \{", ".+(rlimit64).+")
    selectors = libc_source.calc_selectors(c_rlimit_idents)

    print(libc_source.calc_cfg(sorted(selectors["rlimit64"].values()), indent=0))
    print(f"type c_rlimit = libc::rlimit64;")
    print()
    print(libc_source.calc_cfg(sorted(selectors["rlimit64"].values()), indent=0, inverse=True))
    print(f"type c_rlimit = libc::rlimit;")
    print()

    c_sgetrlimit_idents = libc_source.search_ident("fn [sg]etrlimit64", ".+([sg]etrlimit)64.+")
    selectors = libc_source.calc_selectors(c_sgetrlimit_idents)
    for f in sorted(selectors.keys()):
        print(libc_source.calc_cfg(sorted(selectors[f].values()), indent=0))
        print(f"use libc::{f}64 as c_{f};")
        print()
        print(libc_source.calc_cfg(sorted(selectors[f].values()), indent=0, inverse=True))
        print(f"use libc::{f} as c_{f};")
        print()

    rlims = libc_source.search_ident("RLIM_.+?:", ".+RLIM_(.+?):")
    del rlims["NLIMITS"]
    selectors = libc_source.calc_selectors(rlims)

    for rlim in sorted(selectors.keys()):
        print(docs[rlim], end="")
        print(libc_source.calc_cfg(sorted(selectors[rlim].values()), indent=0))
        print(f"pub const {rlim}: u64 = libc::RLIM_{rlim} as u64;")
        print()

        if rlim == "INFINITY":
            print(docs[rlim], end="")
            print(libc_source.calc_cfg(sorted(selectors[rlim].values()), indent=0, inverse=True))
            print(f"pub const {rlim}: u64 = u64::MAX;")
            print()
