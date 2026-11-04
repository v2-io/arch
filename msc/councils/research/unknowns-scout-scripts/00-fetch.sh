#!/bin/bash
# Fetch everything the Python steps read, into the current (work) directory.
# Needs: vsect on PATH; ASF_ROOT (default ~/src/arch/asf). Read-only on ASF.
# no set -e: vsect exits 1 when it has items to act on, which is not a failure
ASF_ROOT=${ASF_ROOT:-$HOME/src/arch/asf}; W=$(pwd)
mkdir -p reads vers tag
cd "$ASF_ROOT"
# 1. vsect read of every live segment (marks + header with current version)
for f in $(ls 0*/src/*.md | grep -v '/old-'); do s=$(basename $f .md); vsect read segment:$s > "$W/reads/$s.txt" 2>&1; done
# 2. the open ledger as JSON
vsect open --format json > "$W/open.json" 2>/dev/null || true
# 3. tag text of every live segment
for f in $(ls 0*/src/*.md | grep -v '/old-'); do b=$(basename $f .md); git show pre-integration-2026-10-30:$f > "$W/tag/$b.md" 2>/dev/null || rm -f "$W/tag/$b.md"; done
# 4. bare text of every version any check read, and of every checked segment now
python3 -I - "$ASF_ROOT" > "$W/pairs.txt" <<'PY'
import glob,yaml,sys
SEC=("rederive","read-check","run","instantiate","read","assert","decline")
ps=set()
for p in glob.glob(sys.argv[1]+"/.vsect/events/segment/*/*.yaml"):
    e=yaml.safe_load(open(p))
    if e["act"] in SEC: ps.add((e["about"],(e.get("target-version") or e["basis"][e["about"]])[:8]))
for a,v in sorted(ps): print(a,v)
PY
while read a v; do s=${a#segment:}; vsect read $a --at $v --bare > "$W/vers/$s@$v.txt" 2>/dev/null; done < "$W/pairs.txt"
for s in $(cut -d' ' -f1 "$W/pairs.txt" | sort -u); do s=${s#segment:}; vsect read segment:$s --bare > "$W/vers/$s@CUR.txt" 2>/dev/null; done
echo "fetched: $(ls $W/reads | wc -l) reads, $(ls $W/vers | wc -l) versions, $(ls $W/tag | wc -l) tag files"
