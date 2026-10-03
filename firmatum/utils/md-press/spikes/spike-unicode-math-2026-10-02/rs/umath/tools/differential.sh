#!/bin/sh
# Full differential: Rust `umath` vs the frozen Python reference, per version.
# Run from the spike root:   sh rs/umath/tools/differential.sh [v3 v4 v5]
# Needs data/bulk/ (regenerable; see the spike README) and data/gold/.
set -e
S=rs/umath/scratch
B=rs/umath/target/release/umath
VERS=${*:-v7}
mkdir -p $S
cargo build --release --manifest-path rs/umath/Cargo.toml 2>&1 | tail -1
[ -f $S/gold.jsonl ] || python3 rs/umath/tools/make_inputs.py
[ -f $S/fuzz.jsonl ] || python3 rs/umath/tools/fuzz_inputs.py 1 200000 > $S/fuzz.jsonl
[ -f $S/fuzz2.jsonl ] || python3 rs/umath/tools/fuzz_inputs.py 2 500000 > $S/fuzz2.jsonl
for v in $VERS; do
  for set in gold estate mathfree fuzz fuzz2; do
    $B --$v --spans --changed-only < $S/$set.jsonl > $S/$set.$v.rs.jsonl
    python3 rs/umath/tools/ref_jsonl.py py/frozen/umath_$v.py --spans --changed-only --procs 12 \
      < $S/$set.jsonl > $S/$set.$v.py.jsonl 2> $S/$set.$v.py.log
    printf '%s %-9s %s\n' $v $set "$(python3 rs/umath/tools/compare.py $S/$set.$v.rs.jsonl $S/$set.$v.py.jsonl $S/$set.jsonl | tee $S/$set.$v.cmp.txt | head -1)"
  done
done
