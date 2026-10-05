#!/usr/bin/env python3
"""Cross-check `utf-mcp dump` against Python's `unicodedata`, an independent
implementation of an *older* Unicode version. Only code points whose
DerivedAge (per utf-mcp) is <= Python's unidata_version are compared, so every
difference is either a property Unicode changed after that version or a bug.

    target/release/utf-mcp dump > /tmp/dump.tsv
    python3.14 tests/crosscheck_unicodedata.py /tmp/dump.tsv

Prints per-field mismatch counts and the first examples of each.
"""
import sys
import unicodedata as ud
from collections import defaultdict
from fractions import Fraction


def vtuple(v):
    return tuple(int(x) for x in v.split("."))


def main(path):
    pyver = vtuple(ud.unidata_version)
    print(f"python {sys.version.split()[0]} unicodedata {ud.unidata_version}")
    mism = defaultdict(list)
    compared = 0
    with open(path, encoding="utf-8") as f:
        header = f.readline().rstrip("\n").split("\t")
        for line in f:
            row = dict(zip(header, line.rstrip("\n").split("\t")))
            age = row["age"]
            if age == "Unassigned" or vtuple(age) > pyver:
                continue
            cp = int(row["cp"], 16)
            c = chr(cp)
            compared += 1
            checks = {
                "name": (row["name"], ud.name(c, "")),
                "gc": (row["gc"], ud.category(c)),
                "ccc": (row["ccc"], str(ud.combining(c))),
                "bidi": (row["bidi"], ud.bidirectional(c)),
                "decomposition": (row["decomposition"], ud.decomposition(c)),
                "mirrored": (row["mirrored"] or "N", "Y" if ud.mirrored(c) else "N"),
                "eaw": (row["eaw"], ud.east_asian_width(c)),
            }
            pynum = ud.numeric(c, None)
            ours = float(Fraction(row["numeric_value"])) if row["numeric_value"] else None
            checks["numeric"] = (ours, pynum)
            for k, (a, b) in checks.items():
                if a != b:
                    mism[k].append((row["cp"], a, b))
    print(f"compared {compared} code points assigned by Unicode {ud.unidata_version}")
    if not mism:
        print("no mismatches")
    for k, v in sorted(mism.items()):
        print(f"\n{k}: {len(v)} mismatches (utf-mcp vs python)")
        for cp, a, b in v[:12]:
            print(f"  U+{cp}  {a!r}  vs  {b!r}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
