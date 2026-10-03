#!/bin/bash
# usage: ctx.sh FILE "fixed string" [N]
N=${3:-3}
echo "=== $1 :: $2"
grep -n -B$N -A$N -F -- "$2" "$1" | cut -c1-240 | head -$((4*N+6))
