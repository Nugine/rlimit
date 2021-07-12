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
    /// A value of Rlim indicating no limit.
"""

docs[
    "SAVED_CUR"
] = """
    /// A value of type Rlim indicating an unrepresentable saved soft limit.
"""

docs[
    "SAVED_MAX"
] = """
    /// A value of type Rlim indicating an unrepresentable saved hard limit.
"""

if __name__ == "__main__":
    rlims = libc_source.search_ident("RLIM_.+?:", ".+RLIM_(.+?):")
    del rlims["NLIMITS"]

    selectors = libc_source.calc_selectors(rlims)

    print(f"// generated from rust-lang/libc {libc_source.COMMIT_HASH}")

    print("impl Rlim {", end="")
    for rlim in selectors:
        cfg = "".join(f"\n        {v}," for v in selectors[rlim].values())
        cfg = f"    #[cfg(any({cfg}\n    ))]"
        print(docs[rlim], end="")
        print(cfg)
        print(f"    pub const {rlim}: Self = Self(libc::RLIM_{rlim});")
    print("}")
